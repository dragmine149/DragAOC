import file
import time


class Class():
    def __init__(self):
        self.fish = self.getData()

    def getData(self):
        lines = file.get()
        line = lines[0]
        return line.split(',')

    def Loop(self):
        for value in range(len(self.fish)):
            # removes day before next birth
            self.fish[value] = int(self.fish[value]) - 1
            if self.fish[value] == -1:
                self.fish[value] = 6
                self.fish.append(8)


if __name__ == '__main__':
    begin = time.time()
    c = Class()
    print(len(c.fish))

    for i in range(80):  # 80
        c.Loop()

    print(len(c.fish))
    end = time.time()
    print(f'Total Time: {end - begin}')
