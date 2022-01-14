def calc_fuel(positions, cumm_fuel = False):
    consumption = [0] * (max(positions) + 1)
    f = lambda x: int(x * (x + 1) / 2) if cumm_fuel else x
    for pos in range(len(consumption)):
        fuel = 0
        for i in positions:
            distance = abs(pos - i)
            fuel += f(distance)
        consumption[pos] = fuel
    return min(consumption)

def main():
    with open("..\\input\\07.txt",'r') as textfile:
        positions = [int(i) for i in textfile.readline().split(',')]
    result_1 = calc_fuel(positions)
    result_2 = calc_fuel(positions, True)
    print("Result 1:", result_1, "Result 2:", result_2)

if __name__ == "__main__":
    main()