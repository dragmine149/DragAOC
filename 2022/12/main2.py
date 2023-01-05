import ipdb
from helper import helper as h


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

    def GetSquare(self, x: int, y: int) -> int:
        """Get the value of the current square.

        Args:
            x (int): The x coord
            y (int): The y coord

        Returns:
            int: The value of that square
        """
        v = self.grid[y][x]
        if v == "S":
            return ord('a')
        if v == "E":
            return ord('z')
        return ord(v)

    def GetSurroundings(self, x: int, y: int) -> dict:
        """Get the value of the surroundings square

        Args:
            x (int): The x position
            y (int): The y position

        Returns:
            dict: The dictionary of directions and values
        """
        sur = {
            "U": 0,
            "L": 0,
            "R": 0,
            "D": 0
        }

        if y - 1 >= 0:
            sur["U"] = self.GetSquare(x, y - 1)

        if y + 1 < len(self.grid) - 1:
            sur["D"] = self.GetSquare(x, y + 1)

        if x - 1 >= 0:
            sur["L"] = self.GetSquare(x - 1, y)

        if x + 1 < len(self.grid[0]) - 1:
            sur["R"] = self.GetSquare(x + 1, y)

        return sur

    def Visited(self, x: int, y: int) -> bool:
        return self.path[y][x] != "."

    def Translate(self, direction: str) -> tuple:
        match direction:
            case "U":
                return 0, -1, "^"
            case "R":
                return 1, 0, ">"
            case "D":
                return 0, 1, "V"
            case "L":
                return -1, 0, "<"

        return 0, 0, None

    def Simulate(self, directions: dict, x: int, y: int) -> tuple:
        """Loop and try and get good information out of stuff

        Args:
            directions (dict): The directions to loop through
            x (int): The x value
            y (int): The y value

        Returns:
            tuple: The result
        """
        # Mediocore, Do the same thing again, but get compare the new value.
        values = [[], []]
        for direct in directions:
            x1, y1, _ = self.Translate(direct)
            direc, v = self.Getvalue(
                x + x1, y + y1, self.GetSurroundings(x + x1, y + y1))
            values[v].append(direc)

        if len(values[0]) == 1:
            return values[0], 0

        if len(values[1]) == 1 and len(values[0]) < 2:
            return values[1], 0

        if len(values[0]) >= 2:
            return values[0][0], 0

        if len(values[1]) >= 2:
            return values[1][0], 0

        return None, None

    def Getvalue(self, x: int, y: int, sur: dict) -> str:
        """Works out the best way to go

        Args:
            x (int): The x coord of the curent square
            y (int): The y coord of the curent square
            sur (dict): The surrounding squares values

        Returns:
            str: Information TBA
        """

        # Split the surroundings up into good (go higher) or ok (same level)
        directions = []
        other = []
        for value in sur:
            if sur.get(value) == self.GetSquare(x, y) + 1 and not self.Visited(x, y):
                directions.append(value)

            if sur.get(value) == self.GetSquare(x, y) and not self.Visited(x, y):
                other.append(value)

        if len(directions) == 0 and len(other) == 0:
            raise ValueError(
                "...? how? we must be able to go somewhere right?")

        # The best, goes up a square
        if len(directions) == 1:
            return directions[0], 0

        # Check if other is better
        if len(other) == 1 and len(directions) < 2:
            return other[0], 1

        d, i = self.Simulate(directions, x, y)
        if d is not None and i is not None:
            return d, i

        d, i = self.Simulate(other, x, y)
        if d is not None and i is not None:
            return d, i

        raise ValueError("Something went wrong and nothing got returned!")

    def Move(self):
        """Actuall do the moving now!
        """
        # Store x and y
        sX = self.position.get("X")
        sY = self.position.get("Y")

        # Get info
        surroundings = self.GetSurroundings(sX, sY)
        ipdb.set_trace()
        direction, _ = self.Getvalue(sX, sY, surroundings)
        posDirection, posAmount, path = self.Translate(direction)

        # Update info
        self.path[sY][sX] = path
        self.position[posDirection] += posAmount
        self.vP()


if __name__ == "__main__":
    helper = h(12)
    helperData = helper.main()
    mze = maze(helperData)
    mze.vP()
    mze.Move()
