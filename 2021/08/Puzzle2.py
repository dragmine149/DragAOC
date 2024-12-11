import file
import run


class Class():
    def __init__(self):
        self.input = file.get2()
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
        self.Entry = []

    def most_common_character(self, input_str):
        input_str = input_str.lower()
        new_string = "".join(input_str.split())
        higher_count = 0
        return_character = ""

        for i in range(0, len(new_string)):
            count = 0
            length = len(new_string)
            j = 0
            character = new_string[i]
            while length > 0:
                if (character == new_string[j]):
                    count += 1
                j += 1
                length -= 1
                if (higher_count <= count):
                    higher_count = count
                    return_character = character
        return (return_character)

    def decode(self):
        # for line in self.input:
        line = self.input[0]
        newLine = line.split('|')

        # 0 = a, 1 = b, 2 = c, 3 = d, 4 = e, 5 = f, 6 = g
        values = ['', '', '', '', '', '', '']
        complete = False

        six = 0
        splitvar = newLine[0].split(' ')
        total = []
        # while not complete:
        for i in range(10):
            for value in range(len(splitvar)):

                if len(splitvar[value]) == 2:
                    values[2] = splitvar[value][0]
                    values[5] = splitvar[value][1]
                    newLine[0].replace(splitvar[value], '')

                if len(splitvar[value]) == 3:
                    tempvar = splitvar[value]
                    for v in values:
                        if v in tempvar:
                            tempvar = tempvar.replace(v, '')
                    if len(tempvar) == 1:
                        values[0] = tempvar

                if len(splitvar[value]) == 6 and six < 3:
                    tempvar = splitvar[value]
                    for v in values:
                        if v in tempvar:
                            tempvar = tempvar.replace(v, '')
                    total.append(tempvar)
                    if len(total) == 3:
                        print(total)
                        v1 = ''
                        v2 = ''
                        v3 = ''
                        # for value in total:
                        #     print(value)
                    # if len(tempvar) <= 4:
                    #     values[1] = values[1] + tempvar
                    #     values[3] = values[3] + tempvar
                    #     values[6] = values[6] + tempvar
                    #     six = six + 1

                # if six == 3:
                    # values[1] == cef
                    # values[3] == fe
                    # values[6] == ce

                    # common = self.most_common_character(values[1])
                    # values[1] = common
                    # values[3] = values[3].replace(common, '')
                    # values[6] = values[6].replace(common, '')

                    # least = values[3]
                    # for v in values[6]:
                    #     least = least.replace(v, '')
                    # values[3] = least

        #     bad = False
        #     for value in values:
        #         if value == '':
        #             bad = True
        #             break
        #         if len(value) != 1:
        #             bad = True
        #             break
        #     if not bad:
        #         complete = True
        print(values)



    def calc(self):
        for e in self.Entry:
            self.total = self.total + e


if __name__ == '__main__':
    c = Class()
    c.decode()
    print(f'Total: {c.total}')
    print(run.End())
