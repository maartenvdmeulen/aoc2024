use std::collections::HashMap;

test_each_file! { for ["in", "out"] in "./src/day11/input" as test1 => test1 }
test_each_file! { for ["in", "out2"] in "./src/day11/input" as test2 => test2 }

fn part1(input: &str) -> usize {
    blinking(input, 25)
}

fn part2(input: &str) -> usize {
    blinking(input, 75)
}

fn blinking(input: &str, blinks: usize) -> usize {
    let mut stone_map: HashMap<String, usize> = HashMap::new();
    let mut transform_set: HashMap<String, Vec<String>> = HashMap::new();

    let stones = parse_input(input);
    stones
        .into_iter()
        .for_each(|stone| *stone_map.entry(stone).or_insert(0) += 1);

    (0..blinks).for_each(|_| {
        let mut next_stone_map: HashMap<String, usize> = HashMap::new();

        stone_map.iter().for_each(|(stone, amount)| {
            transform_set
                .entry(stone.to_string())
                .or_insert(transform_stone(&stone))
                .iter()
                .for_each(|t_stone| {
                    *next_stone_map.entry(t_stone.to_string()).or_insert(0) += amount
                })
        });

        stone_map = next_stone_map;
    });

    stone_map.values().sum()
}

fn transform_stone(stone: &String) -> Vec<String> {
    match stone.as_str() {
        "0" => return vec!["1".to_string()],
        _ if stone.len() % 2 == 0 => {
            let (stone1, stone2) = stone.split_at(stone.len() / 2);
            return vec![
                stone1.parse::<isize>().unwrap().to_string(),
                stone2.parse::<isize>().unwrap().to_string(),
            ];
        }
        _ => return vec![(stone.parse::<isize>().unwrap() * 2024).to_string()],
    }
}

fn parse_input(input: &str) -> Vec<String> {
    input.split_whitespace().map(|x| x.to_string()).collect()
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
