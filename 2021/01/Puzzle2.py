import sys
sys.path.append('../')
import file  # noqa E402


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


# Main program
if __name__ == '__main__':
    lines = file.get()
    results = 0

    for i in range(0, len(lines)):
        if i == 1997:
            break

        group1 = (int(lines[i].replace('\n', '')) +
                  int(lines[i + 1].replace('\n', '')) +
                  int(lines[i + 2].replace('\n', '')))
        group2 = (int(lines[i + 1].replace('\n', '')) +
                  int(lines[i + 2].replace('\n', '')) +
                  int(lines[i + 3].replace('\n', '')))
        if (getDiff(group1, group2)) == 'increased':
            results = results + 1

    print(results)
