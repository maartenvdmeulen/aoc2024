use std::usize;

use itertools::Itertools;

test_each_file! { for ["in", "out"] in "./src/day2/input" as test1 => test1 }
test_each_file! { for ["in", "out2"] in "./src/day2/input" as test2 => test2 }

fn part1(input: &str) -> usize {
    let lines = parse_input(input);
    lines
        .iter()
        .filter(|elements| {
            let is_increasing = elements[0] < elements[1];

            elements.iter().tuple_windows().all(|(a, b)| {
                (*a < *b) == is_increasing && a.abs_diff(*b) <= 3 && a.abs_diff(*b) != 0
            })
        })
        .count()
}

fn part2(input: &str) -> usize {
    let lines = parse_input(input);

    lines
        .iter()
        .filter(|elements| {
            (0..elements.len()).any(|remove_index| {
                let elements: Vec<_> = elements
                    .iter()
                    .enumerate()
                    .filter(|(j, v)| *j != remove_index)
                    .map(|(_, v)| v)
                    .collect();

                let is_increasing = elements[0] < elements[1];

                elements.iter().tuple_windows().all(|(a, b)| {
                    (*a < *b) == is_increasing && a.abs_diff(**b) <= 3 && a.abs_diff(**b) != 0
                })
            })
        })
        .count()
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<u64>().unwrap())
                .collect()
        })
        .collect()
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
