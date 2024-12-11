import sys
sys.path.append('../')
import file  # noqa E402

'''
life support = oxygen gen rat * CO2 scrub rat

full list, conside first bit
keep that meat bit criteria
discard that don't
only one left, stop (rating)
repeat process with next bit to right

BIT CRITERIA
- oxygen gen = most common value
    - start with all, get first bit
    - if more values with 1, keep 1. if 0 keep 0, if both, keep 1
    - repeat with other digits. (don't worry about previous values)

- CO2 scrub = least common value
    - Same as oxygen, just reverse

Finally, do life support. (denary answer)
'''


class Class():
    def __init__(self, lines):
        self.value = ''
        self.z = 0
        self.o = 0
        self.pos = 0
        self.lines = lines

    def sortBits(self, value):
        if value == '0':
            self.z = self.z + 1
        elif value == '1':
            self.o = self.o + 1
        else:
            return 'error'

    def removeBadOxygen(self):
        #     print(self.o, self.z)
        #     print(f'self.o > self.z: {self.o > self.z}')  # noqa
        #     print(f'self.o == self.z: {self.o == self.z}')
        #     print(f'self.o < self.z: {self.o < self.z}')
        #     for line in self.lines:
        #         if self.o > self.z or self.o == self.z:
        #             if line[self.pos] == '0':
        #                 self.lines.remove(line)
        #         if self.o < self.z:
        #             if line[self.pos] == '1':
        #                 self.lines.remove(line)
        #     # reset for next char
        #     self.z = 0
        #     self.o = 0
        print(self.z, self.o)
        newArray = []

        if self.z > self.o:
            for line in self.lines:
                if line[self.pos] == '0':
                    newArray.append(line)
        elif self.o > self.z or self.o == self.z:
            for line in self.lines:
                if line[self.pos] == '1':
                    newArray.append(line)

        self.lines = newArray

        self.z = 0
        self.o = 0

    def removeBadCO2(self):
        print(self.z, self.o)
        newArray = []

        if self.z < self.o or self.z == self.o:
            for line in self.lines:
                if line[self.pos] == '0':
                    newArray.append(line)
        elif self.o < self.z:
            for line in self.lines:
                if line[self.pos] == '1':
                    newArray.append(line)

        self.lines = newArray

        self.z = 0
        self.o = 0


if __name__ == '__main__':
    Lines = file.get()

    # oxygen
    c = Class(Lines)
    for i in range(12):
        c.pos = i
        for e in c.lines:
            c.sortBits(e[c.pos])
        c.removeBadOxygen()
    new = c.lines[0].replace('\n', '')
    print(new)
    print(file.den(new))

    # co2
    c2 = Class(Lines)
    for i in range(12):
        if len(c2.lines) == 1:
            break
        c2.pos = i
        for e in c2.lines:
            c2.sortBits(e[c2.pos])
        c2.removeBadCO2()
    new2 = c2.lines[0].replace('\n', '')
    print(new2)
    print(file.den(new2))

    answer = file.den(new) * file.den(new2)
    print(f'answer: {answer}')
