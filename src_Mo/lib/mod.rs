use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Map;
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Map<Lines<BufReader<File>>, fn(std::io::Result<String>) -> String>
    where P: AsRef<Path>, {
    let file = File::open(filename).expect("file not found");
    io::BufReader::new(file).lines()
        .map(|s|s.expect("Failed reading line"))
}

pub fn read_usize_vec(lines: &mut impl Iterator<Item=String>) -> Vec<usize> {
    lines.next().expect("input file invalid")
        .split(",")
        .map(|s|s.parse::<usize>().expect("parse failed"))
        .collect()
}