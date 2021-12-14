mod lib;

use itertools::Itertools;
use crate::lib::read_lines;

fn main() {
    let result_a = task_a(read_lines("input/day_1.txt"));
    assert_eq!(result_a, 1374);
    let result_b = task_b(read_lines("input/day_1.txt"));
    assert_eq!(result_a, 1418);
    println!("task-a: {}, task-b: {}", result_a, result_b);
}

fn task_a(lines: impl Iterator<Item=String>) -> usize {
    lines
        .tuple_windows()
        .filter(|(a, b)|  b > a )
        .count() + 1
}

fn task_b(lines: impl Iterator<Item=String>) -> usize {
    lines
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b ,c )|[a, b, c])
        .map(|arr|arr.iter().map(|i|i.parse::<usize>().unwrap()).sum::<usize>())
        .tuple_windows()
        .filter(|(a, b)|  b > a )
        .count()
}


