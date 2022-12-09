import time
import ipdb
import helper


class Visual:
    """Colour info:
    Orange: Selected
    Green: Visible
    Red: Not visible
    """

    def __init__(self, data, visual) -> None:
        self.data = data
        self.colourInfo = {}
        self.visual = visual

    def Print(self, colour: str, x: int, y: int):
        if self.visual:
            print("\x1b[2J\x1b[H", end="")
            self.colourInfo[f"{x, y}"] = colour
            for i, v in enumerate(self.data):
                for j, v2 in enumerate(v):
                    colourInf = self.colourInfo.get(f'{i, j}')
                    colourInf = colourInf if colourInf is not None else ''
                    print(f"{colourInf}{v2}\033[0m", end="")
                print()
            time.sleep(0.1)

# g g g g g
# g g g r g
# g g r g g
# g r g r g
# g g g g g


class Grid:
    def __init__(self, data, visual) -> None:
        self.data = []

        temp = []
        for item in data:
            item = item.strip()
            for num in item:
                temp.append(int(num))
            self.data.append(temp)
            temp = []

        self.highest = 0
        self.Visual = Visual(self.data, visual)

    def LeftEdgeCheck(self, height: int, x: int, y: int) -> bool:
        TreeCount = 0
        for i in range(y - 1, -1, -1):
            TreeCount += 1
            if self.data[x][i] >= height:
                return TreeCount
        return TreeCount

    def RightEdgeCheck(self, height: int, x: int, y: int) -> bool:
        TreeCount = 0
        for i in range(y + 1, len(self.data[0])):
            TreeCount += 1
            if self.data[x][i] >= height:
                return TreeCount
        return TreeCount

    def topEdgeCheck(self, height: int, x: int, y: int) -> bool:
        TreeCount = 0
        for i in range(x - 1, -1, -1):
            TreeCount += 1
            if self.data[i][y] >= height:
                return TreeCount
        return TreeCount

    def bottomEdgeCheck(self, height: int, x: int, y: int) -> bool:
        TreeCount = 0
        for i in range(x + 1, len(self.data)):
            TreeCount += 1
            if self.data[i][y] >= height:
                return TreeCount
        return TreeCount

    def CheckTree(self, y, x, height):
        # ipdb.set_trace()
        self.Visual.Print('\033[33m', x, y)

        if x == 0 or y == 0 or y == len(self.data) - 1 or x == len(self.data[0]) - 1:
            self.Visual.Print('\033[32m', x, y)
            return 0

        top = self.topEdgeCheck(height, x, y)
        left = self.LeftEdgeCheck(height, x, y)
        bottom = self.bottomEdgeCheck(height, x, y)
        right = self.RightEdgeCheck(height, x, y)
        
        print(f"COORDS: {x, y} Top, left, bottom, right: {top, left, bottom, right}. Score: {top * left * bottom * right}")

        # print(f"h: {height}, x: {x}, y: {y}")
        # input(f"{top}, {left}, {bottom}, {right}")

        self.Visual.Print('\033[32m', x, y)
        return top * left * bottom * right

    def main(self):
        for yIn, yV in enumerate(self.data):
            for xIn, xV in enumerate(yV):
                visible = self.CheckTree(xIn, yIn, xV)
                if visible > self.highest:
                    self.highest = visible
                    print(f"New High! Coords: {xIn, yIn} Height: {xV}, Value: {visible}")


if __name__ == "__main__":
    d, v = helper.main()
    g = Grid(d, v)
    g.main()
    helper.output(8)
    print(f"Highest scenic score: {g.highest}")
