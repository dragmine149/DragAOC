import ipdb
from helper import helper as h
from helper import AOCException
from helper import math


class Visual:
    @staticmethod
    def printPath(path, pos, ePos):
        print(f'\033[33m{"-" * len(path[0])}\033[0m')
        for yI, yV in enumerate(path):
            for xI, xV in enumerate(yV):
                if xI == pos.get("X") and yI == pos.get("Y"):
                    print(f'\033[32m{xV}\033[0m', end='')
                elif xI == ePos.get("X") and yI == ePos.get("Y"):
                    print(f'\033[31m{xV}\033[0m', end='')
                else:
                    print(f'{xV}', end='')
            print()

    @staticmethod
    def printData(gdata, pos, ePos):
        print(f'\033[33m{"-" * len(gdata[0])}\033[0m')
        for yI, yV in enumerate(gdata):
            for xI, xV in enumerate(yV):
                if xI == pos.get("X") and yI == pos.get("Y"):
                    print(f'\033[32m{xV}\033[0m', end='')
                elif xI == ePos.get("X") and yI == ePos.get("Y"):
                    print(f'\033[31m{xV}\033[0m', end='')
                else:
                    print(f'{xV}', end='')
            print()


class mazeGeneration:
    def __init__(self) -> None:
        self.grid = []
        self.path = []

    def generateGrid(self, gridData):
        for line in gridData:
            x = []
            p = []
            for l in line.strip():
                x.append(l)
                p.append('.')
            self.grid.append(x)
            self.path.append(p)

    def getInfo(self):
        pos = {
            "X": 0,
            "Y": 0
        }
        ePos = {
            "X": 0,
            "Y": 0
        }
        S = False
        E = False
        for yI, yV in enumerate(self.grid):
            for xI, xV in enumerate(yV):
                if xV == "S":
                    print("Found start")
                    pos = {
                        "X": xI,
                        "Y": yI
                    }
                    S = True
                if xV == "E":
                    print("Found end")
                    ePos = {
                        "X": xI,
                        "Y": yI
                    }
                    E = True

                if S and E:
                    break

        return pos, ePos


