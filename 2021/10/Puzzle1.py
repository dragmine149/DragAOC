import file
import run


class Class():
    def __init__(self, Debug):
        self.Debug = Debug.lower()[0] == 't'
        self.lines = file.get()
        if self.Debug:
            self.lines = file.get2()
        # self.points = [')', ']', '}', '>']
        self.points = [3, 57, 1197, 25137]
        self.illegal = []
        self.score = 0

    def getOther(self, char):
        if char == '[':
            return ']'
        if char == '(':
            return ')'
        if char == '<':
            return '>'
        if char == '{':
            return '}'

    def addNext(self, char):
        if char == '[':
            return True
        if char == '(':
            return True
        if char == '<':
            return True
        if char == '{':
            return True

    def sort(self):
        for line in range(len(self.lines)):
            # print(f'Line: {line + 1}')
            tempstr = ''
            line = self.lines[line].replace('\n', '')
            for charValue in range(len(line)):
                char = line[charValue]

                # checks if the string is not empty
                if len(tempstr) == 0 or self.addNext(char):
                    tempstr += char
                    continue

                # checks if the current value is equal to the last value.
                if self.getOther(tempstr[len(tempstr) - 1]) == char:
                    tempstr = tempstr[:-1]
                else:
                    # print('Unexcepted character!')
                    # print(f'Excepted: {self.getOther(tempstr[len(tempstr) - 1])}, Got: {char}')  # noqa
                    self.illegal.append(char)  # and char to table
                    break

                # if (charValue == len(line) - 1):
            #         print('FULL')
            #
            # print(f'End:{tempstr}')
            # print('\n')

    def getScore(self):
        for value in self.illegal:
            if value == ')':
                self.score += self.points[0]
            if value == ']':
                self.score += self.points[1]
            if value == '}':
                self.score += self.points[2]
            if value == '>':
                self.score += self.points[3]


if __name__ == '__main__':
    c = Class(input('Debug?: '))
    c.sort()
    c.getScore()
    print(c.score)
    print(run.End())
