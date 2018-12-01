import fileinput

frequency = 0
for line in fileinput.input():
    frequency += int(line)

print frequency
