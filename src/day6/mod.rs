use std::collections::HashMap;

test_each_file! { for ["in", "out"] in "./src/day6/input" as test1 => test1 }
test_each_file! { for ["in", "out2"] in "./src/day6/input" as test2 => test2 }

fn part1(input: &str) -> usize {
    let mut grid = parse_input(input);

    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut direction_index = 0;
    let mut guard_pos = *grid.iter().find(|(_, value)| **value == '^').unwrap().0;
    _ = grid.insert(guard_pos, 'X');
    loop {
        let next_coordinate = (
            guard_pos.0 + directions[direction_index].0,
            guard_pos.1 + directions[direction_index].1,
        );

        match grid.get(&next_coordinate) {
            None => break,
            Some('#') => {
                if direction_index == 3 {
                    direction_index = 0;
                } else {
                    direction_index += 1;
                }
            }
            Some('X') => guard_pos = next_coordinate,
            Some(_) => {
                _ = grid.insert(next_coordinate, 'X');
                guard_pos = next_coordinate;
            }
        }
    }

    grid.iter().filter(|(_, value)| **value == 'X').count()
}

fn part2(input: &str) -> usize {
    1
}

fn parse_input(input: &str) -> HashMap<(i32, i32), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
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
