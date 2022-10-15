import os

year = input("Please enter the current year: ")

for i in range(25):
    try:
        os.makedirs(f"{year}/{i + 1}")
    except FileExistsError:
        pass