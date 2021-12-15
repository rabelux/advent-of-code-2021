mod lib;

use std::usize;
use crate::lib::read_lines;

fn main() {
    let result_a = task(read_lines("input/day_6.txt"), 80);
    assert_eq!(388419, result_a);
    let result_b = task(read_lines("input/day_6.txt"), 256);
    assert_eq!(1740449478328, result_b);
    println!("task-a: {}, task-b: {}", result_a, result_b);
}

fn task(mut lines: impl Iterator<Item=String>, days: usize) -> usize {
    let values = read_input(&mut lines);
    let mut gens: Vec<Generation> = vec![];
    (0..9).for_each(|i| gens.push((i, 0)));
    values.iter().for_each(|v| gens[*v].1+=1);
    (0..days).for_each(|_|next_cycle(&mut gens));
    gens.iter().map(|g|g.1).sum()
}

fn next_cycle(gen: &mut Vec<Generation>) {
    gen.iter_mut().skip(1).for_each(|g|g.0-=1);
    gen[7].1 += gen[0].1;
    gen[0].0 = 8;
    gen.sort_by_key(|g| g.0);
}

type Generation = (usize, usize);

fn read_input(lines: &mut impl Iterator<Item=String>) -> Vec<usize> {
    lines.next().expect("input file invalid")
        .split(",")
        .map(|s|s.parse::<usize>().expect("parse failed"))
        .collect()
}