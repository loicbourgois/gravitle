import torch
from torch.utils.data import DataLoader
from torch.utils.data import Dataset
import pandas as pd
import time
from torch.optim import SGD
import ray
import math
import json
from random import random
import os
from ray import air, tune
from ray.air import Checkpoint, session
from ray.tune.schedulers import ASHAScheduler
from ray.tune import ProgressReporter
from prettytable import PrettyTable
from ray.tune.search.bayesopt import BayesOptSearch
import logging
import uuid


logging.basicConfig(
    level=logging.INFO,
    format="%(message)s",
)
logging.info("start")
start_time = time.time()


class Counter():
    def __init__(self):
        self.c = 0
    def tick(self):
        c = self.c
        self.c += 1
        return c


class Model(torch.nn.Module):
    def __init__(self, model_conf):
        super(Model, self).__init__()
        self.l1 = int(model_conf['l1'])
        self.activation = model_conf['activation']
        activation_f = {
            'Tanh': torch.nn.Tanh,
            'ReLU': torch.nn.ReLU,
            'Hardtanh': torch.nn.Hardtanh,
        }[self.activation]
        self.cidx = model_conf.get('cidx')
        self.lx = int(model_conf['lx'])
        self.ly = int(model_conf['ly'])
        self.pmid = model_conf.get('pmid')
        self.mid = str(uuid.uuid4())
        self.stack = torch.nn.Sequential(
            torch.nn.Linear(self.lx, self.l1),
            activation_f(),
            torch.nn.Linear(self.l1, self.l1),
            activation_f(),
            torch.nn.Linear(self.l1, self.l1),
            activation_f(),
            torch.nn.Linear(self.l1, self.ly),
            # torch.nn.Hardsigmoid(),
        )
    def forward(self, x):
        return self.stack(x)
    def save_dict(self):
        return {
            "l1": self.l1,
            "activation": self.activation,
            "lx": self.lx,
            "ly": self.ly,
            "mid": self.mid,
            "pmid": self.pmid,
            "cidx": self.cidx,
        }


def train_epoch(dataloader, model, loss_fn, optimizer):
    full_loss = 0
    for batch, (X, y) in enumerate(dataloader):
        pred = model(X)
        loss = loss_fn(pred, y)
        optimizer.zero_grad()
        loss.backward()
        optimizer.step()
        full_loss += loss.item()
    return full_loss


def test_epoch(dataloader, model, loss_fn):
    loss, delta = 0, 0
    with torch.no_grad():
        for X, y in dataloader:
            pred = model(X)
            loss += loss_fn(pred, y).item()
            delta += (y - pred).abs().sum().item()
    return {
        'delta': delta,
        'loss': loss,
    }


class CustomDataset(Dataset):
    def __init__(self, path):
        df = pd.read_csv(path)
        x_columns = [ x for x in df.columns.tolist() if x.startswith('x_') ]
        y_columns = [ y for y in df.columns.tolist() if y.startswith('y_') ]
        def transform(r):
            r['x'] = [
                r[k] for k in x_columns
            ]
            r['y'] = [
                r[k] for k in y_columns
            ]
            return r
        df = df.apply(transform, axis=1)
        df = df[ ['x', 'y'] ]
        self.lx = len(x_columns)
        self.ly = len(y_columns)
        self.df = df
    def __len__(self):
        return len(self.df.index)
    def __getitem__(self, idx):
        r = self.df.iloc[idx]
        return torch.tensor(r['x'], dtype=torch.float32), torch.tensor(r['y'], dtype=torch.float32)


