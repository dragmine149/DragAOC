import file
import time


class Class():
    def __init__(self):
        self.Debug = False
        self.lines = file.get()[0].split(',')
        if self.Debug:
            self.lines = file.get2()[0].split(',')
        self.posToMove = 0
        self.Fuel = 0
        self.BestFuel = 0

    def CalculateFuel(self):
        for value in self.lines:

            if (int(value) < self.posToMove):  # below desired position
                PosMoved = (self.posToMove - int(value))
                FuelUsed = (PosMoved * (PosMoved + 1)) / 2
                self.Fuel = self.Fuel + FuelUsed  # goinup

            if (int(value) > self.posToMove):
                PosMoved = (int(value) - self.posToMove)
                FuelUsed = (PosMoved * (PosMoved + 1)) / 2
                self.Fuel = self.Fuel + FuelUsed  # goinDo

    def setUp(self, pos):
        self.Fuel = 0
        self.posToMove = pos

    def FindLowest(self):
        print(f'Low: 0, High: {int(max(self.lines))}')
        for i in range(int(max(self.lines))):
            self.setUp(i)
            self.CalculateFuel()
            print(f'Recent: {self.Fuel}, CurrentLowest: {self.BestFuel}')
            if self.BestFuel == 0:
                self.BestFuel = self.Fuel
            if self.Fuel < self.BestFuel:  # want lower value
                self.BestFuel = self.Fuel


if __name__ == '__main__':
    b = time.time()
    c = Class()

    # c.CalculateFuel()
    # c.BestFuel = c.Fuel
    c.FindLowest()
    e = time.time()
    print(f'Fuel Required: {c.BestFuel}')
    print(f'Time taken: {e - b}')
