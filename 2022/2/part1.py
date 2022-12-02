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
    if p1 == p2:
        return 3 + p2

    # P1 wins w/ rock
    if p1 == 1 and p2 == 3:
        return 0 + p2

    # P2 wins w/ rock
    if p2 == 1 and p1 == 3:
        return 6 + p2

    # P1 wins w/ paper
    if p1 == 2 and p2 == 1:
        return 0 + p2

    # P2 wins w/ paper
    if p2 == 2 and p1 == 1:
        return 6 + p2

    # p1 wins w/ scissors
    if p1 == 3 and p2 == 2:
        return 0 + p2

    # p2 wins w/ scissors
    if p2 == 3 and p1 == 2:
        return 6 + p2

    raise ValueError(f"Value error between {p1} and {p2}")


moveSplit = {
    "A": 1,  # Rock
    "B": 2,  # Paper
    "C": 3,  # Scissors
    "X": 1,  # Rock
    "Y": 2,  # Paper
    "Z": 3  # Scissors
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