class maze:
    def __init__(self, data) -> None:
        self.steps = 0
        self.mGen = mazeGeneration()
        self.mGen.generateGrid(data)

        self.position, self.EndPosition = self.mGen.getInfo()
        self.grid = self.mGen.grid
        self.path = self.mGen.path

    def vP(self):
        """Prints the grid
        """
        Visual.printData(self.grid, self.position, self.EndPosition)
        Visual.printData(self.path, self.position, self.EndPosition)

    def GetSquare(self, x: int, y: int, path: bool = False) -> str | int:
        """Returns information about that square

        Args:
            x (int): The x position
            y (int): The y position
            path (bool, optional): To get the value of that square in the path grid. Defaults to False.

        Returns:
            str | int: The result, string if path is true. Int if path is false.
        """
        if path:
            return self.path[y][x]

        if x < 0 or x >= len(self.grid[0]):
            return 0

        if y < 0 or y >= len(self.grid):
            return 0
        
        if self.path[y][x] != ".":
            return 0

        v = self.grid[y][x]
        if v == "S":
            return ord('a')
        if v == "E":
            return ord('z')
        return ord(v)

    def GetSurroundings(self, x: int, y: int) -> dict[int]:
        """Returns the suroundings values around (x, y)

        Args:
            x (int): X position
            y (int): Y position

        Returns:
            dict[int]: The values of the squares surrounding them.
        """
        surroundings = {
            # "-1,-1": self.GetSquare(x - 1, y - 1),
            "0,-1": self.GetSquare(x, y - 1),
            # "1,-1": self.GetSquare(x + 1, y - 1),
            "-1,0": self.GetSquare(x - 1, y),
            "1,0": self.GetSquare(x + 1, y),
            # "-1,1": self.GetSquare(x - 1, y + 1),
            "0,1": self.GetSquare(x, y + 1),
            # "1,1": self.GetSquare(x + 1, y + 1)
        }
        return surroundings

    def split(self, value: str) -> tuple[int]:
        """Split a surrounding value up into two numbers

        Args:
            value (str): The value to split

        Returns:
            tuple[int]: The result of that value (as int)
        """
        valueInfo = value.split(",")
        return int(valueInfo[0]), int(valueInfo[1])

    def __moveLoop(self, table: list, x: int, y: int) -> tuple[tuple[int], int]:
        """Get the value of the next square based on stuff.

        Args:
            table (list): The table to loop through
            x (int): The x positon
            y (int): The y position

        Returns:
            tuple[tuple[int], int]: The return value
        """
        if len(table) == 0:
            return (None, None)

        choice = ((0, 0), math.max)
        for tbl in table:
            nX, nY = self.split(tbl)
            surValue = self.GetSurroundings(x + nX, y + nY)
            direction, value = self.move(x + nX, y + nY, surValue, True)
            if value < choice[1]:
                choice = (direction, value)

        return choice

    def __NullCheck(self, info: any) -> bool:
        """Check if info is None

        Args:
            info (any): The value to check

        Returns:
            bool: Is it none
        """
        if info is None:
            return True

        if isinstance(info, (str, int)):
            return False

        v = True
        for x in info:
            if x is not None:
                v = False

        return v

    def FinishCheck(self):
        pos = self.position
        ePos = self.EndPosition
        return pos.get("X") == ePos.get("X") and pos.get("Y") == ePos.get("Y")

    def TranslateDirection(self, direction: str | tuple[int]) -> str:
        if isinstance(direction, str):
            direction = direction.split(",")
            direction[0] = int(direction[0])
            direction[1] = int(direction[1])
        
        if direction[0] == 0 and direction[1] == -1:
            return direction, "^"
        if direction[0] == 1 and direction[1] == 0:
            return direction, ">"
        if direction[0] == 0 and direction[1] == 1:
            return direction, "V"
        if direction[0] == -1 and direction[1] == 0:
            return direction, "<"

        raise AOCException(f"Okay, direction: {direction}")

    def move(self, x: int,
             y: int,
             surroundings: dict[int],
             limit: bool = False) -> tuple[tuple[int], int]:
        """Do some math, return some stuff, find the best way for the player to move

        Args:
            x (int): The x position
            y (int): The y position
            surroundings (dict[int]): The surrounding objects
            limit (bool, optional): Do we loop again. Defaults to False.

        Raises:
            AOCException: Something unexcepted happened

        Returns:
            tuple[tuple[int], int]: The good stuff
        """
        currentV = self.GetSquare(x, y)
        similuar = []
        better = []

        for surround in surroundings:
            surV = surroundings.get(surround)
            # Skip if invalid
            if surV == 0:
                continue

            # If better
            if surV == currentV + 1:
                better.append(surround)
                continue

            # If equaal
            if surV == currentV:
                similuar.append(surround)
                continue

            continue

        if len(better) == 1:
            return self.split(better[0]), 0

        if len(better) == 0 and len(similuar) == 1:
            return similuar[0], 1

        if not limit:
            choice = self.__moveLoop(better, x, y)
            choice2 = self.__moveLoop(similuar, x, y)

            if self.__NullCheck(choice) and self.__NullCheck(choice2):
                raise AOCException("How did you get here?")

            if self.__NullCheck(choice):
                return choice2

            return choice

        if len(better) == 0:
            return similuar[0], 1

        return better[0], 0

    def main(self):
        while not self.FinishCheck():
            self.vP()
            
            ipdb.set_trace()

            sX = self.position.get("X")
            sY = self.position.get("Y")

            if self.__NullCheck(sX):
                raise AOCException("Null x position...?")

            if self.__NullCheck(sY):
                raise AOCException("Null y position...?")

            surroundings = self.GetSurroundings(sX, sY)
            direction, _ = self.move(sX, sY, surroundings)
            direction, self.path[sY][sX] = self.TranslateDirection(direction)
            self.position["X"] += direction[0]
            self.position["Y"] += direction[1]
            
            self.steps += 1


if __name__ == "__main__":
    helper = h(12)
    dayData = helper.main()
    mze = maze(dayData)
    mze.main()
    helper.output()
    mze.vP()
    print(mze.steps)
