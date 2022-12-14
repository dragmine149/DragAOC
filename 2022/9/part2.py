import os
import helper


class VisualRope:
    def __init__(self, show):
        self.show = show

    def Print(self, bridge, pos):
        if self.show:
            print(f"\033[32m{'-' * os.get_terminal_size().columns}\033[0m")
            for yI, yV in enumerate(bridge):
                for xI, xV in enumerate(yV):
                    for i in range(10):
                        if xI == pos[f'{i}']["X"] and yI == pos[f'{i}']["Y"]:
                            print(f"\033[33m{i}\033[0m", end="")
                            break
                    if yI == 0 and xI == 0:
                        print("\033[33ms\033[0m", end="")
                    else:
                        print(f'\033[33m{xV}\033[0m', end="")
                print()
            print(f"\033[32m{'-' * os.get_terminal_size().columns}\033[0m")


class Rope:
    def __init__(self, instructions, VR: VisualRope, show) -> None:
        # self.bridge = [[".", "."], [".", "."]]
        self.bridge = {}
        self.instructions = instructions
        self.Pos = {
            "0": {"X": 0, "Y": 0},
            "1": {"X": 0, "Y": 0},
            "2": {"X": 0, "Y": 0},
            "3": {"X": 0, "Y": 0},
            "4": {"X": 0, "Y": 0},
            "5": {"X": 0, "Y": 0},
            "6": {"X": 0, "Y": 0},
            "7": {"X": 0, "Y": 0},
            "8": {"X": 0, "Y": 0},
            "9": {"X": 0, "Y": 0},
        }
        self.VisualRope = VR
        self.show = show

    # def GenerateBridgeHeight(self):
    #     tmp = []
    #     for _ in self.bridge[0]:
    #         tmp.append(".")
    #     self.bridge.append(tmp)

    # def GenerateBridgeWidth(self):
    #     for br in self.bridge:
    #         br.append(".")

    def moveTail(self, index):
        # DIAG CHECK 1
        if self.Pos[f'{index + 1}']["X"] < self.Pos[f'{index}']["X"] - 1 and self.Pos[f'{index + 1}']["Y"] < self.Pos[f'{index}']["Y"] - 1:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"

            self.Pos[f'{index + 1}']["X"] += 1
            self.Pos[f'{index + 1}']["Y"] += 1
            return

        if self.Pos[f'{index + 1}']["X"] > self.Pos[f'{index}']["X"] + 1 and self.Pos[f'{index + 1}']["Y"] > self.Pos[f'{index}']["Y"] + 1:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"
            self.Pos[f'{index + 1}']["X"] -= 1
            self.Pos[f'{index + 1}']["Y"] -= 1
            return

        if self.Pos[f'{index + 1}']["X"] < self.Pos[f'{index}']["X"] - 1 and self.Pos[f'{index + 1}']["Y"] > self.Pos[f'{index}']["Y"] + 1:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"
            self.Pos[f'{index + 1}']["X"] += 1
            self.Pos[f'{index + 1}']["Y"] -= 1
            return

        if self.Pos[f'{index + 1}']["X"] > self.Pos[f'{index}']["X"] + 1 and self.Pos[f'{index + 1}']["Y"] < self.Pos[f'{index}']["Y"] - 1:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"
            self.Pos[f'{index + 1}']["X"] -= 1
            self.Pos[f'{index + 1}']["Y"] += 1
            return

        # END DIAG CHECK 1

        # DIAG CHECK 2
        if self.Pos[f'{index + 1}']["X"] < self.Pos[f'{index}']["X"] - 1 and self.Pos[f'{index + 1}']["Y"] < self.Pos[f'{index}']["Y"]:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"
            self.Pos[f'{index + 1}']["X"] += 1
            self.Pos[f'{index + 1}']["Y"] += 1
            return

        if self.Pos[f'{index + 1}']["X"] < self.Pos[f'{index}']["X"] and self.Pos[f'{index + 1}']["Y"] < self.Pos[f'{index}']["Y"] - 1:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"
            self.Pos[f'{index + 1}']["X"] += 1
            self.Pos[f'{index + 1}']["Y"] += 1
            return

        if self.Pos[f'{index + 1}']["X"] > self.Pos[f'{index}']["X"] + 1 and self.Pos[f'{index + 1}']["Y"] > self.Pos[f'{index}']["Y"]:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"
            self.Pos[f'{index + 1}']["X"] -= 1
            self.Pos[f'{index + 1}']["Y"] -= 1
            return

        if self.Pos[f'{index + 1}']["X"] > self.Pos[f'{index}']["X"] and self.Pos[f'{index + 1}']["Y"] > self.Pos[f'{index}']["Y"] + 1:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"
            self.Pos[f'{index + 1}']["X"] -= 1
            self.Pos[f'{index + 1}']["Y"] -= 1
            return

        if self.Pos[f'{index + 1}']["X"] < self.Pos[f'{index}']["X"] - 1 and self.Pos[f'{index + 1}']["Y"] > self.Pos[f'{index}']["Y"]:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"
            self.Pos[f'{index + 1}']["X"] += 1
            self.Pos[f'{index + 1}']["Y"] -= 1
            return

        if self.Pos[f'{index + 1}']["X"] < self.Pos[f'{index}']["X"] and self.Pos[f'{index + 1}']["Y"] > self.Pos[f'{index}']["Y"] + 1:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"
            self.Pos[f'{index + 1}']["X"] += 1
            self.Pos[f'{index + 1}']["Y"] -= 1
            return

        if self.Pos[f'{index + 1}']["X"] > self.Pos[f'{index}']["X"] + 1 and self.Pos[f'{index + 1}']["Y"] < self.Pos[f'{index}']["Y"]:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"
            self.Pos[f'{index + 1}']["X"] -= 1
            self.Pos[f'{index + 1}']["Y"] += 1
            return

        if self.Pos[f'{index + 1}']["X"] > self.Pos[f'{index}']["X"] and self.Pos[f'{index + 1}']["Y"] < self.Pos[f'{index}']["Y"] - 1:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"
            self.Pos[f'{index + 1}']["X"] -= 1
            self.Pos[f'{index + 1}']["Y"] += 1
            return

        # END DIAG CHECK 2

        # NORMAL CHECK 1
        if self.Pos[f'{index + 1}']["X"] < self.Pos[f'{index}']["X"] - 1:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"
            self.Pos[f'{index + 1}']["X"] += 1
            return

        if self.Pos[f'{index + 1}']["X"] > self.Pos[f'{index}']["X"] + 1:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"
            self.Pos[f'{index + 1}']["X"] -= 1
            return

        if self.Pos[f'{index + 1}']["Y"] < self.Pos[f'{index}']["Y"] - 1:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"
            self.Pos[f'{index + 1}']["Y"] += 1
            return

        if self.Pos[f'{index + 1}']["Y"] > self.Pos[f'{index}']["Y"] + 1:
            if index == 8:
                self.bridge[f"({self.Pos['9']['X']}, {self.Pos['9']['Y']})"] = "#"
            self.Pos[f'{index + 1}']["Y"] -= 1
            return

        # END NORMAL CHECK 1

    def MovePiece(self, direction: str, amount: int):
        self.Pos["0"][direction] += amount

        for i in range(9):
            self.moveTail(i)

    def TranslateInstruction(self, instruction: str):
        direction, amount = instruction.split(" ")
        if self.show:
            print(f"Moving {direction} by {amount}")

        for i in range(int(amount)):
            if self.show:
                print(f"{i + 1}/{amount}")
            match direction:
                case "R":
                    self.MovePiece("X", 1)
                case "L":
                    self.MovePiece("X", -1)

                # up is down, down is up
                case "U":
                    self.MovePiece("Y", 1)
                case "D":
                    self.MovePiece("Y", -1)

            self.VisualRope.Print(self.bridge, self.Pos)

    def main(self):
        self.VisualRope.Print(self.bridge, self.Pos)
        for item in self.instructions:
            self.TranslateInstruction(item)


if __name__ == "__main__":
    data, v = helper.main()
    rope = Rope(data, VisualRope(v), False)

    rope.main()
    rope.bridge[f"({rope.Pos['9']['X']}, {rope.Pos['9']['Y']})"] = "#"

    rope.VisualRope.Print(rope.bridge, rope.Pos)

    visitCount = len(rope.bridge)

    helper.output(9)
    print(f"The tail visted squares (at least onces): {visitCount} times")
