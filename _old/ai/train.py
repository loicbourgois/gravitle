import pandas as pd
import torch
from torch.utils.data import DataLoader
from torch.utils.data import Dataset
from torch.optim import SGD
from hyperopt import hp
import logging
import os
import time
import numpy as np
import ray
from ray import air, tune
from ray.air import Checkpoint, session
from ray.tune.schedulers import ASHAScheduler
# from ray.tune.search.hyperopt import HyperOptSearch
from ray.tune import ProgressReporter
from ray.tune.search.bayesopt import BayesOptSearch


logging.basicConfig(
    level=logging.INFO,
    format="%(message)s",
)


logging.info("start")
start_time = time.time()


# device = torch.device("mps")
device = torch.device("cpu") 
# device = torch.device("cuda:0") 

ai_dir = os.environ['HOME'] + "/github.com/loicbourgois/gravitle/ai/"



class CustomDataset(Dataset):
    def __init__(self, path):
        self.df = pd.read_csv(path)
        # self.decisions = ['turnright', 'turnleft', 'nothing', 'forward', 'slowdown']
    def __len__(self):
        return len(self.df.index)
    def __getitem__(self, idx):
        r = self.df.iloc[idx]
        # d = self.decisions.index(r.decision)
        y = torch.tensor([ r['l'], r['r'], r['f'] ], dtype=torch.float32)
        y = y.to(device)
        r = r.drop(['decision', 'l', 'r', 'f'])
        x = torch.tensor(r.values.tolist(), dtype=torch.float32)
        x = x.to(device)
        return x, y


class NeuralNetwork_1(torch.nn.Module):
    def __init__(self, config):
        self.l1 = int(config['l1'])
        self.activation = config['activation']
        activation_f = {
            'Tanh': torch.nn.Tanh,
            'ReLU': torch.nn.ReLU,
            'Hardtanh': torch.nn.Hardtanh,
        }[self.activation]
        super(NeuralNetwork_1, self).__init__()
        self.flatten = torch.nn.Flatten()
        self.stack = torch.nn.Sequential(
            torch.nn.Linear(12, self.l1),
            activation_f(),
            torch.nn.Linear(self.l1, self.l1),
            activation_f(),
            torch.nn.Linear(self.l1, self.l1),
            activation_f(),
            torch.nn.Linear(self.l1, 3),
            # torch.nn.Hardsigmoid(),
        )
    def forward(self, x):
        logits = self.stack(x)
        return logits
    def save_dict(self):
        return {
            "model_state_dict": self.state_dict(),
            "l1": self.l1,
            "activation": self.activation,
        }


class NeuralNetwork_3(torch.nn.Module):
    def __init__(self, l1, l2, l3, activation):
        self.l1 = int(l1)
        self.l2 = int(l2)
        self.l3 = int(l3)
        self.activation = activation
        activation_f = {
            'Tanh': torch.nn.Tanh,
            'ReLU': torch.nn.ReLU,
            'Hardtanh': torch.nn.Hardtanh,
        }[self.activation]
        super(NeuralNetwork_3, self).__init__()
        self.flatten = torch.nn.Flatten()
        self.stack = torch.nn.Sequential(
            torch.nn.Linear(12, self.l1),
            activation_f(),
            torch.nn.Linear(self.l1, self.l2),
            activation_f(),
            torch.nn.Linear(self.l2, self.l3),
            activation_f(),
            torch.nn.Linear(self.l3, 5),
        )
    def forward(self, x):
        logits = self.stack(x)
        return logits


def train_loop(dataloader, model, loss_fn, optimizer):
    for batch, (X, y) in enumerate(dataloader):
        pred = model(X)
        loss = loss_fn(pred, y)
        optimizer.zero_grad()
        loss.backward()
        optimizer.step()
        loss, _ = loss.item(), (batch + 1) * len(X)
    return loss


def test_loop(dataloader, model, loss_fn):
    size = len(dataloader.dataset)
    num_batches = len(dataloader)
    test_loss, correct, delta = 0, 0, 0
    with torch.no_grad():
        for X, y in dataloader:
            pred = model(X)
            test_loss += loss_fn(pred, y).item()
            # correct += (pred.argmax(1) == y).type(torch.float32).sum().item()
            delta += (y - pred).abs().sum().item()
    test_loss /= num_batches
    accuracy = correct / size
    # logging.info(f"Accuracy: {(100*accuracy):>0.1f}%, Avg loss: {test_loss:>8f}")
    return {
        'accuracy': accuracy,
        'delta': delta,
    }


