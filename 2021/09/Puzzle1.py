import file
import run


class Class():
    def __init__(self):
        self.lines = file.get()
        self.arr = []
        self.total = 0

    def find(self):
        for intLine in range(len(self.lines)):  # len(self.lines)
            line = self.lines[intLine].replace('\n', '')
            for value in range(len(line)):
                # get numbers on the top or bottom
                top = 9
                bottom = 9
                if intLine != 0:
                    top = self.lines[intLine - 1][value]
                if intLine != (len(self.lines) - 1):
                    bottom = self.lines[intLine + 1][value]

                # get numbers on the side
                left = 9
                right = 9
                if value != 0:
                    left = line[value - 1]
                if value != (len(line) - 1):
                    right = line[value + 1]

                # print(f'Value: {line[value]}, Top: {top}, Bottom: {bottom}, Left: {left}, Right:{right}')  # noqa
                v = int(line[value])
                if v < int(top) and v < int(bottom) and v < int(left) and v < int(right):  # noqa
                    self.arr.append(line[value])
                    self.total = self.total + v + 1


if __name__ == '__main__':
    c = Class()
    c.find()
    print(c.arr)
    print(c.total)
    print(run.End())
