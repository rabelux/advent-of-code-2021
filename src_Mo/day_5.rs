mod lib;

use std::usize;
use itertools::Itertools;
use crate::lib::read_lines;

fn main() {
    let result_a = task(read_lines("input/day_5.txt"), false);
    assert_eq!(7644, result_a);
    let result_b = task(read_lines("input/day_5.txt"), true);
    assert_eq!(18627, result_b);
    println!("task-a: {}, task-b: {}", result_a, result_b);
}

fn task(lines: impl Iterator<Item=String>, with_diagonals: bool) -> usize {
    let relevant_lines = read_input(lines).into_iter()
        .filter(|Line(a,b)|
           with_diagonals || a.x == b.x || a.y == b.y
        ).collect_vec();

    let max = relevant_lines.iter()
        .map(|Line(a, b)| (a.x.max(b.x), a.y.max(b.y)))
        .reduce(|accum, max| (accum.0.max(max.0), accum.1.max(max.1))
    ).expect("failed getting boundaries");

    let mut map = vec![vec![0;max.1+1]; max.0+1];

    for Line(a, b) in relevant_lines.iter() {
        let mut current = *a;
        let vector = (b.x as isize - a.x as isize, b.y as isize - a.y as isize);
        loop {
            map[current.x][current.y] += 1;
            if &current == b { break; }
            current.x = (current.x as isize + vector.0.signum()) as usize;
            current.y = (current.y as isize + vector.1.signum()) as usize;
        }
    }
    map.iter().flatten().filter(|v| **v > 1).count()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Point{x: usize, y: usize}
struct Line(Point, Point);

fn read_input(lines: impl Iterator<Item=String>) -> Vec<Line> {
    lines.map(|line| line.split(" -> ")
            .map(|point|
                point.split(",")
                .map(|number| number.parse::<usize>().expect("parsing num failed"))
                .collect_vec()
            ).map(|v|Point{x: v[0], y: v[1]})
            .next_tuple().expect("parsing point failed"))
        .map(|(a, b)| Line(a, b))
        .collect()
}