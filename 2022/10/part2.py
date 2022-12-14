import helper
import os


class Visual:
    @staticmethod
    def Print(crtData):
        print(f'\033[33m{"-" * os.get_terminal_size().columns}\033[0m')
        for line in crtData:
            print(f'{"":3}', end='')
            for value in line:
                print(value, end='')
            print()


class main:
    def __init__(self, cpuInstructions) -> None:
        self.cpu = cpuInstructions
        self.tick = 0
        self.register = {
            "X": 1
        }

        self.pixelInedx = 0
        self.spriteLine = ['#', '#', '#']
        self.CRT = [
            [],
            [],
            [],
            [],
            [],
            []
        ]

    def DrawPixel(self):
        # print(self.tick // 40, self.pixelInedx)
        print(self.spriteLine)
        try:
            self.CRT[((self.tick - 1) // 40)].append(self.spriteLine[self.pixelInedx])
        except IndexError:
            self.CRT[((self.tick - 1) // 40)].append('.')
        Visual.Print(self.CRT)

        self.pixelInedx = (self.pixelInedx + 1) % 40

    def ProcessInstruction(self, instruction, value):
        print(instruction)
        match instruction:
            case "noop":
                self.tick += 1
                self.DrawPixel()
            case "addx":
                for _ in range(2):
                    self.tick += 1
                    self.DrawPixel()

                for i, _ in enumerate(self.spriteLine):
                    self.spriteLine[i] = '.'

                self.register["X"] += value

                try:
                    self.spriteLine[self.register["X"]] = '#'
                    self.spriteLine[self.register["X"] + 1] = '#'
                    self.spriteLine[self.register["X"] - 1] = '#'
                except IndexError:
                    print("IE")
                    for _ in range(self.register["X"] + 1):
                        self.spriteLine.append('.')

                    self.spriteLine[self.register["X"] + 1] = '#'
                    self.spriteLine[self.register["X"]] = '#'
                    self.spriteLine[self.register["X"] - 1] = '#'

    def Tick(self):
        for ins in self.cpu:
            ins = ins.strip()
            ins = ins.split(" ")
            v = 0
            if len(ins) >= 2:
                v = ins[1]

            self.ProcessInstruction(ins[0], int(v))

    def main(self):
        self.Tick()


if __name__ == "__main__":
    data = helper.main()
    m = main(data)
    m.main()
    helper.output(10)
