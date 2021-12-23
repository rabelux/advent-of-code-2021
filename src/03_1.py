#!/usr/bin/python3

i = 0
with open("..\\input\\03.txt",'r') as textfile:
    line = textfile.readline()
    cnt = [0]*(len(line)-1)
    while line:
        i += 1
        for x in range(len(cnt)):
            if line[x] == '1':
                cnt[x] += 1
        line = textfile.readline()

gamma_s = ''
eps_s = ''
for x in range(len(cnt)):
    if cnt[x] > i/2:
        gamma_s += '1'
        eps_s += '0'
    else:
        gamma_s += '0'
        eps_s += '1'

gamma_i = int(gamma_s, base=2)
eps_i = int(eps_s, base=2)

print("gamma: " + str(gamma_i) + "\neps: " + str(eps_i))
print("Result: " + str(gamma_i * eps_i))
