import ast
import copy
import ipdb
from helper import helper, AOCException


class main:
    def __init__(self, data, h: helper) -> None:
        self.h = h
        self.data = self.DecodeData(data)

    def DecodeData(self, data):
        packetList = []
        for info in data:
            info = info.strip()
            if info == "":
                continue
            packetList.append(ast.literal_eval(info))
        return packetList

    def compare(self, data):
        left = data[0]
        right = data[1]
        # print(f"COMPARING: {left} with {right}")

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

    def sortList(self, sortedData):
        for itemIndex in range(0, len(sortedData), 1):

            if itemIndex + 1 >= len(sortedData):
                v = self.compare(
                    [sortedData[itemIndex - 1], sortedData[itemIndex]])
                if v == -1:
                    tmp = copy.copy(sortedData[itemIndex - 1])
                    sortedData[itemIndex -
                               1] = copy.copy(sortedData[itemIndex])
                    sortedData[itemIndex] = tmp
                break

            v = self.compare(
                [sortedData[itemIndex], sortedData[itemIndex + 1]])
            if v == -1:  # wrong order
                tmp = copy.copy(sortedData[itemIndex])
                sortedData[itemIndex] = copy.copy(sortedData[itemIndex + 1])
                sortedData[itemIndex + 1] = tmp

        return sortedData

    def AppendItem(self, data, sortedData: list):
        # print(f"Appending data: {data}")
        for index, item in enumerate(sortedData):
            v = self.compare([data, item])
            if v == 1:
                continue
            if v == -1:
                sortedData.insert(index, data)
                break

        # If not appened
        try:
            sortedData.index(data)
        except ValueError:
            sortedData.append(data)

        if len(sortedData) == 0:
            sortedData.append(data)

        return sortedData

    def sort(self, data):
        sortedData = []

        for itemIndex in range(0, len(data), 1):
            item = data[itemIndex]
            # print(f'ITEM: {item}')
            if len(item) == 0:
                # If blank, add at start
                sortedData.insert(0, item)

            # ipdb.set_trace()
            sortedData = self.AppendItem(item, sortedData)
            sortedData = self.sortList(sortedData)

        # Just finishing up
        sortedData = self.AppendItem([[2]], sortedData)
        print("END SORTING")
        sortedData = self.sortList(sortedData)

        sortedData = self.AppendItem([[6]], sortedData)
        print("END SORTING")
        sortedData = self.sortList(sortedData)

        return sortedData

    def FindPacketIndex(self, data: list):
        try:
            a = data.index([[2]]) + 1
            b = data.index([[6]]) + 1
            print(a, b)
            return a * b
        except ValueError:
            AOCException("Okay, how?")

if __name__ == "__main__":
    h = helper(13)
    helperData = h.main()
    m = main(helperData, h)
    result = m.sort(m.data)
    packetResult = m.FindPacketIndex(result)
    h.output()
    print(f"Result: {packetResult}")
