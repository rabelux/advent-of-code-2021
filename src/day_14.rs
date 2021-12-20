mod lib;

use std::collections::HashMap;
use std::fmt::{Debug};
use std::time::Instant;
use itertools::{Either, Itertools, MinMaxResult};
use rayon::prelude::*;
use crate::lib::{read_lines};

fn main() {
    let result_a = task(read_lines("input/day_14.txt"), 10);
    assert_eq!(result_a, 2621);

    for i in 20..30 {
        let before = Instant::now();
        task(read_lines("input/day_14.txt"), i);
        println!("Elapsed time {}: {:.2?}", i, before.elapsed());
    }

    let result_b = task(read_lines("input/day_14.txt"), 40);
    println!("task-a: {}, task-b: {}", result_a, result_b);
}

fn task(lines: impl Iterator<Item=String>, steps: usize) -> usize {
    let (template, rules) = read_input(lines);
    let rule_map = rules.iter().map(|r|(r.0.clone(), r.clone())).collect::<HashMap<_,_>>();
    let seed = template.iter().tuple_windows().collect_vec().into_iter().map(|(a, b)|[*a, *b]);
    let mut stats = seed.collect_vec().par_iter()
        .map(|seed| {
            stream_apply(Box::new(vec![[seed[0], seed[1]]].into_iter()), &rule_map, steps)
                .map(|pair|pair[0])
                .fold(HashMap::new(), |mut stat,c| {
                    *stat.entry(c).or_insert(0) += 1;
                    stat
                })
    }).reduce_with(|mut a: HashMap<_, _>, b: HashMap<_, _>| {
        b.iter().for_each(|(k,v)| *a.entry(*k).or_insert(0) += v);
        a
    }).unwrap();

    *stats.entry(template[template.len()-1]).or_insert(0) += 1;
    score(&stats)
}

fn single_edit(pair: &[char; 2], rules: &HashMap<[char;2], Rule>) -> Vec<[char;2]>  {
    match rules.get(pair) {
        None => vec![pair.clone()],
        Some((_, insert)) => vec![[pair[0], *insert],[*insert, pair[1]]],
    }
}

fn stream_apply<'a>(pairs: Box<dyn Iterator<Item=[char;2]> + 'a>, rules: &'a HashMap<[char;2], Rule>, steps: usize)
    -> Box<dyn Iterator<Item=[char;2]>+'a>{
    let applied = Box::new(pairs
        .map(move |pair|single_edit(&pair, &rules))
        .flatten());

    if steps-1 > 0 {
        stream_apply(applied, &rules, steps-1)
    } else {
       applied
    }
}


fn score(stat: &HashMap<char,usize>) -> usize {
    let  result= stat.iter().minmax_by_key(|(_,count)|**count);
    match result {
        MinMaxResult::MinMax((_ , min), (_, max)) => max-min,
        _ => panic!("unexpected result")
    }
}

type Rule = ([char;2], char);

fn read_input(mut lines: impl Iterator<Item=String>) -> (Vec<char>, Vec<Rule>) {
    let template = lines.next().expect("invalid input").chars().collect_vec();
    let rules = lines.skip(1).map(|s| s
        .split(" -> ")
        .map(|s|s.chars().collect_vec())
        .collect_tuple()
        .map(|(p, i)| ([p[0], p[1]], i[0]))
        .expect("invalid input")
    ).collect_vec();
    (template, rules)
}