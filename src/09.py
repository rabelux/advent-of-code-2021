def part1(grid):
    risk = 0
    low_points = []
    for i in range(1, len(grid) - 1):
        for j in range(1, len(grid[0]) - 1):
            if grid[i][j] < min(grid[i-1][j], grid[i+1][j], grid[i][j-1], grid[i][j+1]):
                risk += int(grid[i][j]) + 1
                low_points.append((i, j))
    return risk, low_points

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

def find_neighbours(grid, row, col):
    basin_size = 0
    # check if current field is part of basin
    if grid[row][col][0] < '9' and grid[row][col][1] == False:
        basin_size += 1
        grid[row][col][1] = True
    else:
        return 0
    # check above
    basin_size += find_neighbours(grid, row - 1, col)
    # check right
    basin_size += find_neighbours(grid, row, col + 1)
    # check below
    basin_size += find_neighbours(grid, row + 1, col)
    # check left
    basin_size += find_neighbours(grid, row, col - 1)
        
    return basin_size

def create_grid(content):
    grid = [list('9' + line.strip() + '9') for line in content]
    grid.insert(0, list('9' * len(grid[0])))
    grid.append(grid[0])
    return grid

def main():
    with open("..\\input\\09.txt",'r') as textfile:
        content = textfile.readlines()
    grid = create_grid(content)
    sol_1 = part1(grid)
    
    print('part 1: ', sol_1[0])
    print('part 2: ', part2(grid, sol_1[1]))

if __name__ == "__main__":
    main()