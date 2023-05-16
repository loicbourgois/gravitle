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
from ray.tune.search.hyperopt import HyperOptSearch


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
        self.decisions = ['turnright', 'turnleft', 'nothing', 'forward', 'slowdown']


    def __len__(self):
        return len(self.df.index)

    def __getitem__(self, idx):
        r = self.df.iloc[idx]
        d = self.decisions.index(r.decision)
        # except:
        #     d = len(self.decisions)
        r = r.drop('decision')
        return torch.tensor(r.values.tolist(), dtype=torch.float32).to(device), torch.tensor(d).to(device)


class NeuralNetwork_3(torch.nn.Module):
    def __init__(self, l1, l2, l3):
        super(NeuralNetwork_3, self).__init__()
        self.flatten = torch.nn.Flatten()
        self.linear_relu_stack = torch.nn.Sequential(
            torch.nn.Linear(12, l1),
            torch.nn.ReLU(),
            torch.nn.Linear(l1, l2),
            torch.nn.ReLU(),
            torch.nn.Linear(l2, l3),
            torch.nn.ReLU(),
            torch.nn.Linear(l3, 5),
        )
    def forward(self, x):
        logits = self.linear_relu_stack(x)
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
    test_loss, correct = 0, 0
    with torch.no_grad():
        for X, y in dataloader:
            pred = model(X)
            test_loss += loss_fn(pred, y).item()
            correct += (pred.argmax(1) == y).type(torch.float32).sum().item()
    test_loss /= num_batches
    correct /= size
    logging.info(f"Accuracy: {(100*correct):>0.1f}%, Avg loss: {test_loss:>8f}")
    return correct

# https://docs.ray.io/en/latest/tune/api/doc/ray.tune.Trainable.save_checkpoint.html#ray.tune.Trainable.save_checkpoint
class Trainable(tune.Trainable):
    def setup(self, config: dict):
        self.config = config
        batch_size = 100
        # epochs = config['epochs']
        if config.get("start_from_checkpoint"):
            checkpoint_ref = config["start_from_checkpoint"]
            checkpoint: Checkpoint = ray.get(checkpoint_ref)
            cd = checkpoint.to_dict()
            self.model = NeuralNetwork_3(
                cd['l1'],
                cd['l2'],
                cd['l3'],
            )
            model_state_dict = cd["model_state_dict"]
            self.model.load_state_dict(model_state_dict)
        else:
            self.model = NeuralNetwork_3(
                config['l1'],
                config['l2'],
                config['l3'],
            )
        self.optimizer = SGD(self.model.parameters(), lr=config['lr'])
        training_data = CustomDataset(f"{ai_dir}/train.csv")
        self.train_dataloader = DataLoader(training_data, batch_size=batch_size)
        test_data = CustomDataset(f"{ai_dir}/test.csv")
        self.test_dataloader = DataLoader(test_data, batch_size=batch_size)
        self.loss_fn = torch.nn.CrossEntropyLoss()
        self.model.to(device)
        self.scheduler = torch.optim.lr_scheduler.ReduceLROnPlateau(
            self.optimizer, 'min', patience=config['patience'], factor=config['reduce_factor'],
        )

    def load_checkpoint(self, checkpoint):
        pass
    
    def save_checkpoint(self, checkpoint_dir: str):
        pass

    def step(self):
        loss = train_loop(self.train_dataloader, self.model, self.loss_fn, self.optimizer)
        self.scheduler.step(loss)
        accuracy = test_loop(self.test_dataloader, self.model, self.loss_fn)
        self.save_checkpoint(Checkpoint.from_dict({
            "model_state_dict": self.model.state_dict(),
            "l1": self.config['l1'],
            "l2": self.config['l2'],
            "l3": self.config['l3'],
        }))
        return {"accuracy": accuracy}
    

    def log_result(self, result):
        pass


