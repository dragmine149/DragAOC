import os
import shutil

year = input("Please enter the current year: ")

try:
    os.makedirs(f"{year}")
except FileExistsError:
    print("Year already created")

for i in range(25):
    try:
        shutil.copytree("templates", f"{year}/{i + 1}")
    except FileExistsError:
        print("Day already exists")