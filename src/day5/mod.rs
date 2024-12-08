use itertools::Itertools;
use std::{cmp::Ordering, collections::HashSet};

test_each_file! { for ["in", "out"] in "./src/day5/input" as test1 => test1 }
test_each_file! { for ["in", "out2"] in "./src/day5/input" as test2 => test2 }

fn part1(input: &str) -> usize {
    let (rules, tests) = parse_input(input);

    let tests: Vec<Vec<_>> = tests
        .into_iter()
        .filter(|test| {
            test.iter()
                .cloned()
                .tuple_windows()
                .all(|x| rules.get(&x).is_some())
        })
        .collect();

    tests
        .iter()
        .map(|test| test.get(test.len() / 2).unwrap())
        .sum()
}

fn part2(input: &str) -> usize {
    let (rules, tests) = parse_input(input);

    let aaa: Vec<Vec<_>> = tests
        .into_iter()
        .filter(|test| {
            test.iter()
                .cloned()
                .tuple_windows()
                .any(|x| rules.get(&x).is_none())
        })
        .collect();

    let bbb: Vec<Vec<_>> = aaa
        .into_iter()
        .map(|mut x| {
            x.sort_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    return Ordering::Less;
                } else if rules.contains(&(*b, *a)) {
                    return Ordering::Greater;
                } else {
                    return Ordering::Equal;
                }
            });
            x
        })
        .collect();

    bbb.iter()
        .map(|test| test.get(test.len() / 2).unwrap())
        .sum()
}

fn parse_input(input: &str) -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
    let (rules, tests) = input.split_once("\r\n\r\n").unwrap();

    let rules: HashSet<(usize, usize)> = rules
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    let tests: Vec<Vec<usize>> = tests
        .lines()
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    (rules, tests)
}

fn parse_output(output: &str) -> usize {
    output.parse().unwrap()
}

fn test1([input, output]: [&str; 2]) {
    assert_eq!(part1(input), parse_output(output))
}

fn test2([input, output]: [&str; 2]) {
    assert_eq!(part2(input), parse_output(output))
}
