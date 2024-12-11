def get():
    lines = []
    with open('Inputs.txt', 'r') as f:
        lines = f.readlines()
    return lines


def get2():
    lines = []
    with open('Inputs2.txt', 'r') as f:
        lines = f.readlines()
    return lines


def den(binaryValue):
    denV = 0
    for digit in binaryValue:
        denV = denV*2 + int(digit)
    return denV
