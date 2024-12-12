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
    let mut laid_out_memory: Vec<(Option<usize>, usize)> = parse_input(input)
        .into_iter()
        .enumerate()
        .map(|(i, amount)| match i % 2 == 0 {
            true => (Some(i / 2), amount as usize),
            false => (None, amount as usize),
        })
        .collect();

    let mut reordered_memory_rev = vec![];

    while let Some(front_value) = laid_out_memory.pop() {
        match front_value.0 {
            Some(_) => {
                //find empty space
                let x = laid_out_memory
                    .iter()
                    .enumerate()
                    .find(|(index, &(v, length))| v.is_none() && length >= front_value.1);

                match x {
                    None => reordered_memory_rev.push(front_value),
                    Some((i, (_, length))) => {
                        reordered_memory_rev.push((None, front_value.1));
                        let remaining_space = length - front_value.1;
                        match remaining_space {
                            0 => laid_out_memory.splice(i..i + 1, vec![front_value]),
                            y => laid_out_memory.splice(i..i + 1, vec![front_value, (None, y)]),
                        };
                    }
                }
            }
            None => reordered_memory_rev.push(front_value),
        }
    }

    let mut sum = 0;
    let mut index = 0;
    reordered_memory_rev
        .iter()
        .rev()
        .for_each(|&(value, size)| {
            if let Some(id) = value {
                for i in index..index + size {
                    sum += i * id
                }
            }
            index += size
        });

    sum
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

//[(Some(0), 2), (Some(9), 2), (Some(2), 1), (None, 1), (Some(7), 3), (Some(4), 2), (None, 1), (Some(3), 3), (Some(1), 3), (None, 3), (None, 3), (None, 3), (None, 1), (None, 3), (None, 3), (None, 1), (None, 2), (None, 1), (Some(5), 4), (None, 1), (Some(6), 4), (None, 1), (None, 3), (None, 1), (Some(8), 4), (None, 0), (None, 2)]
//00992.77744.333111..................
