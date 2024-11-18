import itertools

##########################
#### INTCODE COMPUTER ####
##########################

# run as follows:
#   computer = IntcodeComputer(code)
#   computer.run()
#
# for programmatic inputs,
#   computer = IntcodeComputer(code, inputs=[1, 2, ...])
# or
#   computer = IntcodeComputer(code)
#   computer.setInputs([1, 2, ...])
# before running
#
# for programmatic outputs,
#   computer = IntcodeComputer(code, storeOutputs=True)
# or
#   computer = IntcodeComputer(code)
#   computer.setStoreOutputs()

class IntcodeComputer():

    # map defining the recognized opcodes
    # and how many parameters they take
    NPARAM = {1:3, 2:3, 3:1, 4:1, 5:2, 6:2, 7:3, 8:3, 9:1, 99:0}


    # initialize with the program to be run
    def __init__(self, intcode, inputs=None, storeOutputs=False):
        self.intcode = intcode[:]
        self.pointer = 0
        self.halt    = False
        self.wait    = False
        self.relbase = 0

        self.setInputs(inputs)
        self.setStoreOutputs(storeOutputs)

    # wrapper for setting inputs, used by the constructor or the user
    def setInputs(self, inputs):
        if inputs is not None:
            self.inputs = list(reversed(inputs))
        else:
            self.inputs = None

    # wrapper for programmatically adding inputs
    # inputs are consumed by popping off a stack, so
    # adding inputs must be done by adding to the front
    # use this when the computer is in the WAIT state before running again
    def addInput(self, code):
        if self.inputs is not None:
            self.inputs.insert(0, code)

    # wrapper for storing outputs, used by the constructor or the user
    def setStoreOutputs(self, flag=True):
        self.storeOutputs = flag
        if self.storeOutputs:
            self.outputs = []

    # run the computer from pointer 0 with the stored program
    # the WAIT boolean PAUSES execution if there are not enough inputs
    # the only time WAIT is currently True is if there is a pending input
    # so assert that there's at least one input before resetting WAIT and continuing
    def run(self):
        if self.wait:
            assert(len(self.inputs) > 0)
            self.wait = False
        while not self.halt and not self.wait:
            self.compute()

    # for debugging purposes
    # print the current pointer, the halt and wait booleans, the inputs and outputs
    # inputs should be represented in reverse order so that inputs can be popped on and off
    def inspectState(self):
        print('Pointer: {:3d} Halt: {:5s} Wait: {:5s} RelBase: {} Inputs: {} Outputs: {}'.format(
            self.pointer,
            str(self.halt),
            str(self.wait),
            self.relbase,
            self.inputs,
            self.outputs,
        ))


    # abstract the access and write so that it can also allocate more memory if necessary
    # could have been done more cutely with __index__, I suppose. anyway,
    # this is so that a program can access / write to memory beyond the size of the program itself
    # whenever accessing or writing to a position, make sure that the position exists
    # the method for doing this is simple: add memory till, given a position
    # then all access and writes can proceed without out-of-bound errors
    # access: p ~ intcode[p]; p, q ~ intcode[p:q], but with memory addition
    def access(self, p, q=None):
        self.addMemoryTill(p)
        if q is None:
            return self.intcode[p]
        else:
            self.addMemoryTill(q)
            return self.intcode[p:q]
            
    # write: p, value ~ intcode[p] = value
    def write(self, p, value):
        self.addMemoryTill(p)
        self.intcode[p] = value

    # increase allocated memory until p is accessible
    def addMemoryTill(self, p):
        while p > len(self.intcode)-1:
            self.intcode.append(0)


    # given a pointer, cut up the opcode and get the modes
    # Suppose the opcode is 01102: this is opcode 2, with the params in mode 1 1 0
    # so get 110 by doing 01102 // 100 ( = 11), right justifying with 0s ( = 011), and reversing
    def getModes(self, p):
        opcode   = self.access(p) % 100
        if opcode not in IntcodeComputer.NPARAM:
            raise Exception('{} is not a valid op code'.format(opcode))
        parcodes = str(self.access(p) // 100).rjust(IntcodeComputer.NPARAM[opcode], '0')
        modes    = list( reversed([ int(i) for i in parcodes ]) )
        return modes

    # implementation of parameter modes
    # don't use for write positions; instead, use getAddress()
    # mode 0: position mode , return access(p)
    # mode 1: immediate mode, return p
    # mode 2: relative mode , return access(p + relbase)
    def getValue(self, p, mode):
        if   mode == 0:
            return self.access(p)
        elif mode == 1:
            return p
        elif mode == 2:
            return self.access(p + self.relbase)
        else:
            raise Exception(f'Unknown (read) parameter mode {mode}')

    # implementation of parameter mode 2 for write addresses
    # mode 0: position mode, return o
    # mode 2: relative mode, return o + relbase
    def getAddress(self, o, mode):
        if   mode == 0:
            return o
        elif mode == 2:
            return o + self.relbase
        else:
            raise Exception(f'Unknown (write) parameter mode {mode}')

    # main computer
    # processes the intcode at the current pointer
    def compute(self):

        p = self.pointer

        # opcode is the last two digits of intcode[p]
        # modes are the parameter modes, in the correct order (and of the correct length of parameters)
        # parameters are just intcode[p+1] through intcode[p+1 + nParams]
        # newp is where the pointer should move to; this is usually p + nParams + 1
        # compute it first, and reset it if the instruction demands it

        opcode = self.access(p) % 100
        modes  = self.getModes(p)
        params = self.access(p+1, p+IntcodeComputer.NPARAM[opcode]+1)
        newp   = p + IntcodeComputer.NPARAM[opcode] + 1

        print(f"Processing... POINTER: {p} OPCODE:{opcode} MODES:{modes} PARAMS:{params} RELATIVE:{self.relbase}")

        # add
        if   opcode == 1:
            x , y , o  = params
            mx, my, mo = modes
            self.write(self.getAddress(o, mo), self.getValue(x, mx) + self.getValue(y, my))

        # multiply
        elif opcode == 2:
            x , y , o  = params
            mx, my, mo = modes
            self.write(self.getAddress(o, mo), self.getValue(x, mx) * self.getValue(y, my))

        # input
        elif opcode == 3:
            o  = params[0]
            mo = modes [0]

            # get input from the command line if inputs is None
            if self.inputs is None:
                i = input('Provide input: ')

            # get input by popping off the inputs list if it exists
            else:

                # consume the next available input
                if len(self.inputs) > 0:
                    i = self.inputs.pop()

                # if there aren't any, pause execution
                # the run loop will break when this wait boolean is True
                # immediately end the computation
                # the next time run is called, wait will be reset to False
                # no pointers have changed, no modifications were made
                # so execution will resume from where it last left off
                else:
                    self.wait = True
                    return

            self.write(self.getAddress(o, mo), int(i))

        # output
        elif opcode == 4:
            x  = params[0]
            mx = modes [0]

            # print output or store the outputs
            value = self.getValue(x, mx)
            if not self.storeOutputs:
                print(value)
            else:
                self.outputs.append(value)

        # jump if true
        elif opcode == 5:
            t , v  = params
            mt, mv = modes
            if self.getValue(t, mt) != 0:
                newp = self.getValue(v, mv)

        # jump if false
        elif opcode == 6:
            t , v  = params
            mt, mv = modes
            if self.getValue(t, mt) == 0:
                newp = self.getValue(v, mv)

        # less than
        elif opcode == 7:
            a , b , o  = params
            ma, mb, mo = modes
            if self.getValue(a, ma) < self.getValue(b, mb):
                self.write(self.getAddress(o, mo), 1)
            else:
                self.write(self.getAddress(o, mo), 0)

        # equal to
        elif opcode == 8:
            a , b , o  = params
            ma, mb, mo = modes
            if self.getValue(a, ma) == self.getValue(b, mb):
                self.write(self.getAddress(o, mo), 1)
            else:
                self.write(self.getAddress(o, mo), 0)

        # relative base adjust
        elif opcode == 9:
            x  = params[0]
            mx = modes [0]
            value = self.getValue(x, mx)
            self.relbase += value

        # halt
        elif opcode == 99:
            self.pointer = newp
            self.halt    = True
            return

        # error
        else:
            raise Exception(f'{opcode} is not a valid op code')

        self.pointer = newp
        self.halt    = False


####

master = None
with open('d9_default_input') as f:
    for line in f:
        master = list(map(int, line.strip('\n').split(',')))

# part 1 and 2 at the same time, using the part number as input; kind of cute
# for part in (1, 2):
    # computer = IntcodeComputer(master, inputs=[part], storeOutputs=False)
    # print(f'Part {part}: ', end='')
    # computer.run()

computer = IntcodeComputer(master, inputs=[1], storeOutputs=False)
computer.run()
