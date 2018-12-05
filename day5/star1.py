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

print reduce_polymer(polymer)
