class board:
    def __init__(self, fields):
        if sum([len(listElem) for listElem in fields]) != 25:
            raise ValueError 
        self.fields = [[fields[i][j],False] for i in range(5) for j in range(5)]

    def mark(self, string):
        # mark field if exists
        for count, value in enumerate(self.fields):
            if value[0] == string:
                self.fields[count][1] = True
                return True
        return False

    def check(self):
        ## check if bingo is true
        # check lines
        for x in range(0,25,5):
            if all([self.fields[x:x+5][i][1] for i in range(5)]):
                return True
        #check columns
        for x in range(5):
            if all([self.fields[x:25:5][i][1] for i in range(5)]):
                return True
        return False

    def sum_unmarked(self):
        return sum([int(i[0]) for i in self.fields if not i[1]])

    def mark_n_check(self, string):
        if self.mark(string) and self.check():
            return self.sum_unmarked()
        return 0

def mark_boards(boardlist, numberstring, scorelist):
    tmp = []
    for elem in boardlist:
        a = elem.mark_n_check(numberstring)
        if a:
            tmp.append(elem)
            scorelist.append(a*int(numberstring))
    [boardlist.remove(i) for i in tmp]

def main():
    boards = []
    with open("..\\input\\04.txt",'r') as textfile:
        numberlist = textfile.readline().split(',')
        textfile.readline()
        content = textfile.readlines()
    ##TESTDATA
    #numberlist = [str(i) for i in [7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1]]
    #content = ["22 13 17 11  0\n", "8  2 23  4 24\n", "21  9 14 16  7\n", "6 10  3 18  5\n", " 1 12 20 15 19\n", "\n"]
    #content.extend([" 3 15  0  2 22\n", "9 18 13 17  5\n", "19  8  7 25 23\n", "20 11 10 24  4\n", "14 21 16 12  6\n", "\n"])
    #content.extend(["14 21 17 24  4\n", "10 16 15  9 19\n", "18  8 23 26 20\n", "22 11 13  6  5\n", " 2  0 12  3  7\n", "\n"])

    for i in range(0,len(content),6):
        board_tmp = [content[i:i+5][z].split() for z in range(5)]
        boards.append(board(board_tmp))
    
    scorelist = []
    while numberlist and boards:
        current_num = numberlist.pop(0)
        print("Number drawn:", current_num)
        mark_boards(boards, current_num, scorelist)

    print("Score of first board:", scorelist[0])
    print("Score of last board:", scorelist[-1])

if __name__ == "__main__":
    main()