metadata_sum = 0

def process_input(b):
    global metadata_sum
    children_count = b[0]
    metadata_count = b[1]

    b = b[2:]
    for i in range(children_count):
        b = process_input(b)

    for i in range(metadata_count):
        metadata_sum += b[0]
        b = b[1:]

    return b

inputs = []
with open("test1") as f:
    inputs = [int(s) for s in f.read().strip().split(" ")]

process_input(inputs)

print metadata_sum
