import file
import run


class Class():
    def __init__(self, debug):
        self.Debug = debug.lower()[0] == 't'
        self.lines = file.get()
        if self.debug:
            self.lines = file.get2()


if __name__ == '__main__':
    c = Class(input('Debug?: '))
    print(run.End())
