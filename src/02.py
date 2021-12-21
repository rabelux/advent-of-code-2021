#!/usr/bin/python3
 
pos_x = 0
depth = 0
aim = 0

with open("..\\input\\02.txt",'r') as textfile:
    line = textfile.readline()
    while line:
        a = line.split()
        if a[0] == 'forward':
            pos_x += int(a[1])
            depth += aim * int(a[1])
        elif a[0] == 'up':
            aim -= int(a[1])
        elif a[0] == 'down':
            aim += int(a[1])
        else:
            print('Unparsed element: ' + line)
        
        line = textfile.readline()

print('Result: ' + str(pos_x * depth))
