import time

st = time.time()

data = []
with open("data.txt", "r", encoding="utf-8") as f:
    data = f.readlines()

# Thanks to Guy_732
# changes letter to number based in the alphabet


def decode(s: str) -> int:
    s = s.lower()
    ref = ord('a') - 1
    v = 0
    exp = 1
    for c in reversed(s):
        v += (ord(c) - ref) * exp
        exp *= 26

    return v


piorityTotal = 0
group = []

for index, line in enumerate(data):
    group.append(line)

    if (index + 1) % 3 == 0:
        for item in group[0]:
            if item in group[1] and item in group[2]:
                cost = decode(item)
                if item.isupper():
                    cost += 26

                piorityTotal += cost
                group.clear()
                break

        if len(group) > 0:
            raise ValueError("Not everything got included!")


print("DAY 3 ELF ISSUE RESULTS:")
print(f"TOTAL SUM: {piorityTotal}")
