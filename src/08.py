def part1(stringlist):
    cnt = 0
    for i in stringlist:
        for j in i:
            if not len(j) in [5, 6]:
                cnt += 1
    return cnt

def lookup7d():
    return {frozenset("abcefg"):'0', frozenset("cf"):'1', frozenset("acdeg"):'2',\
        frozenset("acdfg"):'3', frozenset("bcdf"):'4', frozenset("abdfg"):'5', frozenset("abdefg"):'6',\
        frozenset("acf"):'7', frozenset("abcdefg"):'8', frozenset("abcdfg"):'9'}

def part2(left, right):
    lookuptable = lookup7d()
    int_res = []
    for i in range(len(left)):
        # assign values to variables
        five_digits = []
        six_digits = []
        for j in left[i]:
            if len(j) == 2:
                one = set(j)
            elif len(j) == 3:
                seven = set(j)
            elif len(j) == 4:
                four = set(j)
            elif len(j) == 5: # 2, 3, 5
                five_digits.append(set(j))
            elif len(j) == 6: # 0, 6, 9
                six_digits.append(set(j))
            else:
                eight = set(j)
        # solve for segments
        a = seven.difference(one).pop() # 7 - 1
        for elem in five_digits:
            if one < elem: # found number 3
                b = four.difference(elem).pop() # 4 - 3
                g = elem.difference(four, a).pop() # 3 - 4 - a
                e = eight.difference(elem, b).pop() # 8 - 3 - b
                break
        d = four.difference(one, b).pop() # 4 - 1 - b
        for elem in six_digits:
            if not one < elem: # found 6
                c = one.difference(elem).pop() # 1 - 6
                break
        f = one.difference(c).pop() # 1 - c
        mytable = "".maketrans(a+b+c+d+e+f+g, 'abcdefg')
        str_res = "".join([lookuptable.get(frozenset(s.translate(mytable))) for s in right[i]])
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
