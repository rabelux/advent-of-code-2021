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