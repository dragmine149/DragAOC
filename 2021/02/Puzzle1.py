# Get inputs
def getInputs():
    lines = []
    with open('Inputs.txt', 'r') as f:
        lines = f.readlines()
    return lines


class Class:
    def __init__(self):
        self.horizontal = 0
        self.depth = 0

    def ProcessInput(self, value):
        objects = value.split(" ")
        if objects[0] == 'forward':
            self.horizontal = self.horizontal + int(objects[1])
        if objects[0] == 'down':
            self.depth = self.depth + int(objects[1])
        if objects[0] == 'up':
            self.depth = self.depth - int(objects[1])


if __name__ == '__main__':
    lines = getInputs()
    PI = Class()
    for i in range(0, len(lines)):
        PI.ProcessInput(lines[i])
    print(PI.horizontal, PI.depth)
    print(PI.horizontal * PI.depth)
