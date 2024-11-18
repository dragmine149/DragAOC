import argparse
import copy
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

        if len(self.grid) == y:
            for yV in self.grid:
                yV: list
                for _ in range(x - len(yV)):
                    yV.append(self.defaultValue)

        if len(self.grid[0]) == x:
            x = []
            for _ in range(len(self.grid[0])):
                x.append(self.defaultValue)

            for _ in range(y - len(self.grid)):
                self.grid.append(copy.copy(x))

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
    def print(data):
        """Print a grid of data with no formatting

        Args:
            data (list): The data to print
        """
        for y in data:
            for x in y:
                print(x, end='')
            print()

    @staticmethod
    def printColour(data, info: dict):
        """Print data but with special colour formatting 
        if it matches a certain symbol

        E.g. '-' = orange

        Args:
            data (list): The data to print
            info (dict): The colour to format that data with
        """
        for y in data:
            for x in y:
                print(f'{info.get(x)}{x}\033[0m', end='')
            print()

    @staticmethod
    def printPosition(data, posInfo):
        """Print the data but at X,Y print a colour

        Args:
            data (list): The data to print
            posInfo (dict): The colour and position info
        """
        for yI, yV in enumerate(data):
            for xI, xV in enumerate(yV):
                for v in posInfo:
                    if v.get("X") == xI and v.get("Y") == yI:
                        print(f'{v.get("c")}{xV}\033[0m', end='')
                        break
                print()

    @staticmethod
    def printSurround(data):
        """Surroundings data by -

        Args:
            data (any): The data to surround
        """
        print('-' * os.get_terminal_size().columns)
        print(data)
        print('-' * os.get_terminal_size().columns)


class helper:
    def __init__(self, day) -> None:
        """The helper class, some files to do extra stuff

        Args:
            day (int): The day to use in the helper class
        """
        self.day = day
        self.__setup()
        self.args = None
        self.unknown = None

    def time(self):
        """Return the time since the start of the program

        Returns:
            float: Time since start
        """
        return time.time() - st

    def __setup(self):
        self.parser = argparse.ArgumentParser(description="AOC helper",
                                              add_help=True,
                                              allow_abbrev=True)
        self.parser.add_argument("-o", "--Other", type=int, choices=[
            1, 2], default=1, help="TO read the true data or to read the example data")
        self.parser.add_argument('-r', '--Read', action='store_const', default=True, const=not (True),
                                 help="Use this to read the whole file instead of line by line")

    def __GetParser(self):
        self.args, self.unknown = self.parser.parse_known_args()
        return self.args.Other, self.args.Read

    def reUnknown(self):
        return self.unknown

    def __read(self, mode=1, rl=True):
        path = "data.txt" if mode == 1 else "data.txt2"
        with open(path, "r", encoding="utf-8") as f:
            if rl:
                return f.readlines()
            return f.read()

    def add_argument(self, short: str, long: str, type: any, choices: list, action: str, default: bool, help: str):
        """Add a new argument

        Args:
            short (str): The short hand terminal argument
            long (str): The long hand terminal code
            type (any): The type of the required argument
            choices (list): The optional choices
            action (str): idk something about how data works. ('store_const' means that we can make it w/ arguments)
            default (bool): The default value
            help (str): The help of the message
        """
        self.parser.add_argument(short, long, type=type, choices=choices,
                                 action=action, default=default, help=help, const=not (default))

    def GetArgument(self, value):
        return self.unknown[value]

    def main(self):
        """Get information

        Returns:
            A lot: Returns data, and other arguments
        """
        # supports multiple command line args
        other, Read = self.__GetParser()
        return self.__read(other, Read)

    def output(self):
        """Like the last thing to print Just general information
        """
        print(f"Day {self.day} elf results:")
        print(f"Total time: {time.time() - st}")

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

    def shrink(self, a: int, b: int):
        """Reduce an int down but still make it usable

        Args:
            a (int): something here
            b (int): something here

        Returns:
            _type_: _description_
        """
        d, m = divmod(a, b)
        return m
