def calc_error_score(brackets):
    lookup = {')': 3, ']': 57, '}': 1197, '>': 25137}
    return sum([lookup[i] for i in brackets])

def compl_score(missing_chars):
    lookup = {')': 1, ']': 2, '}': 3, '>': 4}
    scores = []
    for i in missing_chars:
        score = 0
        for j in i:
            score *= 5
            score += lookup[j]
        scores.append(score)
    scores.sort()
    return scores[int((len(scores) - 1) / 2)]

def part1_2(strlist):
    corrupted = []
    missing = []
    lookup = {'(':')', '{':'}', '[':']', '<':'>'}
    for string in strlist:
        opened = []
        clean = True
        for char in string:
            if char in ['(', '{', '[', '<']:
                opened.append(char)
            elif lookup[opened.pop()] == char:
                continue
            else:
                corrupted.append(char)
                clean = False
                break
        if clean:
            missing.append([lookup[i] for i in opened.__reversed__()])
    return corrupted, missing

def main():
    with open("..\\input\\10.txt",'r') as textfile:
        content = textfile.readlines()
        content = [i.strip() for i in content]
    corr, missing = part1_2(content)
    print('part 1: ', calc_error_score(corr))
    print('part 2: ', compl_score(missing))

if __name__ == "__main__":
    main()