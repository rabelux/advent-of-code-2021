mod lib;

use std::collections::HashSet;
use std::usize;
use itertools::Itertools;
use crate::lib::{read_lines};

fn main() {
    let result_a = task_a(read_lines("input/day_9.txt"));
    assert_eq!(541, result_a);
    let result_b =  task_b(read_lines("input/day_9.txt"));
    assert_eq!(847504, result_b);
    println!("task-a: {}, task-b: {}", result_a, result_b);
}

fn task_a(lines: impl Iterator<Item=String>) -> usize {
    let grid = read_input(lines);
    find_low_points(&grid).iter().map(|(_, _,v)|v+1).sum()
}

fn task_b(lines: impl Iterator<Item=String>) -> usize {
    let grid = read_input(lines);
    let low_points = find_low_points(&grid);
    low_points.into_iter().map(|p| {
        let mut checked = HashSet::new();
        checked.insert((p.0, p.1));
        let unchecked = larger_neighbours(&grid, p.0, p.1).into_iter()
            .filter(|(x, y)| grid.fields[*x][*y] < 9).collect();
        basin_size(&grid, checked, unchecked)
    }).sorted().rev().take(3).product()
}

fn basin_size(grid: &Grid, checked: HashSet<(usize, usize)>, unchecked: HashSet<(usize, usize)>) -> usize {
    let new_unchecked = unchecked.iter()
        .map(|field| {
            larger_neighbours(grid, field.0, field.1).into_iter()
                .filter(|f| !checked.contains(f) && grid.fields[f.0][f.1] < 9)
                .collect::<HashSet<_>>()
        }).flatten().collect::<HashSet<_>>();

    let new_checked = checked.union(&unchecked).map(|f|*f).collect::<HashSet<_>>();

    if new_unchecked.len() == 0 {
        new_checked.len()
    } else {
        basin_size(grid, new_checked, new_unchecked)
    }
}

fn find_low_points(grid: &Grid) -> Vec<(usize, usize, usize)> {
    let mut low_points = vec![];
    for x in 0..grid.width {
        for y in 0..grid.height {
            let center = grid.fields[x][y];
            if neighbours(grid, x, y).into_iter().all(|(_, _, value)| value > center) {
                low_points.push((x, y, center))
            }
        }
    }
    low_points
}

fn larger_neighbours(grid: &Grid, x: usize, y: usize) -> HashSet<(usize, usize)> {
    let center = grid.fields[x][y];
    neighbours(grid, x, y).into_iter()
        .filter_map(|(x, y, value)|
            if value > center {Some((x, y))} else {None}).collect()
}

fn neighbours(grid: &Grid, x: usize, y: usize) -> Vec<(usize, usize, usize)> {
    vec![(x as isize - 1, y as isize), ((x+1) as isize, y as isize),
         (x as isize, y as isize - 1), (x as isize, (y+1) as isize)]
        .into_iter().filter_map(|(x, y)| {
            if x<0 || x >= grid.width as isize || y<0 || y >= grid.height as isize
            {None} else {Some((x as usize,y as usize))}
        }).map(|(x,y)| (x, y, grid.fields[x][y]))
        .collect()
}

struct Grid {fields: Vec<Vec<usize>>, width: usize, height: usize}

fn read_input(lines: impl Iterator<Item=String>) -> Grid {
    let fields =lines.map(|line| line.chars()
        .map(|c| c.to_digit(10).expect("failed to parse") as usize).collect_vec()
    ).collect_vec();
    let (width, height) = (fields.len(), fields[0].len());
    Grid{fields, height, width}
}