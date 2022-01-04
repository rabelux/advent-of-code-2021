def remove_diags(list):
    return [i for i in list if (i[0] == i[2] or i[1] == i[3])]

def extend(x1,y1,x2,y2):
    a = max(map(abs,[x2-x1, y2-y1]))
    return [(int(x1+i*(x2-x1)/a), int(y1+i*(y2-y1)/a)) for i in range(a+1)]

def main():
    boards = []
    with open("..\\input\\05.txt",'r') as textfile:
        content = textfile.readlines()
    
    #create vector with format [x1,y1,x2,y2]
    vector = [[int(s.replace(" -> ",",").split(",")[i]) for i in range(4)] for s in content]
    red_vector = remove_diags(vector)
    
    x_max = max([max(i[0],i[2]) for i in red_vector])
    y_max = max([max(i[1],i[3]) for i in red_vector])
    xy_map = [[0] * (x_max + 1) for _ in range(y_max + 1)]
    for i in red_vector:
        for j in extend(*i):
            xy_map[j[1]][j[0]] += 1
    #count occurances where xy_map > 1
    
    occ = sum([len(i)-i.count(0)-i.count(1) for i in xy_map])
    print("Occurances greater 1:", occ)


if __name__ == "__main__":
    main()