from ray.tune import ProgressReporter
class CustomReporter(ProgressReporter):
    def __init__(self, num_samples):
        super(CustomReporter, self).__init__()
        self.num_samples = num_samples
        self.start_time = time.time()
        self.last_time_should_report = self.start_time
    def should_report(self, trials, done=False):
        if int(time.time()) > int(self.last_time_should_report):
            self.last_time_should_report = time.time()
            return True
        else:
            return False
    def report(self, trials, *sys_info):
        # print(dir(trials[0]))
        # print(trials[0].config)
        # print(trials[0].status)
        # print(trials[0].results)
        # print(trials[0].last_result)
        # print(trials[0].l)
        # acc = "-"
        # try:
        #     acc = x.last_result['accuracy']
        # except:
        #     pass
        data = [{
            'acc': x.last_result.get('accuracy'),
            'iter': x.last_result.get('training_iteration'),
            'status': x.status,
            **x.config,
            'done': x.last_result.get('done'),
        } for x in trials]
        df = pd.DataFrame(data)
        # df = df.fillna('')
        df = df.astype({'acc':float, 'iter':int}, errors="ignore").sort_values(
            by=['acc'],
            ascending=[False]
        )
        progress = df[ df['status']  == 'TERMINATED' ].count()['status']
        print(*sys_info)
        print(df.head(10))
        print(f"{progress} / {self.num_samples} - {int(time.time()-start_time)}")
    def print_result(self):
        pass


def run(epochs=None, num_samples=None):
    _context = ray.init()
    config = {
        "lr": tune.uniform(0.01, 10),
        "patience": tune.uniform(1, 10),
        "reduce_factor": tune.uniform(0.1, 1),
        "l1": tune.randint(10, 30),
        "l2": tune.randint(10, 30),
        "l3": tune.randint(10, 30),
        'epochs': epochs,
    }
    tuner = tune.Tuner(
        Trainable,
        tune_config=tune.TuneConfig(
            num_samples=num_samples,
            scheduler=ASHAScheduler(
                metric="accuracy", 
                mode="max",
                max_t=config['epochs'],
            ),
        ),
        run_config=air.RunConfig(
            progress_reporter=CustomReporter(num_samples),
            verbose=1,
        ),
        param_space=config,
    )
    result_grid = tuner.fit()
    df_1 = result_grid.get_dataframe()
    df_1 = df_1.sort_values(
        by=['accuracy'],
        ascending=[False]
    )
    logging.info(df_1[['accuracy', 'config/lr', 'config/patience', 'config/reduce_factor', 'config/lr', 'config/l1', 'config/l2', 'config/l3']].head(10))
    logging.info(f"duration: {time.time()-start_time}")

# log_result

# best_result = result_grid.get_best_result(metric="accuracy", mode="max")
# best_checkpoint = best_result.checkpoint
# config['start_from_checkpoint'] = ray.put(best_checkpoint)
# config['epochs'] = 1000
# new_tuner = tune.Tuner(
#     train,
#     param_space=config,
#     tune_config=tune.TuneConfig(
#         num_samples=1000,
#         scheduler=ASHAScheduler(
#             metric="accuracy", 
#             mode="max",
#             max_t=config['epochs'],
#         ),
#     ),
# )
# result_grid_2 = new_tuner.fit()


# search_space = {
#     "lr": tune.uniform(0.1, 1),
#     "patience": tune.uniform(1, 10),
#     'epochs': 100,
# }
# tuner = tune.Tuner(
#     train,
#     tune_config=tune.TuneConfig(
#         num_samples=100,
#         # time_budget_s=10,
#         scheduler=ASHAScheduler(metric="accuracy", mode="max"),
#     ),
#     param_space=search_space,
# )
# result_grid_3 = tuner.fit()



# df_2 = result_grid_2.get_dataframe()
# df_2 = df_2.sort_values(
#     by=['accuracy'],
#     ascending=[False]
# )
# df_3 = result_grid_3.get_dataframe()
# df_3 = df_3.sort_values(
#     by=['accuracy'],
#     ascending=[False]
# )
# logging.info(df_2[['accuracy', 'config/lr', 'config/patience', 'config/reduce_factor', 'config/lr', 'config/l1', 'config/l2', 'config/l3']].head(10))
# logging.info(df_3.head())

# logging.info(df.info())


run(epochs=100, num_samples=1000)
