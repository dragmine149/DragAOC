
import copy
import ipdb
from helper import helper, AOCException, Visual, Grid


class main:
    def __init__(self, data):
        self.data = data
        self.Grid = Grid()
        self.TranslateData()

    def TranslateData(self):
        # Okay, so to make sure that we don't generate a GIGANTIC list
        # all data - 462
        for line in self.data:
            line: str
            lineCords = line.split(" -> ")

            for l in lineCords:
                l: str
                xy = l.split(",")
                lX = int(xy[0]) - 462
                lY = int(xy[1])

                print(lX, lY)
                self.Grid.GenerateTo(lX, lY)

            oldCords = (0, 0)
            for l in lineCords:
                l: str
                xy = l.split(",")
                lX = int(xy[0]) - 462
                lY = int(xy[1])

                if oldCords[0] == 0 and oldCords[1] == 0:
                    oldCords = (lX, lY)

                ipdb.set_trace()
                self.Grid.grid[lY - 1][lX - 1] = "#"


if __name__ == "__main__":
    h = helper(14)
    dayData = h.main()
    m = main(dayData)
    h.output()
    Visual.print(m.Grid.grid)
    h.print(m.Grid.grid)
    h.print(f'x: {len(m.Grid.grid[0])} y: {len(m.Grid.grid)}')
