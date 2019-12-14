def get_value(b):
    children_count = b[0]
    metadata_count = b[1]

    b = b[2:]
    node_value = 0
    if children_count == 0:
        for i in range(metadata_count):
            node_value += b[0]
            b = b[1:]

        return (b, node_value)

    children_values = [0 for i in range(children_count)]
    for i in range(children_count):
        (b, v) = get_value(b)
        children_values[i] = v

    for i in range(metadata_count):
        idx = b[0]
        if idx == 0:
            pass
        elif idx > children_count:
            pass
        else:
            node_value += children_values[idx-1]

        b = b[1:]

    return (b, node_value)

inputs = []
with open("test1") as f:
    inputs = [int(s) for s in f.read().strip().split(" ")]

(b, v) = get_value(inputs)

print v
