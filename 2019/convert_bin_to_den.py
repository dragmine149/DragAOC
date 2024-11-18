l = []
for i in range(8):
    inp = input('num: ')
    if inp == '':
        l.append(0)
    else:
        l.append(int(inp))

# l = [240, 81, 48, 0, 0, 0, 0, 0]
s = 0
for i in range(len(l)):
  s += (256 ** i) * l[i]
print(f'{s:,}')
