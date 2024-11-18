if __name__ == "__main__":
    file = input("file: ")
    data = ""
    with open(file) as f:
        data = f.read()

    data = data.replace(',', '\n')
    with open(file + '2', 'w') as f:
        f.write(data)

