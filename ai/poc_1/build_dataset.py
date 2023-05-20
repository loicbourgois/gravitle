# PYTHONPATH=$HOME/github.com/loicbourgois/gravitle python3 -m ai.poc_1.build_dataset
import os
from random import random
import pandas
from math import sin
def gen_test_case():
    x_0 = random()
    x_1 = random()
    y_0 = x_0
    y_1 = max(0.0, sin(x_0) + sin(2*x_1) )
    return {
        'x_0':x_0,
        'x_1':x_1,
        'y_0':y_0,
    }
count = 256
df = pandas.DataFrame(
    [ gen_test_case() for _ in range(count) ]
)
df.to_csv("ai/poc_1/train.csv", index=False)
(pandas.DataFrame(
    [ gen_test_case() for _ in range(int(count)) ]
)).to_csv("ai/poc_1/test.csv", index=False)
