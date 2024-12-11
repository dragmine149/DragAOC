# Get inputs
def getInputs():
    lines = []
    with open('Inputs.txt', 'r') as f:
        lines = f.readlines()
    return lines


class Class:
    def __init__(self):
        self.h = 0
        self.aim = 0
        self.depth = 0

    def ProcessInput(self, value):
        split = value.split(" ")
        if (split[0] == 'down'):
            self.aim = self.aim + int(split[1])
        if split[0] == 'up':
            self.aim = self.aim - int(split[1])
        if split[0] == 'forward':
            self.h = self.h + int(split[1])
            self.depth = self.depth + (self.aim * int(split[1]))


if __name__ == '__main__':
    lines = getInputs()
    PI = Class()
    for i in range(0, len(lines)):
        PI.ProcessInput(lines[i])
    print(PI.h, PI.depth, PI.aim)
    print(PI.h * PI.depth)
