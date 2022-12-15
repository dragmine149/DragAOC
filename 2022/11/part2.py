import ipdb
import time
from dataclasses import dataclass
from helper import helper

class Data:
    def __init__(self) -> None:
        self.monkeys = []

    def DecodeData(self, mky: str):
        mkInfo = mky.split("\n")

        # monkey number
        mkNumber = int(mkInfo[0].split(" ")[1].strip().removesuffix(":"))

        # monkey items
        mkItems = mkInfo[1].split(":")[1].strip().split(", ")
        for index, item in enumerate(mkItems):
            mkItems[index] = int(item)

        # monkey operation
        mkOper = mkInfo[2].split(": ")[1].strip()

        # monkey test
        mkTest = mkInfo[3].split(": ")[1].strip()

        # monkey Conditions
        mkCondition = {
            "True": int(mkInfo[4].strip().split(" ")[5]),
            "False": int(mkInfo[5].strip().split(" ")[5])
        }

        self.monkeys.append(
            Monkey(mkNumber, mkItems, mkOper, mkTest, mkCondition))


@dataclass
class Monkey:
    Number: int
    Items: list
    Operation: str
    Test: str
    Condition: dict
    Inspected: int = 0

    def __Calculation(self, mode, original, value):
        """Return the result of the operation

        Args:
            mode (str): The operater to do
            original (int): The original valu
            value (int): The value to add on (or times)

        Returns:
            int: The new value
        """
        # Catching for old mode old (e.g. old * old)
        try:
            value = int(value)
        except ValueError:
            value = original

        # Decoding mode
        match mode:
            case "*":
                return original * value
            case "+":
                return original + value

        # Errors
        info = {
            "mode": mode,
            "original": original,
            "value": value
        }
        print(
            f"\033[31mError in calculating monkey test\tMonkey: {self.Number}. Info: {info}\033[0m")

    def Calculate(self, old):
        """Split up the operater and calculate it

        Args:
            old (int): The current value

        Returns:
            int: The new value
        """
        paramaters = self.Operation.split(" ")
        paramaters = paramaters[-2:]
        self.Inspected += 1
        return self.__Calculation(paramaters[0], old, paramaters[1])

    def test(self, value):
        testValue = self.Test.split(" ")[-1:][0]
        if value % int(testValue) == 0:
            return self.Condition.get("True")
        return self.Condition.get("False")

    def testDivisible(self):
        return int(self.Test.split(" ")[-1:][0])


class main:
    def __init__(self, monkeys: list[Monkey], helper: helper, printer) -> None:
        self.monkeys = monkeys
        self.helper = helper
        self.printer = printer

    def __getMonkeyDiv(self):
        div = 1
        for monkey in self.monkeys:
            div *= monkey.testDivisible()

        return div

    def __Test(self, monkeyIndex: int):
        mk = self.monkeys[monkeyIndex]

        # Turn ends if holding nothing
        if len(mk.Items) == 0:
            return

        # Inspect item
        for index, _ in enumerate(mk.Items):
            mk.Items[index] = mk.Calculate(mk.Items[index])

            throwTO = mk.test(mk.Items[index])
            self.monkeys[throwTO].Items.append(mk.Items[index])

        mk.Items = []

    def __round(self):
        for monkey in range(len(self.monkeys)):
            self.__Test(monkey)

        for monkey in self.monkeys:
            for index, _ in enumerate(monkey.Items):
                # Thanks redit
                d, m = divmod(monkey.Items[index], self.__getMonkeyDiv())
                # if m == 0:
                #     monkey.Items[index] = d
                monkey.Items[index] = m

    def __Finish(self):
        inpsected = []
        for monkey in self.monkeys:
            inpsected.append((monkey.Number, monkey.Inspected))

        inpsected.sort(key=lambda i: i[1])
        inpsected.reverse()
        return inpsected

    def main(self):
        for i in range(10000):
            rst = time.time()
            if self.printer:
                self.helper.print(f"Round {i}")
                print(self.monkeys)

            self.__round()

            if self.printer:
                self.helper.print(f"End Round {i}")
                print(self.monkeys)
            
            print(f"== {i} ({time.time() - rst}) ==")

            if (i + 1) % 1000 == 0:
                print(f"== After round {i} (Time: {self.helper.time()})==")
                for monk in self.monkeys:
                    print(
                        f"Monkey {monk.Number} inspected items {monk.Inspected} times.")

            if i == 19:
                print("== After round 20 ==")
                for monk in self.monkeys:
                    print(
                        f"Monkey {monk.Number} inspected items {monk.Inspected} times.")

        return self.__Finish()


if __name__ == "__main__":
    hpler = helper(11)
    monkStoredData, printer = hpler.main()
    monkData = Data()
    for i in monkStoredData.split("\n\n"):
        monkData.DecodeData(i)

    m = main(monkData.monkeys, hpler, printer)
    data = m.main()
    hpler.output()
    for monk in data:
        print(f"Monkey {monk[0]} inspected items {monk[1]} times.")

    print(f"Total: {data[0][1] * data[1][1]}")