class CustomReporter(ProgressReporter):
    def __init__(self, num_samples, i, c, config):
        super(CustomReporter, self).__init__()
        self.num_samples = num_samples
        self.start_time = time.time()
        self.i = i
        self.c = c
        self.ascending = min_max_to_asc(config['opti']['mode'])
        self.sort_metric = config['opti']['metric']
        self.last_time_should_report = self.start_time
    def should_report(self, trials, done=False):
        if int(time.time()) > int(self.last_time_should_report):
            self.last_time_should_report = time.time()
            return True
        else:
            return False
    def report(self, trials, *sys_info):
        data = [{
            'pmid': x.last_result.get('pmid'),
            'mid': x.last_result.get('mid'),
            'dpps': x.last_result.get('dpps'),
            'delta': x.last_result.get('delta'),
            'loss': x.last_result.get('loss'),
            'activation': x.last_result.get('activation'),
            'l1': x.last_result.get('l1'),
            'iter': x.last_result.get('training_iteration'),
            'status': x.status,
            'done': x.last_result.get('done'),
            'cidx': x.last_result.get('cidx'),
        } for x in trials]
        df = pd.DataFrame(data)
        df = df.astype(
            {self.sort_metric: float, 'iter':int}, errors="ignore"
        ).sort_values(
            by=self.sort_metric,
            ascending=[self.ascending]
        )
        df = df.drop(columns=['checkpoint', 'data_dir'], errors="ignore")
        progress = df[ df['status']  == 'TERMINATED' ].count()['status']
        print(*sys_info)
        print(df.head(10))
        print(f"{self.i+1}/{self.c} - {progress}/{self.num_samples} - {int(time.time()-self.start_time)}")


def count_parameters(model):
    # table = PrettyTable(["Modules", "Parameters"])
    total_params = 0
    for name, parameter in model.named_parameters():
        if not parameter.requires_grad: continue
        params = parameter.numel()
        # table.add_row([name, params])
        total_params+=params
    # print(table)
    # print(f"Total Trainable Params: {total_params}")
    return total_params


def write_force(content, path):
    folder = path.replace(path.split("/")[-1], '')
    if not os.path.exists(folder):
        os.makedirs(folder)
    with open(path, 'w') as f:
        f.write(content)


