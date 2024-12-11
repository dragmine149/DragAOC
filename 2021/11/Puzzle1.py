import file
import run


class Class():
    def __init__(self, debug):
        self.debug = debug.lower()[0] == 't'
        self.lines = file.get()
        if self.debug:
            self.lines = file.get2()
        self.splitLines()
        self.flashes = 0

    def splitLines(self):
        max = []
        for value in self.lines:
            arr = []
            for v in value.replace('\n', ''):
                arr.append(int(v))
            max.append(arr)
        self.lines = max

    def Loop(self, steps):
        for i in range(steps):
            flashes = 0
            # increment
            for Intarr in range(len(self.lines)):
                arr = self.lines[Intarr]
                for Intvalue in range(len(arr)):
                    arr[Intvalue] += 1

            # flash and share
            for Intarr in range(len(self.lines)):
                arr = self.lines[Intarr]
                for Intvalue in range(len(arr)):
                    value = arr[Intvalue]
                    if value >= 9:
                        flashes += 1

                        # finds value in thing
                        if not (Intarr - 1) < 0 and not (Intarr + 1) >= len(self.lines):  # noqa
                            self.lines[Intarr - 1][Intvalue] += 1
                            if self.lines[Intarr - 1][Intvalue] == 9:
                                flashes += 1

                            if not (Intvalue - 1) < 0:
                                self.lines[Intarr - 1][Intvalue - 1] += 1
                                if self.lines[Intarr - 1][Intvalue - 1] == 9:
                                    flashes += 1

                            if not (Intvalue + 1) >= len(self.lines[Intvalue - 1]):  # noqa
                                self.lines[Intarr - 1][Intvalue + 1] += 1
                                if self.lines[Intarr - 1][Intvalue + 1] == 9:
                                    flashes += 1

                        if not (Intarr + 1) < 0 and not (Intarr + 1) >= len(self.lines):  # noqa
                            self.lines[Intarr + 1][Intvalue] += 1

                            if not (Intvalue - 1) < 0:
                                self.lines[Intarr + 1][Intvalue - 1] += 1

                            if not (Intvalue + 1) >= len(self.lines[Intvalue - 1]):  # noqa
                                self.lines[Intarr + 1][Intvalue + 1] += 1

                        if not (Intvalue - 1) < 0:
                            self.lines[Intarr][Intvalue - 1] += 1
                            if self.lines[Intarr][Intvalue - 1] == 9:
                                flashes += 1

                        if not (Intvalue + 1) >= len(self.lines[Intarr]):
                            self.lines[Intarr][Intvalue + 1] += 1

            # resets time
            for Intarr in range(len(self.lines)):
                arr = self.lines[Intarr]
                for Intvalue in range(len(arr)):
                    value = arr[Intvalue]
                    if value >= 9:
                        self.lines[Intarr][Intvalue] = 0
            print(flashes)
            self.flashes += flashes

    def getLine(self):
        length = 0
        for value in self.lines:
            length += len(value)
        return length


if __name__ == '__main__':
    c = Class(input('Debug?: '))
    c.Loop(int(input('Number of steps?: ')))
    print(f'Len:{c.getLine()}')
    print(f'Flashes: {c.flashes}')
    print(run.End())
