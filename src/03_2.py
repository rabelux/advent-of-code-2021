#!/usr/bin/python3

import bisect

#Expect an array containing numbers
def eliminate_by_occurance(array, bit):
    comp = 0
    for z in reversed(range(bit)):
        i = bisect.bisect_left(array, comp + (1<<z))
        if i < len(array)/2:
            array = array[:i]
        else:
            array = array[i:]
            comp += 1<<z

        print("Comparing bit", str(z), "yielded", str(len(array)), "results.")
        if len(array) < 2:
            return array[0]
        

with open("..\\input\\03.txt",'r') as textfile:
    string_array = textfile.readlines()

#TESTDATA
#string_array = ["00100","11110","10110","10111","10101","01111","00111","11100","10000","11001","00010","01010"]

#determine bit length
bitlength = len(string_array[0].strip('\n'))
print("Total bits per line:", str(bitlength))
#create sorted int array
num_array = [int(i.strip('\n'), base=2) for i in string_array]
num_array.sort()

##get oxygen generator rating
oxygen = num_array.copy()
z = bitlength - 1
comp = 0
while len(oxygen) > 1:
    i = bisect.bisect_left(oxygen, comp + (1<<z))
    if i > len(oxygen)/2:
        oxygen = oxygen[:i]
    else:
        oxygen = oxygen[i:]
        comp += 1<<z
    print("Comparing o2-bit", str(z), "yielded", str(len(oxygen)), "results.")
    z -= 1
print("Oxygen Generator is:", str(oxygen))

##get CO2 scrubber rating
co2 = num_array.copy()
z = bitlength - 1
comp = 0
while len(co2) > 1:
    i = bisect.bisect_left(co2, comp + (1<<z))
    if i > len(co2)/2:
        co2 = co2[i:]
        comp += 1<<z
    else:
        co2 = co2[:i]
    print("Comparing co2-bit", str(z), "yielded", str(len(co2)), "results.")
    z -= 1
print("CO2 scrubber rating is:", str(co2))

print("Solution for O2*CO2:", str(oxygen[0]*co2[0]))