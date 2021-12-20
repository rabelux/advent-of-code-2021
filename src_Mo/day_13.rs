mod lib;

use std::fmt::{Debug, Formatter};
use itertools::{Either, Itertools};
use crate::lib::{read_lines};

fn main() {
    let result_a = task_a(read_lines("input/day_13.txt"));
    assert_eq!(755, result_a);
    println!("task-a: {}", result_a);

    task_b(read_lines("input/day_13.txt"));
}

fn task_a(lines: impl Iterator<Item=String>) -> usize {
    let (points, folds) = read_input(lines);
    let mut grid = Grid::new(points, &folds);
    grid.fold(&vec![folds[0].clone()]);
    grid.count_marked()
}

fn task_b(lines: impl Iterator<Item=String>) {
    let (points, folds) = read_input(lines);
    let mut grid = Grid::new(points, &folds);
    grid.fold(&folds);
    println!("{:?}", &grid);
}

impl Grid {
    fn new(points: Vec<(usize, usize)>, folds: &Vec<(String,usize)>) -> Grid {
        let (width, height) = folds.iter().fold((0usize, 0usize),|(width, height), (axis, index)| {
            match axis.as_str() {
                "x" => (width.max(*index*2+1), height),
                "y" => (width, height.max(*index*2+1)),
                _ => panic!("unexpected axis")
            }
        });
        let mut fields = vec![vec![false; height]; width];
        points.iter().for_each(|(x, y)| fields[*x][*y] = true);
        Grid { fields, width, height}
    }

    fn fold_up(&mut self, at_row: usize) {
        for x in 0..self.width {
            for y in at_row+1..self.height {
                let folded_y = self.height-y-1;
                let folded_value = self.fields[x][y];
                self.fields[x][folded_y] |= folded_value;
            }
        }
        self.height = at_row;
        self.fields.iter_mut().for_each(|col|col.truncate(at_row));
    }

    fn fold_left(&mut self, at_col: usize) {
        for x in at_col+1..self.width {
            for y in 0..self.height {
                let folded_x = self.width-x-1;
                let folded_value = self.fields[x][y];
                self.fields[folded_x][y] |= folded_value;
            }
        }
        self.width = at_col;
        self.fields.truncate(at_col);
    }

    fn fold(&mut self, folds: &Vec<(String, usize)>) {
        for (dir, pos) in folds.iter() {
            match (dir.as_str(), pos) {
                ("x", pos) => self.fold_left(*pos),
                ("y", pos) => self.fold_up(*pos),
                _ => panic!("unexpected fold")
            }
        }
    }

    fn count_marked(&self) -> usize {
        self.fields.iter()
            .map(|col|col.iter().filter(|v|**v).count()).sum()
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = "    ".to_string();
        for x in 0..self.width {
            s.push_str(&format!("{}", x%10));
        }
        s.push_str("\n");
        for y in 0..self.height {
            s.push_str(&format!("{: <3}|", y));
            for x in 0..self.width {
                s.push_str(&format!("{}", if self.fields[x][y] { "█" } else { "." }));
            }
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}

fn read_input(lines: impl Iterator<Item=String>) -> (Vec<(usize, usize)>, Vec<(String, usize)>) {
    lines
        .filter(|l| !l.is_empty())
        .partition_map(|l| if l.contains(",") {
            Either::Left(parse_point(&l))
        } else {
            Either::Right(parse_fold(&l))
        })
}

fn parse_point(line: &str) -> (usize, usize) {
    line.split(",")
        .map(|v| v.parse::<usize>().expect("parse failed"))
        .collect_tuple().expect("illegal input")
}

fn parse_fold(line: &str) -> (String, usize) {
    line.split(" ")
        .skip(2)
        .next()
        .map(|s| s.split("=").collect_vec())
        .map(|v| (v[0].to_string(), v[1].parse::<usize>().expect("parse failed")))
        .expect("illegal input")
}


#[derive(Copy, Clone)]
enum Axis { X, Y }
struct Grid { width: usize, height: usize, fields: Vec<Vec<bool>>}