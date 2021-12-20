mod lib;

use std::usize;
use itertools::Itertools;
use crate::lib::read_lines;

fn main() {
    let (result_a, result_b) = task_a_and_b(read_lines("input/day_4.txt"));
    assert_eq!(result_a, 10680);
    assert_eq!(result_b, 31892);
    println!("task-a: {}, task-b: {}", result_a, result_b);
}

fn task_a_and_b(lines: impl Iterator<Item=String>) -> (usize, usize) {
    let mut first_winning_board_score = 0;
    let mut last_winning_board_score = 0;
    let mut won_board_count = 0;


    let (numbers, mut boards) = parse_input(lines);
    println!("{} numbers and {} boards", numbers.len(), boards.len());
    let mut board_count = boards.len();
        for number in numbers.iter() {
        println!("drawing number {}", number);
        for board in boards.iter_mut() {
            if !board.2 {
                assert_eq!(board.0.len(), 5);
                for i in 0..5 {
                    assert_eq!(board.0[i].len(), 5);
                    for j in 0..5 {
                        if board.0[i][j].0 == *number {
                            board.0[i][j].1 = true;
                            board.1[0][i] += 1;
                            board.1[1][j] += 1;
                        }
                        if !board.2 && (board.1[0][i] == 5 || board.1[1][j] == 5) {
                            println!("winning_board #{}", won_board_count);
                            if won_board_count == 0 {
                                first_winning_board_score = calculate_score(board, *number);
                            } else if won_board_count == board_count -1 {
                                last_winning_board_score = calculate_score(board, *number);
                                return (first_winning_board_score, last_winning_board_score);
                            }
                            won_board_count += 1;
                            board.2 = true;
                        }
                    }
                }
            }
            print_board(&board);
        }
    };

    panic!("no solution found");
}

type Field = (usize, bool);
type Board = (Vec<Vec<Field>>, Vec<Vec<usize>>, bool);

fn calculate_score(board: &Board, winning_number: usize) -> usize {
    board.0.iter().map(|row|
        row.iter()
            .filter(|(_, marked)| !*marked)
            .map(|(value, _)| *value)
            .sum::<usize>()
    ).sum::<usize>() * winning_number
}

fn parse_input(mut lines: impl Iterator<Item=String>) -> (Vec<usize>, Vec<Board>) {
    let numbers = parse_numbers(&mut lines);
    let mut boards = vec![];
    while lines.next().is_some() {
        boards.push(parse_board(&mut lines));
    }
    (numbers, boards)
}

fn parse_numbers(lines: &mut impl Iterator<Item=String>) ->  Vec<usize> {
    lines.next()
        .expect("line with numbers not found")
        .split(",")
        .map(|s| s.parse::<usize>().expect("parsing number failed"))
        .collect()
}

fn parse_board(lines: &mut impl Iterator<Item=String>) -> Board {
    (lines
        .take(5)
        .map(|line| line
            .split_whitespace()
            .map(|s|s.parse::<usize>().expect("parse board value failed"))
            .map(|value| (value, false))
            .collect())
        .collect(), vec![vec![0;5];2], false)
}

fn print_board(board: &Board) {
    board.0.iter().for_each(|row| {
       println!("{}", &row.iter().map(|(value, marked)| format!("{: >2} {: >2}", value, if *marked {"X"} else {"_"})).join(" | "))
    });

    board.1.iter().for_each(|axis| {
        println!("{}", &axis.iter().map(|count| format!("{: >5}", count)).join(" | "))
    });
    println!()
}
