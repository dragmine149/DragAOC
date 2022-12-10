import argparse
import time

st = time.time()


def setup():
    parser = argparse.ArgumentParser(description="AOC helper",
                                     add_help=True,
                                     allow_abbrev=True)
    parser.add_argument("-o", "--Other", type=int, choices=[
                        1, 2], default=1, help="TO read the true data or to read the example data")
    parser.add_argument("-v", "--Visulasation", type=bool, choices=[True, False], default=False, nargs="?", help="To show the directory listing or not to show")
    args = parser.parse_args()
    return args.Other, args.Visulasation


def read(mode=1):
    path = "data.txt" if mode == 1 else "data.txt2"
    with open(path, "r", encoding="utf-8") as f:
        return f.readlines()


def main():
    O, V = setup()
    return read(O), V


def output(day: int):
    print(f"Day {day} elf results:")
    print(f"Total time: {time.time() - st}")
