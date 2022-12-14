import os
import helper


class VisualRope:
    def __init__(self, show):
        self.show = show

    def Print(self, bridge, head, tail):
        if self.show:
            print(f"\033[32m{'-' * os.get_terminal_size().columns}\033[0m")
            for yI, yV in enumerate(bridge):
                for xI, xV in enumerate(yV):
                    if xI == head["X"] and yI == head["Y"]:
                        print("\033[33mH\033[0m", end="")
                    elif xI == tail["X"] and yI == tail["Y"]:
                        print("\033[33mT\033[0m", end="")
                    elif yI == 0 and xI == 0:
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
        self.HeadPos = {"X": 0, "Y": 0}
        self.TailPos = {"X": 0, "Y": 0}
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

    def moveTail(self):
        # DIAG CHECK 1
        if self.TailPos["X"] < self.HeadPos["X"] - 1 and self.TailPos["Y"] < self.HeadPos["Y"] - 1:
            # self.bridge[self.TailPos["Y"]][self.TailPos["X"]] = "#"
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["X"] += 1
            self.TailPos["Y"] += 1
            return

        if self.TailPos["X"] > self.HeadPos["X"] + 1 and self.TailPos["Y"] > self.HeadPos["Y"] + 1:
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["X"] -= 1
            self.TailPos["Y"] -= 1
            return

        if self.TailPos["X"] < self.HeadPos["X"] - 1 and self.TailPos["Y"] > self.HeadPos["Y"] + 1:
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["X"] += 1
            self.TailPos["Y"] -= 1
            return

        if self.TailPos["X"] > self.HeadPos["X"] + 1 and self.TailPos["Y"] < self.HeadPos["Y"] - 1:
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["X"] -= 1
            self.TailPos["Y"] += 1
            return

        # END DIAG CHECK 1

        # DIAG CHECK 2
        if self.TailPos["X"] < self.HeadPos["X"] - 1 and self.TailPos["Y"] < self.HeadPos["Y"]:
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["X"] += 1
            self.TailPos["Y"] += 1
            return

        if self.TailPos["X"] < self.HeadPos["X"] and self.TailPos["Y"] < self.HeadPos["Y"] - 1:
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["X"] += 1
            self.TailPos["Y"] += 1
            return

        if self.TailPos["X"] > self.HeadPos["X"] + 1 and self.TailPos["Y"] > self.HeadPos["Y"]:
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["X"] -= 1
            self.TailPos["Y"] -= 1
            return

        if self.TailPos["X"] > self.HeadPos["X"] and self.TailPos["Y"] > self.HeadPos["Y"] + 1:
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["X"] -= 1
            self.TailPos["Y"] -= 1
            return

        if self.TailPos["X"] < self.HeadPos["X"] - 1 and self.TailPos["Y"] > self.HeadPos["Y"]:
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["X"] += 1
            self.TailPos["Y"] -= 1
            return

        if self.TailPos["X"] < self.HeadPos["X"] and self.TailPos["Y"] > self.HeadPos["Y"] + 1:
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["X"] += 1
            self.TailPos["Y"] -= 1
            return

        if self.TailPos["X"] > self.HeadPos["X"] + 1 and self.TailPos["Y"] < self.HeadPos["Y"]:
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["X"] -= 1
            self.TailPos["Y"] += 1
            return

        if self.TailPos["X"] > self.HeadPos["X"] and self.TailPos["Y"] < self.HeadPos["Y"] - 1:
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["X"] -= 1
            self.TailPos["Y"] += 1
            return

        # END DIAG CHECK 2

        # NORMAL CHECK 1
        if self.TailPos["X"] < self.HeadPos["X"] - 1:
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["X"] += 1
            return

        if self.TailPos["X"] > self.HeadPos["X"] + 1:
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["X"] -= 1
            return

        if self.TailPos["Y"] < self.HeadPos["Y"] - 1:
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["Y"] += 1
            return

        if self.TailPos["Y"] > self.HeadPos["Y"] + 1:
            self.bridge[f"({self.TailPos['X']}, {self.TailPos['Y']})"] = "#"
            self.TailPos["Y"] -= 1
            return

        # END NORMAL CHECK 1

    def MovePiece(self, direction: str, amount: int):
        # if self.HeadPos[direction] + amount + 1 > len(self.bridge) or self.HeadPos[direction] + amount + 1 > len(self.bridge[0]):
        #     self.GenerateBridgeHeight()
        #     self.GenerateBridgeWidth()
        self.HeadPos[direction] += amount
        self.moveTail()

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

            self.VisualRope.Print(self.bridge, self.HeadPos, self.TailPos)

    def main(self):
        self.VisualRope.Print(self.bridge, self.HeadPos, self.TailPos)
        for item in self.instructions:
            self.TranslateInstruction(item)


if __name__ == "__main__":
    data, v = helper.main()
    rope = Rope(data, VisualRope(v), False)

    rope.main()
    rope.bridge[f"({rope.TailPos['X']}, {rope.TailPos['Y']})"] = "#"

    rope.VisualRope.Print(rope.bridge, {"X": 100, "Y": 100}, {"X": 100, "Y": 100})

    visitCount = len(rope.bridge)

    helper.output(9)
    print(f"The tail visted squares (at least onces): {visitCount} times")
