mod lib;

use itertools::Itertools;
use crate::lib::{read_lines};

fn main() {
    let result_a = task_a(read_lines("input/day_10.txt"));
    assert_eq!(216297, result_a);
    let result_b =  task_b(read_lines("input/day_10.txt"));
    assert_eq!(847504, result_b);
    println!("task-a: {}, task-b: {}", result_a, result_b);
}

fn task_a(lines: impl Iterator<Item=String>) -> usize {
    lines.map(|line| corruption_score(&line)).sum()
}

fn task_b(lines: impl Iterator<Item=String>) -> usize {
    let scores = lines.filter(|line| corruption_score(&line) == 0)
        .map(|line| completion_score(&line))
        .sorted()
        .collect_vec();
    scores[scores.len()/2]
}

fn completion_score(line: &String) -> usize {
    let mut stack = vec![];

    line.chars().for_each(|c| match c {
        ')' | ']' | '}' | '>' => {stack.pop();},
        s => stack.push(s),
    });

    stack.iter().rev().map(|c| match c {
        '('  => 1, '[' => 2, '{' => 3, '<' => 4,
        _ => panic!("unexpected char {}", c)
    } as usize).fold(0, |sum , score| sum * 5 + score )
}

fn corruption_score(line: &String) -> usize {
    let mut stack = vec![];
    let mut total_score = 0;

    for c in line.chars() {
        let score = match c {
            ')' => stack.pop().filter(|c| c==&'(').map_or(3, |_|0),
            ']' => stack.pop().filter(|c| c==&'[').map_or(57, |_|0),
            '}' => stack.pop().filter(|c| c==&'{').map_or(1197, |_|0),
            '>' => stack.pop().filter(|c| c==&'<').map_or(25137, |_|0),
            s =>  {stack.push(s); 0},
        };
        if score > 0 {
            total_score += score;
            break;
        }
    };
    total_score
}