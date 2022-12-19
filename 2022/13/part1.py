import ast
import ipdb
from helper import helper, AOCException


class main:
    def __init__(self, data, h: helper) -> None:
        self.h = h
        self.data = self.DecodeData(data)

    def DecodeData(self, data):
        packetList = []
        for info in range(0, len(data), 3):
            packetInfo = [ast.literal_eval(
                data[info]), ast.literal_eval(data[info + 1])]
            packetList.append(packetInfo)

        return packetList

    def compare(self, data):
        left = data[0]
        right = data[1]

        # List ran out of items check
        if not isinstance(left, int) and not isinstance(right, int):
            if len(left) == 0 and len(right) == 0:
                return 0

            if len(left) == 0 and len(right) > 0:
                return 1

            if len(right) == 0 and len(left) > 0:
                return -1

        for index, item in enumerate(left):
            # Compare if both items are a list, and go deeper
            try:
                if isinstance(item, list) and isinstance(right[index], list):
                    v = self.compare([item, right[index]])
                    if v == 0:
                        continue
                    return v
            except IndexError:
                return -1

            # If one item is a list, make the other a list and go deeper
            if isinstance(item, list):
                v = self.compare([item, [right[index]]])
                if v == 0:
                    continue
                return v

            try:
                # ^^
                if isinstance(right[index], list):
                    v = self.compare([[item], right[index]])
                    if v == 0:
                        continue
                    return v
            except IndexError:
                # Inputs are not in right order due to out of range
                return -1

            # If both values are the same integer, continue.
            if right[index] == item:
                continue

            # If right is greater than left, they are in the right order
            if right[index] > item:
                return 1

            # If left is greater than right, they are in the wrong order
            if item > right[index]:
                return -1

            # Checks if lists are the same, but no comparisions are made
            if len(left) == len(right):
                continue

        if len(left) < len(right):
            return 1

        if len(right) < len(left):
            return -1

        if len(left) == len(right):
            return 0

        print(data)
        raise AOCException("Okay, something should have returned by now")

    def main(self):
        orderSum = 0
        for index, item in enumerate(self.data):
            order = self.compare(item)
            if order == 1:
                orderSum += index + 1

        return orderSum


if __name__ == "__main__":
    h = helper(13)
    helperData = h.main()
    m = main(helperData, h)
    result = m.main()
    h.output()
    print(f"Sum of incorect items: {result}")
