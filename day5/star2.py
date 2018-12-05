def reduce_polymer(polymer):
    reduced = True
    while reduced:
        i = 0
        reduced = False

        while i < len(polymer)-1:
            if polymer[i] == polymer[i+1].lower() and polymer[i] != polymer[i+1]:
                polymer = polymer[:i] + polymer[i+2:]
                reduced = True
            elif polymer[i].lower() == polymer[i+1] and polymer[i] != polymer[i+1]:
                polymer = polymer[:i] + polymer[i+2:]
                reduced = True

            i += 1

    return len(polymer)

polymer = None
with open("test1") as f:
    polymer = f.read().strip()

min_length = len(polymer)
for c in range(ord('a'), ord('z')+1):
    length = reduce_polymer(polymer.replace(chr(c), '').replace(chr(c-32), ''))

    if length < min_length:
        min_length = length

print min_length
