def calc_error_score(brackets):
    lookup = {')': 3, ']': 57, '}': 1197, '>': 25137}
    return sum([lookup[i] for i in brackets])

def part1(strlist):
    opened = []
    corrupted = []
    lookup = {'(':')', '{':'}', '[':']', '<':'>'}
    for string in strlist:
        for char in string:
            if char in ['(', '{', '[', '<']:
                opened.append(char)
            elif lookup[opened.pop()] == char:
                continue
            else:
                corrupted.append(char)
                break
    return calc_error_score(corrupted)

def part2(grid, low_points):
    enh_grid = [[[i, False] for i in row] for row in grid]
    basin_sizes = []
    for row, col in low_points:
        basin_sizes.append(find_neighbours(enh_grid, row, col))
    basin_sizes.sort(reverse=True)
    result = 1
    for i in range(3):
        result *= basin_sizes[i]
    return result

def main():
    with open("..\\input\\10.txt",'r') as textfile:
        content = textfile.readlines()
        content = [i.strip() for i in content]
    
    print('part 1: ', part1(content))
    #print('part 2: ', part2(grid, sol_1[1]))

if __name__ == "__main__":
    main()