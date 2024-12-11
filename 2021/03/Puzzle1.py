import sys
sys.path.append('../')
import file  # noqa E402

'''
Binary input
need to split into 2
gamma + epsilon
power = gamme * epsilon
'''


def Test():
    line = "01001"
    for i in range(0, len(line)):
        print(line[i])


'''
TODO:
- Scroll through whole file.
- Get first item of each line
- Add to a value. (0 or 1 majority (most))
- Add values to string/table (majority)
- Return answer
- Convert to denary
'''


class Class():
    def __init__(self):
        self.gamma = ''
        self.epsilon = ''
        self.z = 0
        self.o = 0

    def SortValue(self, value):
        # Only two values (1 + 0)
        if (value == '0'):
            self.z = self.z + 1
            return
        self.o = self.o + 1

    def getMajority(self):
        if (self.z > self.o):
            self.gamma = self.gamma + '0'
            self.epsilon = self.epsilon + '1'
        else:
            self.gamma = self.gamma + '1'
            self.epsilon = self.epsilon + '0'
        self.z = 0
        self.o = 0


if __name__ == '__main__':
    c = Class()
    lines = file.get()
    position = 0
    for i in range(0, 12):
        for i in lines:
            c.SortValue(i[position])
        c.getMajority()
        position = position + 1

    print(c.gamma)
    print(c.epsilon)

    gamDen = 0
    for digit in c.gamma:
        gamDen = gamDen*2 + int(digit)

    epsDen = 0
    for digit in c.epsilon:
        epsDen = epsDen*2 + int(digit)

    print(gamDen)
    print(epsDen)

    print(gamDen * epsDen)
