serial_no = 8979
#serial_no = 18

def power(x, y):
    rack_id = x + 10
    level = rack_id * y
    level += serial_no
    level *= rack_id
    level = (level % 1000) / 100
    return level - 5

grid = [[power(x, y) for x in range(300)] for y in range(300)]
cache = [[0 for x in range(300)] for y in range(300)]

def find_max(size):
    max_fuel = max_x = max_y = 0

    for x in range(301-size):
        for y in range(301-size):
            fuel = cache[x][y]
            for i in range(size):
                fuel += grid[x+size-1][y+i]
                fuel += grid[x+i][y+size-1]
            fuel -= grid[x+size-1][y+size-1]

            cache[x][y] = fuel

            if fuel > max_fuel:
                max_fuel = fuel
                max_x = x
                max_y = y

    return (max_x, max_y, max_fuel)

optimal = x = y = s = 0
for size in range(1, 301):
    (x1, y1, f) = find_max(size)
    if f > optimal:
        optimal = f
        s = size
        x = x1
        y = y1

print (y, x, s)
