import argparse
import time
import os

st = time.time()


class helper:
    def __init__(self, day) -> None:
        self.day = day

    def time(self):
        return time.time() - st

    def __setup(self):
        parser = argparse.ArgumentParser(description="AOC helper",
                                         add_help=True,
                                         allow_abbrev=True)
        parser.add_argument("-o", "--Other", type=int, choices=[
            1, 2], default=1, help="TO read the true data or to read the example data")
        parser.add_argument('-r', '--Read', action='store_const', default=True, const=not (True),
                            help="Use this to read the whole file instead of line by line")
        args = parser.parse_args()
        return args.Other, args.Read

    def __read(self, mode=1, rl=True):
        path = "data.txt" if mode == 1 else "data.txt2"
        with open(path, "r", encoding="utf-8") as f:
            if rl:
                return f.readlines()
            return f.read()

    def main(self):
        # supports multiple command line args
        other, Read = self.__setup()
        return self.__read(other, Read)

    def output(self):
        print(f"Day {self.day} elf results:")
        print(f"Total time: {time.time() - st}")

    def print(self, text):
        print('-' * os.get_terminal_size().columns)
        print(text)
        print('-' * os.get_terminal_size().columns)
    
    def Remove(self, data, *, items=None):
        """Remove all of item in data

        Args:
            data (str): The data to change

        Returns:
            str: The resulting data
        """
        if items is None:
            raise AOCException("Okay, who forgot to pass arguments?")
        
        for i in items:
            data = data.replace(i, "")
        return data


class AOCException(Exception):
    msg = "Custom advent of code exception error. Something went wrong whilst trying to do stuff"

    def __init__(self, msg, *args: object) -> None:
        super().__init__(*args)
        self.msg = msg

    def __str__(self):
        return self.msg


class math():
    max = 2_147_483_647
    max64 = 9_233_372_036_854_775_808
