import time

st = time.time()

data = []
with open("data.txt", "r", encoding="utf-8") as f:
    data = f.readlines()


# A = Rock
# B = Paper
# C = Scissors

# X = Rock
# Y = Paper
# Z = Scissors


def ScoreCheck(p1, p2):
    if not p1 or not p2:
        raise ValueError(f"Value error between {p1} and {p2}")

    # Draw
    if p2 == 2:
        return p1 + 3

    if p2 == 1:
        # Force loose
        if p1 == 1:
            return 3 + 0
        if p1 == 2:
            return 1 + 0
        if p1 == 3:
            return 2 + 0

    if p2 == 3:
        if p1 == 1:
            return 2 + 6
        if p1 == 2:
            return 3 + 6
        if p1 == 3:
            return 1 + 6

    raise ValueError(f"Value error between {p1} and {p2}")


moveSplit = {
    "A": 1,  # Rock
    "B": 2,  # Paper
    "C": 3,  # Scissors
    "X": 1,  # loose
    "Y": 2,  # draw
    "Z": 3  # win
}

totalScore = 0
count = 0
for value in data:
    value = value.strip()
    moves = value.split(" ")
    score = ScoreCheck(moveSplit[moves[0]], moveSplit[moves[1]])
    totalScore += score
    count += 1
    print(
        f"Score: {score}, P1: {moveSplit[moves[0]]}, P2: {moveSplit[moves[1]]}, Total: {totalScore}")

print(f"Total RPS Score: {totalScore}, {count}")
print(f"Time: {time.time() - st}")
