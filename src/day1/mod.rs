use std::collections::HashMap;

test_each_file! { for ["in", "out"] in "./src/day1/input" as test1 => test1 }
test_each_file! { for ["in", "out2"] in "./src/day1/input" as test2 => test2 }

fn part1(input: &str) -> u64 {
    let i = parse_input(input);
    let (mut v1, mut v2): (Vec<_>, Vec<_>) = i.into_iter().unzip();
    v1.sort();
    v2.sort();

    v1.iter()
        .zip(v2.iter())
        // .collect::<Vec<(&u8, &u8)>>()
        // .iter()
        .map(|(x, y)| x.abs_diff(*y))
        .sum()
}

fn part2(input: &str) -> u64 {
    let i = parse_input(input);
    let (v1, v2): (Vec<_>, Vec<_>) = i.into_iter().unzip();

    let mut counter: HashMap<u64, u64> = HashMap::new();

    for amount in v2 {
        *counter.entry(amount).or_insert(0) += 1;
    }

    v1.iter()
        .map(|num| *counter.entry(*num).or_insert(0) * *num)
        .sum()
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    // let x = input
    //     .split("\r\n")
    //     .map(|i| i.split(" ").map(|n| n.parse().unwrap()));
    // let list1 = x.map(|y| y[0]).collect();
    // let list2 = x.map(|y| y[1]).collect();
    input
        .lines()
        .map(|n| n.split_whitespace().collect())
        .map(|n: Vec<_>| (n[0].parse().unwrap(), n[1].parse().unwrap()))
        .collect()
}

fn parse_output(output: &str) -> u64 {
    output.parse().unwrap()
}

fn test1([input, output]: [&str; 2]) {
    assert_eq!(part1(input), parse_output(output))
}

fn test2([input, output]: [&str; 2]) {
    assert_eq!(part2(input), parse_output(output))
}
