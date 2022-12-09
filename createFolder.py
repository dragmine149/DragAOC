import os
import shutil

year = input("Please enter the current year: ")

try:
    os.makedirs(f"{year}")
except FileExistsError:
    print(f"Year {year} already created")

for i in range(25):
    try:
        shutil.copytree("templates", f"{year}/{i + 1}")
    except FileExistsError:
        print(f"{i} already exists")
