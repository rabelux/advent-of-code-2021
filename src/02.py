#!/usr/bin/python3
 
import math

pos_x = 0
pos_y = 0

with open("..\\input\\02.txt",'r') as textfile:
    line = textfile.readline()
    while line:
        a = line.split()
        if a[0] == 'forward':
            pos_x += int(a[1])
        elif a[0] == 'up':
            pos_y -= int(a[1])
        elif a[0] == 'down':
            pos_y += int(a[1])
        else:
            print('Unparsed element: ' + line)
        
        line = textfile.readline()

print('Result: ' + str(pos_x*pos_y))
