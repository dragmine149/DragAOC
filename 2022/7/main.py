import os
import ipdb
import helper

class Visulasation:
    def __init__(self) -> None:
        self.indent = 0

    def main(self, data):
        if not vs:
            return

        print("\x1b[2J\x1b[H", end="")
        self.Show(data)
        self.indent = 0
        # helper.time.sleep(0.25)

    def Show(self, data: dict):
        for item in data:
            if not isinstance(data.get(item), dict):
                print('\t' * self.indent,
                      f" - {item} (file, size={data.get(item)})")
            else:
                print('\t' * self.indent, f" - {item} (dir)")

            if isinstance(data, dict):
                value = data.get(item)
                if value is not None and not isinstance(value, str):
                    self.indent += 1
                    self.Show(value)

        self.indent -= 1


class CommandProcessor:
    def __init__(self) -> None:
        self.files = {}
        self.currentPath = ""
        self.listing = False

        self.V = Visulasation()

    def ChangeDirectory(self, path: str):
        if path == "..":
            pathInfo = os.path.split(self.currentPath)
            self.currentPath = pathInfo[0]
            return

        if self.currentPath != "/" and path != "/":
            self.currentPath += "/"

        self.currentPath += f"{path}"

    def __AddToList(self, parent, data):
        dataInfo = data.split(" ")
        if dataInfo[0] == "dir":
            parent[dataInfo[1]] = {}
            return parent
        parent[dataInfo[1]] = dataInfo[0]
        return parent

    def ListDirectory(self, data: str):
        if self.currentPath == "/":
            self.files = self.__AddToList(self.files, data)
            return

        subDirs = self.currentPath.split("/")
        for item in subDirs:
            if item == "":
                subDirs.remove(item)

        path = self.files.get(subDirs[0])
        for file in subDirs[1:]:
            path = path.get(file)
        path = self.__AddToList(path, data)

        self.V.main(self.files)

    def process(self, data: str):
        data = data.strip()
        result = data.split(" ")  # data contains either 2 or 3 sections

        if result[0] == "$":  # Signifys command
            self.listing = False
            if result[1] == "cd":
                self.ChangeDirectory(result[2])
                return

            if result[1] == "ls":
                self.listing = True
                return

        if self.listing:
            self.ListDirectory(data)
            return

# def GetDirectorySize(directory: dict):
#     size = 0
#     for item in directory:
#         tpe = directory.get(item)
#         if isinstance(tpe, dict):
#             dirSize = GetDirectorySize(tpe)
#             sizes[item] = dirSize
#             size += dirSize
#             continue

#         # sizes[item] = int(tpe)
#         size += int(tpe)

#     return size

def GetDirectorySize(directory: dict) -> int:
    dirSize = 0
    for item in directory:
        iType = directory.get(item)
        if isinstance(iType, dict): ## If another directory
            sizes[item] = GetDirectorySize(iType)
            dirSize += sizes[item]
            continue

        dirSize += int(iType)

    return dirSize

if __name__ == "__main__":
    fdata, vs = helper.main()
    cp = CommandProcessor()
    [cp.process(line) for line in fdata]

    sizes = {}
    rootSize = GetDirectorySize(cp.files)

    SumTotalSizes = 0
    TotalSizes = {}
    for dirInfo in sizes:
        size = sizes.get(dirInfo)
        if size <= 100_000:
        # if size == 100_000:
        # if size >= 100_000:
        # if True:
            TotalSizes[dirInfo] = size
            SumTotalSizes += size

    helper.output(7)
    # print(f"Directories + sizes: {TotalSizes}")
    print(f"Sum of sizes: {SumTotalSizes:,} ({SumTotalSizes})")
    print(f"Root size: {rootSize:,} ({rootSize})")
