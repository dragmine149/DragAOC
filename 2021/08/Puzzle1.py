import file
import run


class Class():
    def __init__(self):
        self.input = file.get()
        self.Codes = [['a', 'b', 'c', 'e', 'f', 'g'],  # 6
                      ['c', 'f'],  # 2
                      ['a', 'b', 'd', 'f', 'g'],  # 5
                      ['b', 'c', 'd', 'f'],  # 4
                      ['a', 'b', 'd', 'f', 'g'],  # 5
                      ['a', 'b', 'd', 'e', 'f', 'g'],  # 6
                      ['a', 'c', 'f'],  # 3
                      ['a', 'b', 'c', 'd', 'e', 'f', 'g'],  # 7
                      ['a', 'b', 'c', 'd', 'f', 'g']]  # 6
        self.total = 0

    def decode(self):
        for line in self.input:
            newLine = line.split('|')
            for value in newLine[1].split(' '):
                value = value.replace('\n', '')
                if len(value) == 2 or len(value) == 3 or len(value) == 4 or len(value) == 7:
                    self.total = self.total + 1


if __name__ == '__main__':
    c = Class()
    c.decode()
    print(f'Total: {c.total}')
    print(run.End())
