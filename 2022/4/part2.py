import time

st = time.time()

data = []
with open("data.txt", "r", encoding="utf-8") as f:
    data = f.readlines()

Complety = 0
index = 0
for item in data:
    item = item.strip()
    info = item.split(",")
    P1 = info[0].split("-")
    P2 = info[1].split("-")

    P1[0] = int(P1[0])
    P1[1] = int(P1[1])
    P2[0] = int(P2[0])
    P2[1] = int(P2[1])

    # import ipdb;ipdb.set_trace()

    if P1[0] < P2[0] and P1[1] < P2[0]:
        Complety += 1
        continue

    if P1[0] > P2[1]:
        Complety += 1
        continue

    index += 1


print("Day 4 ELF ISSUE RESULTS:")
print(f"Total fully contain: {Complety}")
print(f"Index (skipped): {index}")
print(f"TIME TAKEN: {time.time() - st}")
