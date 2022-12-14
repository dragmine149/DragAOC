import helper


class main:
    def __init__(self, cpuInstructions) -> None:
        self.cpu = cpuInstructions
        self.tick = 0
        self.register = {
            "X": 1
        }
        self.strength = {

        }

    def AddStrength(self):
        if (self.tick + 20) % 40 == 0:
            self.strength[self.tick] = self.register["X"] * self.tick

    def ProcessInstruction(self, instruction, value):
        match instruction:
            case "noop":
                self.tick += 1
                self.AddStrength()
            case "addx":
                for _ in range(2):
                    self.tick += 1
                    self.AddStrength()
                self.register["X"] += int(value)

    def Tick(self):
        for ins in self.cpu:
            ins = ins.strip()
            ins = ins.split(" ")
            v = 0
            if len(ins) >= 2:
                v = ins[1]

            self.ProcessInstruction(ins[0], v)

    def GetSumOfStrength(self):
        return sum(self.strength.values())

    def main(self):
        self.Tick()


if __name__ == "__main__":
    data = helper.main()
    m = main(data)
    m.main()
    helper.output(10)
    print(f"Strength: {m.strength}")
    print(f"Sum of strength: {m.GetSumOfStrength()}")
