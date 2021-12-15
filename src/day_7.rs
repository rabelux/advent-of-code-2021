mod lib;

use std::usize;
use crate::lib::{read_lines, read_usize_vec};

fn main() {
    let result_a = task(read_lines("input/day_7.txt"), false);
    assert_eq!(354129, result_a);
    let result_b =  task(read_lines("input/day_7.txt"), true);
    assert_eq!(98905973, result_b);
    println!("task-a: {}, task-b: {}", result_a, result_b);
}

fn task(mut lines: impl Iterator<Item=String>, increasing_fuel: bool) -> usize {
    let positions = read_usize_vec(&mut lines);
    let min = *positions.iter().min().expect("no positions");
    let max = positions.iter().max().expect("no positions")+1;

    (min..max).map(|i| fuel_to_target(&positions, i, increasing_fuel))
        .min().expect("no positions?")
}

fn fuel_to_target(current: &Vec<usize>, target: usize, increasing_fuel: bool) -> usize{
    current.iter().map(|pos| {
        let steps = (*pos as isize - target as isize).abs() as usize;
        if increasing_fuel {(1..steps+1).sum::<usize>()} else { steps }
    }).sum::<usize>()
}

