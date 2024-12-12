use std::collections::VecDeque;

test_each_file! { for ["in", "out"] in "./src/day9/input" as test1 => test1 }
test_each_file! { for ["in", "out2"] in "./src/day9/input" as test2 => test2 }

fn part1(input: &str) -> usize {
    let mut laid_out_memory: VecDeque<Option<usize>> = parse_input(input)
        .into_iter()
        .enumerate()
        .flat_map(|(i, amount)| match i % 2 == 0 {
            true => vec![Some(i / 2); amount as usize],
            false => vec![None; amount as usize],
        })
        .collect();

    let mut moved_memory = Vec::new();

    while let Some(front_value) = laid_out_memory.pop_front() {
        match front_value {
            None => {
                while let Some(back_value) = laid_out_memory.pop_back() {
                    if let Some(b_v) = back_value {
                        moved_memory.push(b_v);
                        break;
                    }
                }
            }
            Some(v) => moved_memory.push(v),
        }
    }

    moved_memory
        .into_iter()
        .enumerate()
        .map(|(i, v)| i * v)
        .sum()
}

fn part2(input: &str) -> usize {
    1
}

fn parse_input(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
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
