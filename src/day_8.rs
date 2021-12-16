mod lib;

use std::collections::{HashMap, HashSet};
use std::usize;
use itertools::Itertools;
use crate::lib::{read_lines};

fn main() {
    let result_a = task_a(read_lines("input/day_8.txt"));
    assert_eq!(239, result_a);
    let result_b =  task_b(read_lines("input/day_8.txt"));
    assert_eq!(946346, result_b);
    println!("task-a: {}, task-b: {}", result_a, result_b);
}

fn task_a(mut lines: impl Iterator<Item=String>) -> usize {
    let notes = read_input(&mut lines);
    notes.iter().map(|Note{signals: _, outputs: o}| o).flatten().map(|s|s.len())
        .filter(|length| [2, 3, 4, 7].contains(length))
        .count()
}

fn task_b(mut lines: impl Iterator<Item=String>) -> usize {
    let notes = read_input(&mut lines);
      notes.iter()
          .map(|note| (note, analyze_signals(note)))
          .map(|(note, decoder)|
              note.outputs.iter().map(|o| decoder.get(o).expect("decoder incomplete"))
                  .join(""))
    .map(|s| s.parse::<usize>().expect("parsing decoded value failed"))
    .sum()
}


fn analyze_signals(note: &Note) -> HashMap<String, char> {
    let signals = &note.signals;
    let digit_1 = signals.iter().find(|s| s.len() == 2).expect("not found");
    let digit_4 = signals.iter().find(|s| s.len() == 4).expect("not found");
    let digit_7 = signals.iter().find(|s| s.len() == 3).expect("not found");
    let digit_8 = signals.iter().find(|s| s.len() == 7).expect("not found");
    let digit_2_3_5 = signals.iter().filter(|s| s.len() == 5).collect_vec();
    let digit_6_9_0 = signals.iter().filter(|s| s.len() == 6).collect_vec();
    let digit_3 = *digit_2_3_5.iter().filter(|d| (**d - digit_1).len() == 3).next().expect("not found");
    let digit_2 = *digit_2_3_5.iter().filter(|d| (**d - digit_4).len() == 3).next().expect("not found");
    let digit_5 = *digit_2_3_5.iter().filter(|d| (**d - digit_2).len() == 2).next().expect("not found");
    let digit_9 = *digit_6_9_0.iter().filter(|d| (**d - digit_4).len() == 2).next().expect("not found");
    let digit_6 = *digit_6_9_0.iter().filter(|d| (**d - digit_7).len() == 4).next().expect("not found");
    let digit_0 = *digit_6_9_0.iter().filter(|d|**d != digit_6 && **d != digit_9).next().expect("not found");

    let mut decoder = HashMap::new();
    decoder.insert(digit_0.iter().sorted().join(""), '0');
    decoder.insert(digit_1.iter().sorted().join(""), '1');
    decoder.insert(digit_2.iter().sorted().join(""), '2');
    decoder.insert(digit_3.iter().sorted().join(""), '3');
    decoder.insert(digit_4.iter().sorted().join(""), '4');
    decoder.insert(digit_5.iter().sorted().join(""), '5');
    decoder.insert(digit_6.iter().sorted().join(""), '6');
    decoder.insert(digit_7.iter().sorted().join(""), '7');
    decoder.insert(digit_8.iter().sorted().join(""), '8');
    decoder.insert(digit_9.iter().sorted().join(""), '9');
    //println!("decoder: {:?}", &decoder);
    assert_eq!(decoder.len(), 10);
    decoder
}


type Word = HashSet<char>;
struct Note{signals: Vec<Word>, outputs: Vec<String>}

fn read_input(lines: impl Iterator<Item=String>) -> Vec<Note> {
    lines
        .map(|line| line.split(" | ")
            .map(|s| s.split_whitespace().collect_vec())
            .collect_tuple()
            .map(|(signals, outputs)| Note {
                signals: signals.iter().map(|s|HashSet::from_iter(s.chars())).collect(),
                outputs: outputs.iter().map(|s|s.chars().sorted().join("")).collect()
            }).expect("failed parsing note"))
        .collect_vec()
}
