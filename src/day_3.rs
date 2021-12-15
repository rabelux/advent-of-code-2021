mod lib;

use std::usize;
use itertools::Itertools;
use crate::lib::read_lines;

fn main() {
    let result_a = task_a(read_lines("input/day_3.txt"));
    assert_eq!(result_a, 3549854);
    let result_b = task_b(read_lines("input/day_3.txt"));
    assert_eq!(result_b, 3765399);
    println!("task-a: {}, task-b: {}", result_a, result_b);

}

fn task_a(lines: impl Iterator<Item=String>) -> usize {
    let all_numbers = read_all_numbers(lines);
    let majority = majority(&all_numbers);
    let gamma = binary_vec_to_usize(&majority);
    let epsilon = binary_invert(gamma);
    gamma * epsilon
}

fn task_b(lines: impl Iterator<Item=String>) -> usize {
    let all_numbers = read_all_numbers(lines);
    let oxygen = binary_vec_to_usize(&filter_numbers(&all_numbers, majority));
    let scrubber = binary_vec_to_usize(&filter_numbers(&all_numbers, minority));
    oxygen * scrubber
}


fn filter_numbers(numbers: &Vec<Vec<usize>>, criteria_fun: fn(&Vec<Vec<usize>>)->Vec<usize>) -> Vec<usize> {
    let mut remaining_numbers = numbers.clone();
    let mut index = 0;
    while remaining_numbers.len() > 1 {
        let minority = criteria_fun(&remaining_numbers);
        let criteria = minority[index];
        remaining_numbers.retain(|n|n[index] == criteria);
        index = index + 1;
    }
    remaining_numbers[0].clone()
}


fn binary_invert(number: usize) -> usize {
    !number & (1 << 12) - 1
}


fn majority(numbers: &Vec<Vec<usize>>) -> Vec<usize> {
    let half = numbers.len() as f64 / 2.0;
    let bit_sum = sum_bits(numbers.iter());
    bit_sum.iter().map(|bit_sum| if (*bit_sum as f64) >= half {1} else {0}).collect()
}

fn minority(numbers: &Vec<Vec<usize>>) -> Vec<usize> {
    let half = numbers.len()  as f64 / 2.0;
    let bit_sum = sum_bits(numbers.iter());
    bit_sum.iter().map(|bit_sum| if (*bit_sum as f64) < half {1} else {0}).collect()
}

fn binary_vec_to_usize(s: &Vec<usize>) -> usize {
    let binary_string = s.iter().join("");
    usize::from_str_radix(&binary_string, 2).expect("failed to convert epsilon")
}

fn read_all_numbers(lines: impl Iterator<Item=String>) -> Vec<Vec<usize>> {
    lines
        .map(|line|
            line.chars()
                .map(|c| c.to_digit(2).expect("parse failed") as usize)
                .collect_vec()
        ).collect_vec()
}

fn sum_bits<'a>(numbers: impl Iterator<Item=&'a Vec<usize>>) -> Vec<usize> {
    numbers.fold(vec![0; 12], | bit_sum, bits|
        bit_sum.iter()
            .zip(bits.iter())
            .map(|(a, b)| a + b)
            .collect_vec()
    )
}





