frequencies = set()
frequency = 0
frequencies.add(frequency)
found = False

while not found:
    with open("test1") as f:
        for line in f:
            frequency += int(line)
            if frequency in frequencies:
                found = True
                break

            frequencies.add(frequency)

print frequency
