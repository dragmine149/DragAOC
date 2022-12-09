import argparse
import time

st = time.time()


def setup():
    parser = argparse.ArgumentParser(description="AOC helper",
                                     add_help=True,
                                     allow_abbrev=True)
    parser.add_argument("-o", "--Other", type=int, choices=[
        1, 2], default=1, help="TO read the true data or to read the example data")
    args = parser.parse_args()
    return args.Other


def read(mode=1):
    path = "data.txt" if mode == 1 else "data.txt2"
    with open(path, "r", encoding="utf-8") as f:
        return f.readlines()


def main():
    # supports multiple command line args
    other = setup()
    return read(other)


def output(day: int):
    print(f"Day {day} elf results:")
    print(f"Total time: {time.time() - st}")
