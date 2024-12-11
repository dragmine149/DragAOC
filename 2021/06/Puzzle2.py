import file
import time


class Class():
    def __init__(self):
        self.fish = [0, 0, 0, 0, 0, 0, 0, 0, 0]
        self.getData()

    def getData(self):
        lines = file.get()
        line = lines[0]
        values = line.split(',')
        for value in values:
            self.fish[int(value)] = self.fish[int(value)] + 1

    def Loop(self):
        tempNum = self.fish[0]
        self.fish[0] = self.fish[1]
        self.fish[1] = self.fish[2]
        self.fish[2] = self.fish[3]
        self.fish[3] = self.fish[4]
        self.fish[4] = self.fish[5]
        self.fish[5] = self.fish[6]
        self.fish[6] = self.fish[7] + tempNum
        self.fish[7] = self.fish[8]
        self.fish[8] = tempNum

    def getValue(self):
        endVale = 0
        for i in range(len(self.fish)):
            endVale = endVale + self.fish[i]
        return endVale


if __name__ == '__main__':
    begin = time.time()
    c = Class()
    # print(c.fish)
    print(c.getValue())

    for i in range(256):  # 256
        c.Loop()

    # print(c.fish)
    print(c.getValue())
    end = time.time()
    print(f'Total Time: {end - begin}')
