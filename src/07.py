def part1(positions):
    consumption = [0] * (max(positions) + 1)
    for pos in range(len(consumption)):
        fuel = 0
        for i in positions:
            fuel += abs(pos - i)
        consumption[pos] = fuel
    print("Min Consumption possible for part 1:", min(consumption))

def part2(positions):
    consumption = [0] * (max(positions) + 1)
    for pos in range(len(consumption)):
        fuel = 0
        for i in positions:
            distance = abs(pos - i)
            fuel += distance * (distance + 1) / 2
        consumption[pos] = fuel
    print("Min Consumption possible for part 2:", min(consumption))

def main():
    with open("..\\input\\07.txt",'r') as textfile:
        positions = [int(i) for i in textfile.readline().split(',')]
    
    part1(positions)
    part2(positions)

if __name__ == "__main__":
    main()