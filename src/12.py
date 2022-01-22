class cave:
    def __init__(self):
        self.cave_dict = {}
        self.pathlist = []
        with open("..\\input\\12.txt",'r') as textfile:
            x = textfile.readline().strip().split('-')
            while len(x) > 1:
                for i in range(2):
                    if x[i-1] == 'start':
                        continue
                    self.cave_dict.setdefault(x[i],[]).append(x[i-1])
                x = textfile.readline().strip().split('-')

    def find_recursive(self, path, current_cave, allow_double_visit):
        for val in self.cave_dict[current_cave]:
            if val == 'end':
                self.pathlist.append(path + [val])
            elif val.islower() and val in path:
                if allow_double_visit:
                    self.find_recursive(path + [val], val, False)
                else:
                    continue
            else:
                self.find_recursive(path + [val], val, allow_double_visit)
        return

def main():
    part_1 = cave()
    part_1.find_recursive([], 'start', False)
    
    part_2 = cave()
    part_2.find_recursive([], 'start', True)

    print('part 1:', len(part_1.pathlist))
    print('part 2:', len(part_2.pathlist))

if __name__ == "__main__":
    main()