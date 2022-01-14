#!/usr/bin/python3
 
import math

with open("..\input\01.txt",'r') as textfile:
    str_content = textfile.readlines()
int_content = [int(i) for i in str_content]

#Testing
#int_content = [199,200,208,210,200,207,240,269,260,263]

## Part 2
tms = [] #ThreeMeasurementSum
for i in range(len(int_content)-2):
    tms.append(sum(int_content[i:i+3]))


## Part 1
pre = math.inf
counter = 0

for x in tms: #use 'int_content' for part 1 and 'tms' for part 2
    if x > pre:
        counter += 1
    pre = x

print("Occurances: " + str(counter))
