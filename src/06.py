def main(days):
    with open("..\\input\\06.txt",'r') as textfile:
        ages = [int(i) for i in textfile.readline().split(',')]
    
    fishdict = dict.fromkeys([0, 1, 2, 3, 4, 5, 6, 7 ,8], 0)
    for i in ages:
        fishdict[i] += 1
    
    for _ in range(days):
        tmp = fishdict[0]
        for i in range(8):
            fishdict[i] = fishdict[i+1]
        fishdict[6] += tmp
        fishdict[8] = tmp
    
    print("Amount of fish after",days,"days:", sum(fishdict.values()))

if __name__ == "__main__":
    main(80)
    main(256)