class CustomReporter(ProgressReporter):
    def __init__(self, num_samples, i, c):
        super(CustomReporter, self).__init__()
        self.num_samples = num_samples
        self.start_time = time.time()
        self.i = i
        self.c = c
        self.last_time_should_report = self.start_time
    def should_report(self, trials, done=False):
        if int(time.time()) > int(self.last_time_should_report):
            self.last_time_should_report = time.time()
            return True
        else:
            return False
    def report(self, trials, *sys_info):
        data = [{
            'accuracy': x.last_result.get('accuracy'),
            'delta': x.last_result.get('delta'),
            'loss': x.last_result.get('loss'),
            'iter': x.last_result.get('training_iteration'),
            'status': x.status,
            **x.config,
            'done': x.last_result.get('done'),
        } for x in trials]
        df = pd.DataFrame(data)
        df = df.astype({'accuracy':float, 'iter':int}, errors="ignore").sort_values(
            by=['delta'],
            ascending=[True]
        )
        df = df.drop(columns=['checkpoint'], errors="ignore")
        progress = df[ df['status']  == 'TERMINATED' ].count()['status']
        print(*sys_info)
        print(df.head(10))
        print(f"{self.i+1}/{self.c} - {progress}/{self.num_samples} - {int(time.time()-start_time)}")
    def print_result(self):
        pass


# https://docs.ray.io/en/latest/tune/api/doc/ray.tune.Trainable.save_checkpoint.html#ray.tune.Trainable.save_checkpoint
# https://docs.ray.io/en/latest/tune/faq.html#how-can-i-continue-training-a-completed-tune-experiment-for-longer-and-with-new-configurations-iterative-experimentation 
class Trainable(tune.Trainable):
    def setup(self, config: dict):
        self.config = config
        if config.get('checkpoint'):
            self.load_checkpoint(config.get('checkpoint').to_dict())
        else:
            self.model = NeuralNetwork_1({
                'l1': config['l1'],
                # config['l2'],
                # config['l3'],
                'activation': sorted(
                    {
                        'Tanh': config['activation_Tanh'],
                        'ReLU': config['activation_ReLU'],
                        'Hardtanh': config['activation_Hardtanh'],
                    }.items(),
                    key=lambda x:x[1]
                )[-1][0],
            })
        self.optimizer = SGD(self.model.parameters(), lr=config['lr'])
        training_data = CustomDataset(f"{ai_dir}/train.csv")
        self.train_dataloader = DataLoader(
            training_data, 
            batch_size=int(config['batch_size'])
        )
        test_data = CustomDataset(f"{ai_dir}/test.csv")
        self.test_dataloader = DataLoader(
            test_data, 
            batch_size=int(config['batch_size'])
        )
        # self.loss_fn = torch.nn.CrossEntropyLoss()
        self.loss_fn = torch.nn.L1Loss()
        
        self.model.to(device)
        self.scheduler = torch.optim.lr_scheduler.ReduceLROnPlateau(
            self.optimizer, 'min', patience=config['patience'], factor=config['reduce_factor'],
        )

    def load_checkpoint(self, checkpoint):
        self.model = NeuralNetwork_1(checkpoint)
        self.model.load_state_dict(checkpoint['model_state_dict'])
    
    def save_checkpoint(self, checkpoint_dir: str):
        return self.model.save_dict()

    def step(self):
        loss = train_loop(self.train_dataloader, self.model, self.loss_fn, self.optimizer)
        self.scheduler.step(loss)
        r = test_loop(self.test_dataloader, self.model, self.loss_fn)
        self.save()
        return {
            "accuracy": r['accuracy'],
            "delta": r['delta'],
            "loss": loss,
        }

# https://docs.ray.io/en/latest/tune/api/doc/ray.tune.search.bayesopt.BayesOptSearch.html#ray-tune-search-bayesopt-bayesoptsearch

