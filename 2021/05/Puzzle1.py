import file
import numpy as np

# grid: 0,0 -> 989,990 = 981090


class Class():
    def __init__(self):
        self.board = []  # x = pos across. y = pos down
        self.points = 0
        self.debugMode = False
        self.genBoard()

    def genBoard(self):
        lineBoard = []
        size = 991
        if self.debugMode:
            size = 10
        for i in range(size):  # x 989
            lineBoard.append('.')
        npArrlineBoard = np.array(lineBoard)
        for i in range(size):  # 990
            arr2 = npArrlineBoard.copy()
            self.board.append(arr2)

    def processInputs(self):
        lines = None
        if self.debugMode:
            lines = file.get2()
        else:
            lines = file.get()

        def add(current):
            if current == '.':
                return '1'
            elif current.isdigit():
                return str(int(current) + 1)

        for line in lines:
            newLine = line.split(' -> ')  # x1,y1 + x2,y2

            posX1 = int(newLine[0].split(',')[0].replace('\n', ''))
            posX2 = int(newLine[1].split(',')[0].replace('\n', ''))
            posY1 = int(newLine[0].split(',')[1].replace('\n', ''))
            posY2 = int(newLine[1].split(',')[1].replace('\n', ''))

            # get diff
            diffX = posX2 - posX1
            diffY = posY2 - posY1

            startPosX = posX1
            startPosY = posY1
            self.board[startPosY][startPosX] = add(self.board[startPosY][startPosX])  # noqa
            for i in range(max(abs(diffY), abs(diffX))):
                if diffX > 0:
                    startPosX = startPosX + 1
                elif diffX < 0:
                    startPosX = startPosX - 1
                if diffY > 0:
                    startPosY = startPosY + 1
                elif diffY < 0:
                    startPosY = startPosY - 1
                self.board[startPosY][startPosX] = add(self.board[startPosY][startPosX]) # noqa
            '''
            # print(diffX, diffY)
            if (diffY > 0 or diffY < 0) and diffX == 0:
                Y = 0
                if diffY > 0:
                    Y = posY1
                elif diffY < 0:
                    Y = posY2
                else:
                    print('Error in diffY!')
                for i in range(abs(diffY) + 1):
                    current = self.board[i + Y][posX1]
                    # print(f'Current: {current}, i: "{i}", [{posX1}, {i + Y}]') # noqa E501
                    if current == '.':
                        current = '1'
                    elif current.isdigit():
                        current = str(int(current) + 1)

                    self.board[i + Y][posX1] = current

            # Horizontal
            if (diffX > 0 or diffX < 0) and diffY == 0:
                X = 0
                if diffX > 0:
                    X = posX1
                elif diffX < 0:
                    X = posX2
                else:
                    print('Error in diffX!')
                for i in range(abs(diffX) + 1):
                    # print(f'i: {i}, x: {X}')
                    current = self.board[posY1][i + X]
                    # print(f'Current: {current}, i: "{i}", [{X + i}, {posY1}]') # noqa E501
                    if current == '.':
                        current = '1'
                    elif current.isdigit():
                        current = str(int(current) + 1)

                    self.board[posY1][i + X] = current
            '''
            # self.getBigNumbers()
            prline = line.replace("\n", "")
            print(f'Current Value: {self.points} --- Line: {prline}')  # noqa

        # with open('Table.txt', 'w') as f:
        #     f.write(str(self.board))

    def getBigNumbers(self):
        self.points = 0
        for table in self.board:
            print(table)
            for value in table:
                if value != '.':
                    if int(value) >= 2:
                        self.points = self.points + 1


def main():
    c = Class()
    c.processInputs()
    c.getBigNumbers()
    print(f'Points: {c.points}')


if __name__ == '__main__':
    main()
