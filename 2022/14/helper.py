import argparse
import time
import os

st = time.time()


class Grid:
    def __init__(self) -> None:
        """Generate a 2d grid

        Default Value: .
        """
        self.grid = []
        self.defaultValue = "."

    def changeDefaultValue(self, value: str = '.'):
        """Change the default value to put on the grid

        Args:
            value (str, optional): The value to change to. Defaults to '/'.
        """
        self.defaultValue = value

    def Generate(self, x: int, y: int):
        """Generate a grid of XY

        Args:
            x (int): The x length
            y (int): The y length
        """
        for _ in range(y):
            tmp = []
            for _ in range(x):
                tmp.append(self.defaultValue)
            self.grid.append(tmp)

    def GenerateTo(self, x: int, y: int):
        """Generate missing values

        Args:
            x (int): X pos
            y (int): Y pos
        """
        print(f"Generating to: ({x},{y})")

        if len(self.grid) == 0:
            self.Generate(x, y)
            return

        if len(self.grid[0]) == 0:
            for _ in range(x):
                for index, _ in enumerate(self.grid):
                    self.grid[index].apped(self.defaultValue)
            return

        for _ in range(y - len(self.grid)):
            tmp = []
            for _ in range(len(self.grid)):
                tmp.append(self.defaultValue)
            self.grid.append(tmp)

        self.MakeEqual()

    def MakeEqual(self):
        """Make every single row the same value from the biggest row.
        """
        bigRow = 0
        for _ in self.grid:
            if len(self.grid) > bigRow:
                bigRow = len(self.grid)

        for yV in self.grid:
            yV: list
            if len(yV) < bigRow:
                for _ in range(bigRow - len(yV)):
                    yV.append(self.defaultValue)
                continue
            
            if len(yV) == bigRow:
                continue
            
            raise AOCException("Okay, so how does it become bigger?")

    def GenerateAmount(self, x: int, y: int):
        """Generate X, Y extra

        Args:
            x (int): X value to generate extra
            y (int): Y value to generate extra
        """
        for r in self.grid:
            for _ in range(x):
                r.append(self.defaultValue)

        tmp = []
        for _ in range(self.grid[0]):
            tmp.append(self.defaultValue)

        for _ in self.grid:
            for _ in range(y):
                self.grid.append(tmp)


class Visual:
    @staticmethod
    def print(gdata):
        for yI, yV in enumerate(gdata):
            for xI, xV in enumerate(yV):
                print(f'{xV}', end='')
            print()


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
    """Custom AOC exception
    """
    
    def __init__(self, *args: object) -> None:
        super().__init__(*args)


class math():
    max = 2_147_483_647
    max64 = 9_233_372_036_854_775_808
