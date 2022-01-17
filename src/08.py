def part1(stringlist):
    cnt = 0
    for i in stringlist:
        for j in i:
            if not len(j) in [5, 6]:
                cnt += 1
    return cnt

def part2(left, right):
    int_res = []
    for i in range(len(left)):
        # assign values to variables
        digit_2_3_5 = []
        digit_0_6_9 = []
        for j in left[i]:
            if len(j) == 2:
                digit_1 = frozenset(j)
            elif len(j) == 3:
                digit_7 = frozenset(j)
            elif len(j) == 4:
                digit_4 = frozenset(j)
            elif len(j) == 5: # 2, 3, 5
                digit_2_3_5.append(frozenset(j))
            elif len(j) == 6: # 0, 6, 9
                digit_0_6_9.append(frozenset(j))
            else:
                digit_8 = frozenset(j)
        for digit in digit_2_3_5:
            if digit_1 < digit:
                digit_3 = digit
            elif len(digit | digit_4) == 7:
                digit_2 = digit
            else:
                digit_5 = digit
        for digit in digit_0_6_9:
            if not digit_1 < digit:
                digit_6 = digit
            elif digit_3 < digit:
                digit_9 = digit
            else:
                digit_0 = digit
        
        lookup = {
            digit_0:'0',
            digit_1:'1',
            digit_2:'2',
            digit_3:'3',
            digit_4:'4',
            digit_5:'5',
            digit_6:'6',
            digit_7:'7',
            digit_8:'8',
            digit_9:'9'
            }

        str_res = "".join([lookup.get(frozenset(s)) for s in right[i]])
        int_res.append(int(str_res))
    return sum(int_res)

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
