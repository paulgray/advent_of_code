import re

fabric = [[0 for x in range(1000)] for y in range(1000)]

with open("test1") as f:
    pattern = r'#\d+ @ (\d+),(\d+): (\d+)x(\d+)'
    for line in f:
        match = re.search(pattern, line.strip())
        (left, top, width, height) = match.groups()

        for x in range(int(left), int(left) + int(width)):
            for y in range(int(top), int(top) + int(height)):
                fabric[x][y] += 1

counter = 0
for x in range(1000):
    for y in range(1000):
        if fabric[x][y] > 1:
            counter += 1

print counter
