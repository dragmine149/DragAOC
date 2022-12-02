import time
import copy
st = time.time()

data = []
with open("data.txt", "r", encoding="utf-8") as f:
    data = f.readlines()

elfs = []
elfIndex = 0

for line in data:
    line = line.strip()
    if line in ('\n', ''):
        elfIndex += 1
        continue

    try:
        elfs[elfIndex] = elfs[elfIndex] + int(line)
    except IndexError:
        elfs.append(int(line))


elfCount = [[0, 0], [0, 0], [0, 0]]
for index, elf in enumerate(elfs):
    # print(elf, elfCount[0][1])
    # if elf > elfCount[0][1]:
    #     elfCount[2] = copy.deepcopy(elfCount[1])
    #     elfCount[1] = copy.deepcopy(elfCount[0])
    #     elfCount[0][1] = elf
    #     elfCount[0][0] = index
    #     print(elfCount)

    print(elf)

    if elf > elfCount[0][1]:
        elfCount[2] = elfCount[1]
        elfCount[1] = elfCount[0]
        elfCount[0] = [index, elf]
        continue

    if elf > elfCount[1][1]:
        elfCount[2] = elfCount[1]
        elfCount[1] = [index, elf]
        continue

    if elf > elfCount[2][1]:
        elfCount[2] = [index, elf]
        continue


ed = time.time()
print("Top 3 elfs and callories:")
print(f"1: Index -- {elfCount[0][0]}  Count -- {elfCount[0][1]}")
print(f"2: Index -- {elfCount[1][0]}  Count -- {elfCount[1][1]}")
print(f"3: Index -- {elfCount[2][0]}  Count -- {elfCount[2][1]}")
print(f"Total Calories: {elfCount[0][1] + elfCount[1][1] + elfCount[2][1]}")
print(f"Total Time: {ed - st}")
