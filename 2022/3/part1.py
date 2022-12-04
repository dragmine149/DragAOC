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

for line in data:
    line = line.strip()

    # split line and get different areas
    lineLength = len(line)
    compartment = round(lineLength / 2)
    compartmentA = line[0:compartment]
    compartmentB = line[compartment:]

    # get similar letters
    for letter in compartmentA:
        if letter in compartmentB:
            index = decode(letter)
            if letter.isupper():
                index += 26

            piorityTotal += index
            break

print("DAY 3 ELF ISSUE RESULTS:")
print(f"TOTAL SUM: {piorityTotal}")