class Trainable(tune.Trainable):
    def setup(self, config: dict):
        logging.info(config)
        logging.info(self.config)
        self.config = config
        if len(config.get('checkpoints')):
            l = len( config['checkpoints'] )
            cidx = min( l-1, int(config['cidx']) )
            checkpoint = config['checkpoints'][ cidx ]
            checkpoint['pmid'] = checkpoint['model'].mid
            checkpoint['cidx'] = config['cidx']
            self.load_checkpoint(checkpoint)
        else:
            self.model = Model({
                'l1': config['l1'],
                'lx': config['lx'],
                'ly': config['ly'],
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
        training_data = CustomDataset(f"{config['data_dir']}/train.csv")
        self.train_dataloader = DataLoader(
            training_data, 
            batch_size=int(config['batch_size'])
        )
        test_data = CustomDataset(f"{config['data_dir']}/test.csv")
        self.test_dataloader = DataLoader(
            test_data, 
            batch_size=int(config['batch_size'])
        )
        self.loss_fn = torch.nn.L1Loss()
        self.scheduler = torch.optim.lr_scheduler.ReduceLROnPlateau(
            self.optimizer, 'min', patience=config['patience'], factor=config['reduce_factor'],
        )

    def load_checkpoint(self, checkpoint):
        model_conf = checkpoint['model'].save_dict()
        model_conf['pmid'] = model_conf['mid']
        model_conf['cidx'] = checkpoint['cidx']
        self.model = Model(model_conf)
        self.model.load_state_dict(checkpoint['model'].state_dict())

    def save_checkpoint(self, checkpoint_dir: str):
        return self.model.save_dict()

    def step(self):
        loss = train_epoch(self.train_dataloader, self.model, self.loss_fn, self.optimizer)
        self.scheduler.step(loss)
        r = test_epoch(self.test_dataloader, self.model, self.loss_fn)
        self.save()
        return {
            "trial_id": self.trial_id,
            "cidx": self.model.cidx,
            "pmid": self.model.pmid,
            "mid": self.model.mid,
            "delta": r['delta'],
            "activation": self.model.activation,
            "l1": self.model.l1,
            "model": self.model,
            "lr": self.config['lr'],
            "lx": self.model.lx,
            "ly": self.model.ly,
            "patience": self.config['patience'],
            "reduce_factor": self.config['reduce_factor'],
            "loss": loss,
            "dpps": r['delta'] * math.sqrt(count_parameters(self.model)) # delta per parameters.sqrt
        }


def min_max_to_asc(mode):
    return {
        'min': True,
        'max': False,
    }[mode]


def print_step_results(results, steps):
    for i, result_grid in enumerate(results):
        step_config = steps[i]
        df = result_grid.get_dataframe()
        df = df.sort_values(
            by=[step_config['opti']['metric']],
            ascending=[min_max_to_asc(step_config['opti']['mode'])],
        )
        logging.info(df[['mid', 'pmid', 'dpps', 'delta', 'loss', 'l1', 'activation', 'cidx']].head(10))


def run(steps, training_data):
    results = []
    checkpoints = []
    for i, step in enumerate(steps):
        if len(checkpoints):
            model_ = Model(checkpoints[0]['model'].save_dict())
            model_.load_state_dict(checkpoints[0]['model'].state_dict())
        counter = Counter()
        tuner = tune.Tuner(
            Trainable,
            tune_config=tune.TuneConfig(
                num_samples=step['num_samples'],
                scheduler=ASHAScheduler(
                    metric=step['opti']['metric'],
                    mode=step['opti']['mode'],
                    max_t=step['max_t'],
                ),
                search_alg=BayesOptSearch({
                    "lr": (0.000001, 0.1),
                    "patience": (0.01, 1.0),
                    "reduce_factor": ( 0.0001, 0.9),
                    "l1": ( 2, 64),
                    "activation_Tanh": ( 0.0, 1.0),
                    "activation_ReLU": ( 0.0, 1.0),
                    "activation_Hardtanh": ( 0.0, 1.0),
                    "cidx": (0.0, len(checkpoints)),
                }, metric="loss", mode="min"),
            ),
            run_config=air.RunConfig(
                progress_reporter=CustomReporter(step['num_samples'], i, len(steps), step),
                verbose=1,
            ),
            param_space={
                "lx": training_data.lx,
                "ly": training_data.ly,
                "checkpoints": checkpoints,
                "data_dir": "/Users/loicbourgois/github.com/loicbourgois/gravitle/ai/poc_1",
                'batch_size': 4,
                'counter': counter,
                "step_i": i,
            },
        )
        result_grid = tuner.fit()
        data = [
            {
                'model': r.metrics['model'],
                step['opti']['metric']: r.metrics[ step['opti']['metric'] ],
            } for r in result_grid._results
        ]
        df = pd.DataFrame(data)
        df = df.sort_values(
            by=[step['opti']['metric']],
            ascending=[min_max_to_asc(step['opti']['mode'])],
        )
        df = df.head(step['checkpoint_threshold'])
        checkpoints = df.to_dict('records')
        results.append(result_grid)
    print_step_results(results, steps)
    logging.info(f"duration: {time.time()-start_time}")


glob = {
    'counter': Counter(),
}


def train_loop(training_data, iterations):
    train_dataloader = DataLoader(
        training_data, 
        batch_size=4,
    )
    lr = 0.096991
    patience = 0.834118
    reduce_factor = 0.191184
    model = Model({
        'l1': 3,
        'activation': 'Hardtanh',
        'lx': training_data.lx,
        'ly': training_data.ly,
    })
    loss_fn = torch.nn.L1Loss()
    optimizer = SGD(model.parameters(), lr=0.001)
    for i in range(200):
        loss = train_epoch(train_dataloader, model, loss_fn, optimizer)
        print(loss)


def ray_train(training_data):
    ray.init()
    run([
        {
            'max_t': 16,
            'num_samples': 64,
            'checkpoint_threshold': 4,
            'opti': {
                'metric': "dpps",
                'mode': "min",
            },
        },
        {
            'max_t': 16,
            'num_samples': 64,
            'checkpoint_threshold': 4,
            'opti': {
                'metric': "delta",
                'mode': "min",
            },
        },
        {
            'max_t': 16,
            'num_samples': 64,
            'checkpoint_threshold': 4,
            'opti': {
                'metric': "delta",
                'mode': "min",
            },
        },
    ], training_data)


def ray_train_test(training_data):
    ray.init()
    run([
        {
            'max_t': 1,
            'num_samples': 8,
            'checkpoint_threshold': 4,
            'opti': {
                'metric': "dpps",
                'mode': "min",
            },
        },
        {
            'max_t': 1,
            'num_samples': 8,
            'checkpoint_threshold': 4,
            'opti': {
                'metric': "delta",
                'mode': "min",
            },
        },
        {
            'max_t': 1,
            'num_samples': 8,
            'checkpoint_threshold': 4,
            'opti': {
                'metric': "delta",
                'mode': "min",
            },
        },
    ], training_data)


training_data = CustomDataset("ai/poc_1/train.csv")
# train_loop(training_data, 100)
# ray_train_test(training_data)
ray_train(training_data)

# delta = 0.035635