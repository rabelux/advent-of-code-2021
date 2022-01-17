def part1(stringlist):
    cnt = 0
    for i in stringlist:
        for j in i:
            if not len(j) in [5, 6]:
                cnt += 1
    return cnt

def part2(left, right):
    int_res = 0
    segmcount2index = {2 : 1, 3 : 7, 4 : 4, 7: 8}
    for i in range(len(left)):
        digit_2_3_5 = []
        digit_0_6_9 = []
        digits = [0] * 10
        for j in left[i]:
            if len(j) == 5: # 2, 3, 5
                digit_2_3_5.append(frozenset(j))
            elif len(j) == 6: # 0, 6, 9
                digit_0_6_9.append(frozenset(j))
            else:
                digits[segmcount2index[len(j)]] = frozenset(j)
        
        for digit in digit_2_3_5:
            if digits[1] < digit:
                digits[3] = digit
            elif len(digit | digits[4]) == 7:
                digits[2] = digit
            else:
                digits[5] = digit
        
        for digit in digit_0_6_9:
            if not digits[1] < digit:
                digits[6] = digit
            elif digits[3] < digit:
                digits[9] = digit
            else:
                digits[0] = digit
        
        lookup = dict(zip(digits,'0123456789'))
        str_res = "".join([lookup.get(frozenset(s)) for s in right[i]])
        int_res += int(str_res)
    return int_res

def main():
    with open("..\\input\\08.txt",'r') as textfile:
        content = textfile.readlines()
    partitioned = [i.partition('|') for i in content]
    right_side = [i[-1].split() for i in partitioned]
    left_side = [i[0].split() for i in partitioned]
    print('part 1: ', part1(right_side))
    print('part 2: ', part2(left_side, right_side))

if __name__ == "__main__":
    main()
