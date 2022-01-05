def main(days):
    with open("..\\input\\06.txt",'r') as textfile:
        ages = [int(i) for i in textfile.readline().split(',')]
    
    for _ in range(days):
        i = 0
        for count, value in enumerate(ages):
            if not value:
                ages[count] = 6
                i += 1
            else:
                ages[count] -= 1
        ages.extend([8]*i)
    print("Amount of fish after",days,"days:", len(ages))

if __name__ == "__main__":
    main(80)
    main(256)