def run(steps):
    _context = ray.init()
    best_checkpoint = None
    results = []
    for i, step in enumerate(steps):
        if i == 0:
            tuner = tune.Tuner(
                Trainable,
                tune_config=tune.TuneConfig(
                    num_samples=steps[i]['num_samples'],
                    scheduler=ASHAScheduler(
                        metric=step['opti']['metric'],
                        mode=step['opti']['mode'],
                        max_t=steps[i]['max_t'],
                    ),
                    search_alg=BayesOptSearch({
                        "lr": (0.000001, 10.0),
                        "patience": (0.01, 10.0),
                        "reduce_factor": ( 0.0001, 0.9),
                        "batch_size": (4, 64),
                        "l1": ( 2, 64),
                        "activation_Tanh": ( 0.0, 1.0),
                        "activation_ReLU": ( 0.0, 1.0),
                        "activation_Hardtanh": ( 0.0, 1.0),
                    }, metric="loss", mode="min"),
                ),
                run_config=air.RunConfig(
                    progress_reporter=CustomReporter(steps[i]['num_samples'], i, len(steps)),
                    verbose=1,
                ),
                # param_space={
                #     "batch_size": 64,
                # },
            )
        else:
            tuner = tune.Tuner(
                Trainable,
                tune_config=tune.TuneConfig(
                    num_samples=steps[i]['num_samples'],
                    scheduler=ASHAScheduler(
                        metric=step['opti']['metric'], 
                        mode=step['opti']['mode'],
                        max_t=steps[i]['max_t'],
                    ),
                    search_alg=BayesOptSearch({
                        "lr": (0.000001, 0.1),
                        "patience": (0.01, 10.0),
                        "reduce_factor": ( 0.0001, 0.9),
                        "batch_size": (4, 64),
                    }, metric=step['opti']['metric'], mode=step['opti']['mode']),
                ),
                run_config=air.RunConfig(
                    progress_reporter=CustomReporter(steps[i]['num_samples'], i, len(steps)),
                    verbose=1,
                ),
                param_space={
                    "checkpoint": best_checkpoint,
                    # "l1": best_checkpoint.to_dict()['l1'],
                    # "l2": best_checkpoint.to_dict()['l2'],
                    # "l3": best_checkpoint.to_dict()['l3'],
                },
            )
        result_grid = tuner.fit()
        best_result = result_grid.get_best_result(metric="delta", mode="min")
        best_checkpoint = best_result.checkpoint
        results.append(result_grid)
    for i, result_grid in enumerate(results):
        df = result_grid.get_dataframe()
        df = df.sort_values(
            by=['delta'],
            ascending=[True],
        )
        def transform(r):
            r['activation'] = sorted(
                {
                    'Tanh': r['config/activation_Tanh'],
                    'ReLU': r['config/activation_ReLU'],
                    'Hardtanh': r['config/activation_Hardtanh'],
                }.items(),
                key=lambda x:x[1]
            )[-1][0]
            return r
        try:
            df = df.apply(transform, axis=1)
        except:
            pass
        if i == 0:
            logging.info(df[['accuracy', 'delta', 'loss', 'config/lr', 'config/patience', 'config/reduce_factor', 'config/lr', 'config/l1', 'activation']].head(10))
        else:
            logging.info(df[['accuracy', 'delta', 'loss', 'config/lr', 'config/patience', 'config/reduce_factor', 'config/lr']].head(10))
    logging.info(f"duration: {time.time()-start_time}")


# run([
#     {
#         'max_t': 4,
#         'num_samples': 64,
#         'opti': {
#             'metric': "accuracy",
#             'mode': "max",
#         },
#     },
# ])

model = NeuralNetwork_1({
    'l1': 10,
    'activation': 'ReLU'
})
training_data = CustomDataset(f"{ai_dir}/train.csv")
train_dataloader = DataLoader(
    training_data, 
    batch_size=4,
)
test_data = CustomDataset(f"{ai_dir}/test.csv")
test_dataloader = DataLoader(
    test_data, 
    batch_size=4
)
loss_fn = torch.nn.CrossEntropyLoss()
optimizer = SGD(model.parameters(), lr=0.1)
train_loop(train_dataloader, model, loss_fn, optimizer)
test_loop(test_dataloader, model, loss_fn)


# 47.958
# 39.
# 35.002718
run([
    {
        'max_t': 16,
        'num_samples': 256,
        'opti': {
            'metric': "delta",
            'mode': "min",
        },
    },
    {
        'max_t': 64,
        'num_samples': 128,
        'opti': {
            'metric': "delta",
            'mode': "min",
        },
    },
    {
        'max_t': 64,
        'num_samples': 128,
        'opti': {
            'metric': "delta",
            'mode': "min",
        },
    },
    {
        'max_t': 64,
        'num_samples': 128,
        'opti': {
            'metric': "delta",
            'mode': "min",
        },
    },
    {
        'max_t': 64,
        'num_samples': 128,
        'opti': {
            'metric': "delta",
            'mode': "min",
        },
    },
    {
        'max_t': 64,
        'num_samples': 128,
        'opti': {
            'metric': "delta",
            'mode': "min",
        },
    },
    {
        'max_t': 64,
        'num_samples': 128,
        'opti': {
            'metric': "delta",
            'mode': "min",
        },
    },
    {
        'max_t': 64,
        'num_samples': 128,
        'opti': {
            'metric': "delta",
            'mode': "min",
        },
    }
    # {
    #     'max_t': 32,
    #     'num_samples': 64,
    #     'opti': {
    #         'metric': "loss",
    #         'mode': "min",
    #     },
    # },{
    #     'max_t': 128,
    #     'num_samples': 64,
    #     'opti': {
    #         'metric': "accuracy",
    #         'mode': "max",
    #     },
    # },{
    #     'max_t': 256,
    #     'num_samples': 64,
    #     'opti': {
    #         'metric': "loss",
    #         'mode': "min",
    #     },
    # },{
    #     'max_t': 256,
    #     'num_samples': 64,
    #     'opti': {
    #         'metric': "accuracy",
    #         'mode': "max",
    #     },
    # }
])
