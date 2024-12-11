import time


# Get inputs
def getInputs():
    lines = []
    with open('Inputs.txt', 'r') as f:
        lines = f.readlines()
    return lines


# Gets the different between the 2 values
# - number = decreased, + = increased
def getDiff(old, new):
    if type(old) != int:
        old = old.replace('\n', '')

    if type(new) != int:
        new = new.replace('\n', '')

    print(old, new)
    diff = int(new) - int(old)
    print(diff)
    if diff < 0:
        return 'decreased'
    if diff == 0:  # forgot this in first code...
        return 'netrual'
    return 'increased'


# main part
if __name__ == '__main__':
    lines = getInputs()
    results = 0

    for i in range(0, len(lines)):
        time.sleep(0.05)
        if i == 1999:
            break  # end of loop

        if i == 0:  # first check
            if (getDiff(lines[0], lines[1])) == 'increased':
                results = results + 1

        else:  # other checks
            if getDiff(lines[i], lines[i+1]) == 'increased':
                results = results + 1

    print(results)
