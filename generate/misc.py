
import os


def read(path):
    with open(path, "r") as file:
        return file.read()


def write_force(path, content):
    folder = path.replace(path.split("/")[-1], "")
    if not os.path.exists(folder):
        os.makedirs(folder)
    with open(path, "w") as f:
        f.write(content)