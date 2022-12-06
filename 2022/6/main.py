import helper

Limit, data = helper.main()


def checkList(chkList: list):
    for item in chkList:
        if chkList.count(item) > 1 or item == "":
            return False

    return True


data: str = data[0].strip()

chrList = []

for i in range(Limit):
    chrList.append("")

unique4 = 0
for index, value in enumerate(data):
    chrList[index % Limit] = value

    if checkList(chrList):
        unique4 = index + 1
        break

helper.output(6)
print(f"Unique 4 char: {unique4}")
