import time

b = time.time()
msg = 'Time Taken'


def End():
    e = time.time()
    return f'{msg}: {e - b}'


def Msg(message):
    global msg
    msg = message
