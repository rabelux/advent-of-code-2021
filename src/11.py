import math

class board:
    def __init__(self, content):
        self.grid = [[int(i) for i in line.strip()] for line in content]
        self.flashes = 0

    def check_pos(self, row, col):
        self.grid[row][col] += 1
        if self.grid[row][col] > 9:
            self.grid[row][col] = -math.inf
            self.flashes += 1
            self.inc_adjacent(row, col)

    def inc_adjacent(self, row, col):
        for i in range(row - 1, row + 2):
            if not 0 <= i < 10:
                continue
            for j in range(col - 1, col + 2):
                if not 0 <= j < 10:
                    continue
                self.check_pos(i, j)
    
    def reset(self):
        for row in range(10):
            for col in range(10):
                if self.grid[row][col] == -math.inf:
                    self.grid[row][col] = 0

def part_1(content):
    a = board(content)
    for step in range(100):
        for i in range(10):
            for j in range(10):
                a.check_pos(i,j)
        a.reset()
    return a.flashes

def part_2(content):
    a = board(content)
    diff = 0
    counter = 0
    while diff != 100:
        counter += 1
        diff = a.flashes
        for i in range(10):
            for j in range(10):
                a.check_pos(i,j)
        a.reset()
        diff = a.flashes - diff
    return counter

def main():
    with open("..\\input\\11.txt",'r') as textfile:
        content = textfile.readlines()
    
    print('part 1:', part_1(content))
    print('part 2:', part_2(content))

if __name__ == "__main__":
    main()