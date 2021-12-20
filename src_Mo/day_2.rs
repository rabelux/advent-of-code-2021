mod lib;

use itertools::Itertools;
use crate::lib::read_lines;

fn main() {
    let result_a = task_a(read_lines("input/day_2.txt"));
    assert_eq!(result_a, 2215080);
    let result_b = task_b(read_lines("input/day_2.txt"));
    assert_eq!(result_b, 1864715580);
    println!("task-a: {}, task-b: {}", result_a, result_b);
}

fn task_a(lines: impl Iterator<Item=String>) -> isize {
    let (x, y) = lines
        .map(|line|
            line.split_whitespace()
            .map(str::to_owned).next_tuple().expect("split failed"))
        .map(|(direction, magnitude)|
            (direction, magnitude.parse::<isize>().expect("parsing magnitude failed")))
        .map(|(direction, magnitude)|
            match direction.as_str() {
                "up" => (0, -magnitude),
                "down" => (0, magnitude),
                "forward" => (magnitude, 0),
                _ => panic!("unexpected direction!")
            })
        .reduce(|(x, y), (delta_x, delta_y)| (x+delta_x, y+delta_y))
        .expect("reduce failed");
    x * y
}

fn task_b(lines: impl Iterator<Item=String>) -> isize {
    let (_, x, y) = lines
        .map(|line|
            line.split_whitespace()
                .map(str::to_owned).next_tuple().expect("split failed"))
        .map(|(direction, magnitude)|
            (direction, magnitude.parse::<isize>().expect("parsing magnitude failed")))
        .fold((0, 0, 0), |(aim, x, y), (direction, magnitude)|
            match direction.as_str() {
                "up" => (aim - magnitude, x, y),
                "down" => (aim + magnitude, x, y),
                "forward" => (aim, x + magnitude, y+aim*magnitude),
                _ => panic!("unexpected direction!")
            });
    x * y
}