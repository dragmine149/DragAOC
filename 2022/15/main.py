from helper import helper, AOCException, Grid, math, Visual


class main():
    def __init__(self, h: helper) -> None:
        self.h = h
        self.data = self.h.main()
        self.grid = Grid()
        self.DecodeData()

    def DecodeData(self):
        for line in self.data:
            line: str
            sensor, beacon = line.split(":")
            sensor = sensor.removeprefix("Sensor at ")
            beacon = beacon.removeprefix(" closest beacon is at ")
            sensorLocation = sensor.split(",")
            beaconLocation = beacon.split(",")
            sensorLocation = {
                "X": int(sensorLocation[0][2:]),
                "Y": int(sensorLocation[1][3:])
            }
            beaconLocation = {
                "X": int(beaconLocation[0][2:]),
                "Y": int(beaconLocation[1][3:])
            }

            self.grid.GenerateTo(sensorLocation.get("X"),
                                 sensorLocation.get("Y"))
            self.grid.GenerateTo(beaconLocation.get("X"),
                                 beaconLocation.get("Y"))
            self.grid.SetValue(sensorLocation.get(
                "X"), sensorLocation.get("Y"), "S")
            self.grid.SetValue(beaconLocation.get(
                "X"), beaconLocation.get("Y"), "B")

    def vP(self):
        Visual.printColour(self.grid.grid, {
            "S": "\033[31m",
            "B": "\033[32m"
        })

    def main(self):
        self.vP()


if __name__ == "__main__":
    h = helper(15)
    m = main(h)
    m.main()
