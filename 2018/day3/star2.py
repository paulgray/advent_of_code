import re

ids = [[0 for x in range(1000)] for y in range(1000)]
clear = []

with open("test1") as f:
    pattern = r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)'
    for line in f:
        match = re.search(pattern, line.strip())
        (i, left, top, width, height) = match.groups()

        clear.append(int(i))

        for x in range(int(left), int(left) + int(width)):
            for y in range(int(top), int(top) + int(height)):
                if ids[x][y] != 0:
                    if clear.count(ids[x][y]) > 0:
                        clear.remove(ids[x][y])
                    if clear.count(int(i)) > 0:
                        clear.remove(int(i))

                ids[x][y] = int(i)

print str(clear)
