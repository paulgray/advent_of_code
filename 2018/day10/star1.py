import re
import time
from Tkinter import *

def simulate(p):
    return map(lambda (x, y, vx, vy): (x+vx, y+vy, vx, vy), p)

def print_points(points):
    (x_spread, y_spread) = max_spread(points)
    canvas = [[False for y in range(2*y_spread)] for x in range(2*x_spread)]
    for (x, y, v1, v2) in points:
        canvas[x][y] = True
    for y in range(y_spread+1):
        s = ''
        for x in range(x_spread+1):
            if canvas[x][y]:
                s += '#'
            else:
                s += ' '
        print s
    print ''

def max_spread(p):
    min_x = min_y = 922337203685477580
    max_x = max_y = -922337203685477580
    for (x, y, v1, v2) in p:
        if x < min_x:
            min_x = x
        if y < min_y:
            min_y = y
        if x > max_x:
            max_x = x
        if y > max_y:
            max_y = y

    return (max_x-min_x, max_y-min_y)

def normalize(p):
    min_x = min_y = 922337203685477580
    max_x = max_y = -922337203685477580
    for (x, y, v1, v2) in p:
        if x < min_x:
            min_x = x
        if y < min_y:
            min_y = y
        if x > max_x:
            max_x = x
        if y > max_y:
            max_y = y

    p = map(lambda (x, y, vx, vy): (x-min_x, y-min_y, vx, vy), p)
    max_x -= min_x
    max_y -= min_y

    return p

points = []
with open("test1") as f:
    pattern = r'position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>'
    for line in f:
        match = re.search(pattern, line)
        (x, y, vx, vy) = match.groups()
        points.append((int(x), int(y), int(vx), int(vy)))

counter = 0
while True:
    (x_spread, y_spread) = max_spread(points)
    if x_spread < 75 and y_spread < 75:
        print "Found message in " + str(counter) + " steps"
        break

    points = simulate(points)

    if (counter % 1000) == 0:
        print "Step " + str(counter)

    counter += 1

points = normalize(points)
for i in range(5):
    print_points(points)
    points = normalize(simulate(points))
