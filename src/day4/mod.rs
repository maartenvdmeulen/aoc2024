use std::collections::HashMap;

test_each_file! { for ["in", "out"] in "./src/day4/input" as test1 => test1 }
test_each_file! { for ["in", "out2"] in "./src/day4/input" as test2 => test2 }

fn part1(input: &str) -> usize {
    let grid: HashMap<(i32, i32), char> = parse_input(input);

    let directions: [[(i32, i32); 3]; 8] = [
        [(0, 1), (0, 2), (0, 3)],
        [(0, -1), (0, -2), (0, -3)],
        [(1, 0), (2, 0), (3, 0)],
        [(-1, 0), (-2, 0), (-3, 0)],
        [(1, 1), (2, 2), (3, 3)],
        [(-1, -1), (-2, -2), (-3, -3)],
        [(1, -1), (2, -2), (3, -3)],
        [(-1, 1), (-2, 2), (-3, 3)],
    ];

    let mas = ['M', 'A', 'S'];

    grid.iter()
        .filter(|(position, c)| **c == 'X')
        .fold(0, |acc, (pos, c)| {
            acc + directions
                .iter()
                .filter(|dir| {
                    dir.iter().zip(mas).all(|(dir_step, mas_step_c)| {
                        grid.get(&(pos.0 + dir_step.0, pos.1 + dir_step.1))
                            .map_or(false, |c| *c == mas_step_c)
                    })
                })
                .count()
        })
}

fn part2(input: &str) -> usize {
    let grid: HashMap<(i32, i32), char> = parse_input(input);

    let directions: [[(i32, i32); 4]; 4] = [
        [(-1, -1), (1, -1), (1, 1), (-1, 1)],
        [(-1, 1), (-1, -1), (1, -1), (1, 1)],
        [(1, 1), (-1, 1), (-1, -1), (1, -1)],
        [(1, -1), (1, 1), (-1, 1), (-1, -1)],
    ];

    let mas = ['M', 'M', 'S', 'S'];

    grid.iter()
        .filter(|(position, c)| **c == 'A')
        .fold(0, |acc, (pos, c)| {
            acc + directions
                .iter()
                .filter(|dir| {
                    dir.iter().zip(mas).all(|(dir_step, mas_step_c)| {
                        grid.get(&(pos.0 + dir_step.0, pos.1 + dir_step.1))
                            .map_or(false, |c| *c == mas_step_c)
                    })
                })
                .count()
        })
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
