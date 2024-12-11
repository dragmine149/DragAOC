import file
import run


class Class():
    def __init__(self, Debug):
        self.Debug = Debug.lower()[0] == 't'
        self.lines = file.get()
        if self.Debug:
            self.lines = file.get2()
        # self.points = [')', ']', '}', '>']
        self.points = [1, 2, 3, 4]
        self.rest = []
        self.score = []
        self.SCORE = 0

    def getOther(self, char):
        if char == '[':
            return ']'
        if char == '(':
            return ')'
        if char == '<':
            return '>'
        if char == '{':
            return '}'
        return 'Error'

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
                    # corrupted lines should die.
                    tempstr = ''  # no point in having line
                    break

                # if (charValue == len(line) - 1):
            #         print('FULL')
            #
            # print(f'End:{tempstr}')
            if tempstr != '':
                self.rest.append(self.finish(tempstr))
            # print('\n')

    def finish(self, line):
        chars = []
        while len(line) != 0:
            char = self.getOther(line[len(line) - 1:])
            chars.append(char)
            line = line[:-1]
        return chars

    def getScore(self):
        for table in self.rest:
            score = 0
            for value in table:
                score *= 5
                if value == ')':
                    score += self.points[0]
                if value == ']':
                    score += self.points[1]
                if value == '}':
                    score += self.points[2]
                if value == '>':
                    score += self.points[3]
            self.score.append(score)
        self.score.sort()

        # get mid point
        midpoint = round(len(self.score) / 2)
        self.SCORE = self.score[midpoint]


if __name__ == '__main__':
    c = Class(input('Debug?: '))
    c.sort()
    c.getScore()
    print(c.score)
    print(c.SCORE)
    print(run.End())
