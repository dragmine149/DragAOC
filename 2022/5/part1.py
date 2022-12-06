import helper
import time

st = time.time()

data = helper.main()

# instructions = lines after \n
# Starting = lines before \n

startingInfo = ""
instructions = []

# get instructions and crates order
instr = False
for line in data:
    if line == "\n":
        instr = True
        continue

    if not instr:
        startingInfo += line
    else:
        instructions.append(line)

# sort order out into table
order = []

start = startingInfo.split("\n")
start.reverse()
start.pop(0)
start.pop(0)

# Get all the info from the crates
for line in start:
    for index, info in enumerate(line):
        try:
            if info not in (" ", "[", "]"):
                order[index // 4].append(info)
        except IndexError:
            order.append([])
            order[index // 4].append(info)

# debug instructions

for info in instructions:
    info:str
    infoInst = info.split(" ")
    # 0 = moves
    # 1 = ammount
    # 2 = from
    # 3 = index
    # 4 = to
    # 5 = index
    for i in range(int(infoInst[1])):
        v = order[int(infoInst[3]) - 1]
        mvValue = v.pop()
        order[int(infoInst[5]) - 1].append(mvValue)

msg = ""
for crate in order:
    msg += crate[len(crate) - 1]

print("Day 5 elf Result:")
print(f"Message: {msg}")
print(f"Time: {time.time() - st}")
