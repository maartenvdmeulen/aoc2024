use std::collections::HashMap;
use std::collections::VecDeque;

test_each_file! { for ["in", "out"] in "./src/day11/input" as test1 => test1 }
test_each_file! { for ["in", "out2"] in "./src/day11/input" as test2 => test2 }

fn part1(input: &str) -> usize {
    blinking_with_queue(input, 25)
}

fn part2(input: &str) -> usize {
    blinking_with_queue(input, 75)
}

fn blinking(input: &str, blinks: usize) -> usize {
    let mut stones = parse_input(input);
    let mut transform_set: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..blinks {
        let mut new_stones = Vec::with_capacity(stones.len() * 2);

        for stone in stones {
            match transform_set.get(&stone) {
                Some(transformed_stone) => new_stones.extend(transformed_stone.clone()),
                None => {
                    let transformed_stone = transform_stone(&stone);
                    transform_set.insert(stone, transformed_stone.clone());

                    new_stones.extend(transformed_stone);
                }
            }
        }

        stones = new_stones
    }

    stones.len()
}

fn blinking_with_queue(input: &str, blinks: usize) -> usize {
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.extend(parse_input(input));

    let mut transform_set: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..blinks {
        if i % 5 == 0 {
            println!("iter {:#} of {:#}", i, blinks);
        }

        let batch_size = queue.len();

        for _ in 0..batch_size {
            if let Some(stone) = queue.pop_front() {
                match transform_set.get(&stone) {
                    Some(transformed_stone) => queue.extend(transformed_stone.clone()),
                    None => {
                        let transformed_stone = transform_stone(&stone);
                        transform_set.insert(stone, transformed_stone.clone());
                        queue.extend(transformed_stone);
                    }
                }
            }
        }
    }

    queue.len()
}

fn transform_stone(stone: &String) -> Vec<String> {
    match (stone.as_str(), stone.len() % 2 == 0) {
        ("0", _) => return vec!["1".to_string()],
        (_, true) => {
            let (stone1, stone2) = stone.split_at(stone.len() / 2);
            return vec![
                stone1.parse::<isize>().unwrap().to_string(),
                stone2.parse::<isize>().unwrap().to_string(),
            ];
        }
        (_, _) => return vec![(stone.parse::<isize>().unwrap() * 2024).to_string()],
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
