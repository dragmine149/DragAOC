import copy
import ipdb
from helper import helper


class Visual:
    @staticmethod
    def print(gdata, pos, ePos, pData):
        print(f'\033[33m{"-" * len(gdata[0])}\033[0m')
        print(pData)
        for yI, yV in enumerate(gdata):
            for xI, xV in enumerate(yV):
                if xI == pos.get("X") and yI == pos.get("Y"):
                    print(f'\033[32m{xV}\033[0m', end='')
                elif xI == ePos.get("X") and yI == ePos.get("Y"):
                    print(f'\033[31m{xV}\033[0m', end='')
                else:
                    print(f'{xV}', end='')
            print()


class main:
    def __init__(self) -> None:
        self.position = {
            "X": 0,
            "Y": 0
        }
        self.EndPosition = {
            "X": 0,
            "Y": 0
        }
        self.grid = []
        self.steps = 0
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

        self.getInfo()

    def getInfo(self):
        for yI, yV in enumerate(self.grid):
            for xI, xV in enumerate(yV):
                if xV == "S":
                    self.position = {
                        "X": xI,
                        "Y": yI
                    }
                if xV == "E":
                    self.EndPosition = {
                        "X": xI,
                        "Y": yI
                    }

    def __reachedEnd(self):
        posX, EposX = self.position.get("X"), self.EndPosition.get("X")
        posY, EposY = self.position.get("Y"), self.EndPosition.get("Y")
        return posX == EposX and posY == EposY

    def vP(self):
        Visual.print(self.grid, self.position, self.EndPosition, self.path)

    def __GetSquare(self, x=None, y=None):
        if x is None:
            x = self.position.get("X")
        if y is None:
            y = self.position.get("Y")

        v = ord(self.grid[y][x])

        if v == ord('S'):
            return ord('a')

        if v == ord('E'):
            return ord('z')

        return v

    def __GetSurroundings(self):
        Suroundings = {
            "L": 0,
            "R": 0,
            "U": 0,
            "D": 0
        }
        if self.position.get("Y") - 1 >= 0:
            Suroundings["U"] = self.__GetSquare(
                self.position.get("X"), self.position.get("Y") - 1)

        if self.position.get("Y") + 1 <= len(self.grid) - 1:
            Suroundings["D"] = self.__GetSquare(
                self.position.get("X"), self.position.get("Y") + 1)

        if self.position.get("X") - 1 >= 0:
            Suroundings["L"] = self.__GetSquare(
                self.position.get("X") - 1, self.position.get("Y"))

        if self.position.get("X") + 1 <= len(self.grid[0]) - 1:
            Suroundings["R"] = self.__GetSquare(
                self.position.get("X") + 1, self.position.get("Y"))

        return Suroundings

    def __MoveCheck(self, cV, sur, x, y):
        if cV + 1 == sur:
            if self.path[self.position.get("Y") + y][self.position.get("X") + x] == ".":
                return 1
            return 0  # Already been that way

        return 2 if cV == sur else None

    def __DecodeV(self, v: int, pD: list, d: str):
        """Decode v

        Args:
            v (int): v
            pD (list): The list if v is 2
            d (str): The direction
        """
        if v == 2:
            pD.append(d)

        if v == 1:
            return pD, self.__TranslateDir(d)

        return pD, None

    def __Simulate(self, direction, value):
        count = 0
        noneEmpty = 0
        tmpPos = copy.deepcopy(self.position)
        squareValue = self.__GetSquare()

        self.position[direction] += value
        sur = self.__GetSurroundings()

        for item in sur:
            if sur.get(item) == squareValue + 1:
                count += 1

            if sur.get(item) != 0 and sur.get(item) == squareValue:
                noneEmpty += 1

        self.position = tmpPos
        return count if count != 0 else noneEmpty

    def __TranslateDir(self, direction: str):
        """Translate the direction into x and y

        Args:
            direction (str): The direction (u, d, l, r)

        Raises:
            ValueError: Invalid dir

        Returns:
            Tuple: The information
        """
        match direction:
            case "U":
                return "Y", -1, "^"
            case "D":
                return "Y", 1, "V"
            case "L":
                return "X", -1, "<"
            case "R":
                return "X", 1, ">"

        raise ValueError("Dir is not valid")

    def __newMove(self, currentValue: int, surroundings: dict):
        possibleDirections = []

        # Try to move up
        v = self.__MoveCheck(currentValue, surroundings["U"], 0, -1)
        possibleDirections, dI = self.__DecodeV(v, possibleDirections, "U")
        if dI is not None:
            return dI

        # Try to move down
        v = self.__MoveCheck(currentValue, surroundings["D"], 0, 1)
        possibleDirections, dI = self.__DecodeV(v, possibleDirections, "D")
        if dI is not None:
            return dI

        # Try to move left
        v = self.__MoveCheck(currentValue, surroundings["L"], -1, 0)
        possibleDirections, dI = self.__DecodeV(v, possibleDirections, "L")
        if dI is not None:
            return dI

        # Try to move right
        v = self.__MoveCheck(currentValue, surroundings["R"], 1, 0)
        possibleDirections, dI = self.__DecodeV(v, possibleDirections, "R")
        if dI is not None:
            return dI

        count = 1_000
        chosen = ""
        for direction in possibleDirections:
            crDir, amt, _ = self.__TranslateDir(direction)
            cnt = self.__Simulate(crDir, amt)
            if cnt < count:
                count = cnt
                chosen = direction

        drXY, amt, pth = self.__TranslateDir(chosen)
        return drXY, amt, pth

    def main(self):
        # while not self.__reachedEnd():
        for _ in range(30):
            if self.__reachedEnd():
                return

            ipdb.set_trace()
            self.vP()
            currentValue = self.__GetSquare()
            surroundings = self.__GetSurroundings()
            direct, amount, path = self.__newMove(currentValue, surroundings)
            self.path[self.position.get("Y")][self.position.get("X")] = path
            self.position[direct] += amount


if __name__ == "__main__":
    hlper = helper(12)
    data = hlper.main()
    m = main()
    m.generateGrid(data)
    print(m.grid)
    m.main()
    hlper.output()
