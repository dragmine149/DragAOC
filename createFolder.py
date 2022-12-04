import os
import shutil

year = input("Please enter the current year: ")

for i in range(25):
    try:
        os.makedirs(f"{year}/{i + 1}")
        shutil.copy('helper.py', f"{year}/{i + 1}/helper.py")
    except FileExistsError:
        pass
