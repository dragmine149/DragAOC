import file
import run


class Class():
    def __init__(self):
        self.lines = file.get2()
        self.arr = []
        self.total = 0

    # def find(self):
    #     maxArr = []
    #     for intLine in range(len(self.lines)):  # len(self.lines)
    #         temp = []
    #         line = self.lines[intLine].replace('\n', '')
    #         for value in range(len(line)):
    #             # get numbers on the top or bottom
    #             top = 9
    #             bottom = 9
    #             if intLine != 0:
    #                 top = self.lines[intLine - 1][value]
    #             if intLine != (len(self.lines) - 1):
    #                 bottom = self.lines[intLine + 1][value]
    #
    #             # get numbers on the side
    #             left = 9
    #             right = 9
    #             if value != 0:
    #                 left = line[value - 1]
    #             if value != (len(line) - 1):
    #                 right = line[value + 1]
    #
    #             v = int(line[value])
    #             if v < int(top) or v < int(bottom) or v < int(left) or v < int(right):  # noqa
    #                 temp.append(line[value])
    #         maxArr.append(temp)
    #
    #     valueArr = []
    #     lastnum = 0
    #     start = True
    #     for value in range(len(maxArr)):
    #         for item in range(len(maxArr[value])):
    #             v = int(maxArr[value][item])
    #
    #             # removes values of 9
    #             if v == 9:
    #                 continue
    #
    #             # append if first value
    #             if item == 0:
    #                 # check last value to see if new table needed
    #                 if v > lastnum and not start:
    #                     self.arr.append(valueArr)
    #                     valueArr = [v]
    #                     continue
    #                 start = False
    #                 valueArr.append(v)
    #                 continue
    #
    #             # check if value is last value
    #             if item + 1 == len(maxArr[value]):
    #                 valueArr.append(v)
    #                 continue
    #
    #             # check value with the previous
    #             if v < int(maxArr[value][item + 1]) or v < int(maxArr[value][item - 1]):  # noqa
    #                 valueArr.append(v)
    #                 continue
    #
    #             # make new array if needed
    #             if v > int(maxArr[value][item - 1]):
    #                 self.arr.append(valueArr)
    #                 valueArr = [v]
    #                 continue
    #
    #             lastnum = v

    def findLow(self):
        for intLine in range(len(self.lines)):  # len(self.lines)
            line = self.lines[intLine].replace('\n', '')
            for value in range(len(line)):
                # get numbers on the top or bottom
                top = 9
                bottom = 9
                if intLine != 0:
                    top = self.lines[intLine - 1][value]
                if intLine != (len(self.lines) - 1):
                    bottom = self.lines[intLine + 1][value]

                # get numbers on the side
                left = 9
                right = 9
                if value != 0:
                    left = line[value - 1]
                if value != (len(line) - 1):
                    right = line[value + 1]

                v = int(line[value])
                if v < int(top) and v < int(bottom) and v < int(left) and v < int(right):  # noqa
                    self.arr.append([intLine, line[value]])
                    # self.total = self.total + v + 1

    def checkFalse(self, arr):
        for value in arr:
            if value:
                return False
        return True

    def find(self):
        for value in self.arr:
            line = self.lines[value[0]].replace('\n', '')
            start = int(value[1])
            list = [line[start]]
            directions = [True, True, True, True]

            while not self.checkFalse(directions):
                # search around for 1 higher number
                if start > 0 and start - 1 <= len(line):  # range check

                    if int(line[start - 1]) == (int(line[start]) + 1):
                        list.append(int(line[start - 1]))
                        print('left')
                        start = int(value[0]) - 1
                    else:
                        directions[0] = False

                    if int(line[start + 1]) == (int(line[start]) + 1):
                        list.append(int(line[start + 1]))
                        print('right')
                        start = int(value[0]) + 1
                    else:
                        directions[2] = False
                else:
                    directions[0] = False
                    directions[2] = False

                if int(value[0]) >= 0 and int(value[0] + 1) <= len(self.lines):

                    if int(self.lines[value[0] - 1]) == (int(line[start]) + 1):
                        list.append(int(self.lines[value[0] - 1]))
                        print('up')
                        line = self.lines[value[0] - 1].replace('\n', '')
                    else:
                        directions[1] = False

                    if int(self.lines[value[0] + 1]) == (int(line[start]) + 1):
                        list.append(int(self.lines[value[0] + 1]))
                        print('down')
                        line = self.lines[value[0] + 1].replace('\n', '')
                    else:
                        directions[3] = False
                else:
                    directions[1] = False
                    directions[3] = False

                print(list)


if __name__ == '__main__':
    c = Class()
    c.findLow()
    c.find()
    print(c.arr)
    # print(c.total)
    print(run.End())
