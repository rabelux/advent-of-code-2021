mod lib;

use itertools::Itertools;
use crate::lib::{read_lines};

struct Grid{fields: Vec<Vec<usize>>, dimensions: Dimensions}
#[derive(Debug, Clone)]
struct Position {x: usize, y: usize}
#[derive(Debug, Clone)]
struct Dimensions{width: usize, height: usize}

fn main() {
    let result_a = task_a(read_lines("input/day_11.txt"));
    assert_eq!(1679, result_a);
    let result_b =  task_b(read_lines("input/day_11.txt"));
    assert_eq!(519, result_b);
    println!("task-a: {}, task-b: {}", result_a, result_b);
}

fn task_a(lines: impl Iterator<Item=String>) -> usize {
    let mut grid = Grid::new(lines);
    let mut count = 0;
    for _ in 0..100 {
       count += grid.run_step();
    }
    count
}

fn task_b(lines: impl Iterator<Item=String>) -> usize {
    let mut grid = Grid::new(lines);
    let field_count = grid.dimensions.width * grid.dimensions.height;
    let mut flash_count = 0;
    let mut step = 0;
    while flash_count != field_count {
        flash_count = grid.run_step();
        step += 1;
    }
    step
}

impl Grid {
    fn run_step(&mut self) -> usize {
        let mut flashing = self.rise_energy_of(self.positions());
        let mut flash_count = flashing.len();
        while !flashing.is_empty() {
            let adjacent =  flashing.iter()
                .map(|p|self.adjacent_positions(p))
                .flatten().collect_vec();
            flashing = self.rise_energy_of(adjacent.into_iter());
            flash_count += flashing.len();
        }
        self.reset_flashed_ones();
        flash_count
    }

    fn positions(&self) -> impl Iterator<Item=Position> {
        let Dimensions{width, height} = self.dimensions;
        (0usize..width).map(move |x| (0usize..height)
            .map(move |y| Position {x, y})).flatten()
    }

    fn adjacent_positions(&self, p: &Position) -> impl Iterator<Item=Position>{
        let p = p.clone();
        let Dimensions{width, height} = self.dimensions;
        (-1..2).map(move |x| (-1..2)
            .map(move |y| (p.x as isize+x, p.y as isize+y))).flatten()
            .filter(move |(x, y)|
                !(*x==p.x as isize && *y==p.y as isize)
                && *x>=0 && *x<width as isize
                && *y>=0 && *y<height as isize)
            .map(move |(x, y)|Position{x: x as usize, y: y as usize})
    }

    fn rise_energy_of(&mut self, positions: impl Iterator<Item=Position>) -> Vec<Position> {
        positions.filter_map(|p| {
            self.fields[p.x][p.y] += 1;
            match self.fields[p.x][p.y] {
                10 => Some(p),
                _ => None
            }
        }).collect()
    }

    fn reset_flashed_ones(&mut self) {
        self.positions()
            .for_each(|pos| if self.fields[pos.x][pos.y] > 9 {
                self.fields[pos.x][pos.y] = 0;
            })
    }

    fn new(lines: impl Iterator<Item=String>) -> Grid {
        let fields = lines.map(|line| line.chars()
            .map(|c| c.to_digit(10).expect("failed to parse") as usize).collect_vec()
        ).collect_vec();
        let (width, height) = (fields.len(), fields[0].len());
        Grid{fields, dimensions: Dimensions{width, height}}
    }
}
