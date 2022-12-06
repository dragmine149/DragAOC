import time
import helper

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
        instructions.append(line.strip())

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


class visualisation:
    def __init__(self) -> None:
        pass

    def Update(self, data: list):
        for index, item in enumerate(data):
            print(f"{index}: ({len(item)}) [{item}] ")

v = visualisation()
v.Update(order)
input()

# debug instructions
for info in instructions:
    print("\x1b[2J\x1b[H", end="")
    print(info)
    info: str
    infoInst = info.split(" ")
    # 0 = moves
    # 1 = ammount
    # 2 = from
    # 3 = index
    # 4 = to
    # 5 = index
    ammount = int(infoInst[1])
    frIndex = int(infoInst[3]) - 1
    toIndex = int(infoInst[5]) - 1

    items = order[frIndex][-ammount:]
    order[toIndex].extend(items)

    del order[frIndex][-ammount:]

    v.Update(order)
    print(f"Removing {items} (len of {len(items)}) from {frIndex}")
    # input()
    time.sleep(0.05)


print(order)

msg = ""
for crate in order:
    msg += crate[len(crate) - 1]

print("Day 5 elf Result:")
print(f"Message: {msg}")
print(f"Time: {time.time() - st}")
