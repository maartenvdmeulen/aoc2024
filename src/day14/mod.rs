test_each_file! { for ["in", "out"] in "./src/day14/input" as test1 => test1 }
test_each_file! { for ["in", "out2"] in "./src/day14/input" as test2 => test2 }

use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

fn part1(input: &str) -> usize {
    let robot_plans = parse_input(input);

    let (wide, height) = match robot_plans.len() {
        12 => (11, 7),
        _ => (101, 103),
    };
    const SECONDS: i32 = 100;

    let mut final_map: HashMap<(i32, i32), usize> = HashMap::new();

    robot_plans.into_iter().for_each(|((px, py), (vx, vy))| {
        let px2 = (px + vx * SECONDS).rem_euclid(wide);
        let py2 = (py + vy * SECONDS).rem_euclid(height);

        *final_map.entry((px2, py2)).or_insert(0) += 1;
    });

    let mut q = (0, 0, 0, 0);

    final_map.into_iter().for_each(|((x, y), amount)| {
        if x < wide / 2 {
            if y < height / 2 {
                q.0 += amount;
            } else if y > height / 2 {
                q.1 += amount;
            }
        } else if x > wide / 2 {
            if y < height / 2 {
                q.2 += amount;
            } else if y > height / 2 {
                q.3 += amount;
            }
        }
    });

    q.0 * q.1 * q.2 * q.3
}

fn part2(input: &str) -> usize {
    let mut robot_plans = parse_input(input);

    let (wide, height) = (101, 103);
    const SECONDS: i32 = 100;

    for i in 1.. {
        let mut final_map: HashSet<(i32, i32)> = HashSet::new();

        robot_plans.iter_mut().for_each(|((px, py), (vx, vy))| {
            *px = (*px + *vx).rem_euclid(wide);
            *py = (*py + *vy).rem_euclid(height);

            final_map.insert((*px, *py));
        });

        if final_map.len() == robot_plans.len() {
            return i;
        }
    }

    panic!()
}

fn parse_input(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    let mut parser = separated_list1(
        line_ending::<&str, ()>,
        separated_pair(
            preceded(
                tag("p="),
                separated_pair(complete::i32, tag(","), complete::i32),
            ),
            space1,
            preceded(
                tag("v="),
                separated_pair(complete::i32, tag(","), complete::i32),
            ),
        ),
    );

    let (_, rows) = parser(input).unwrap();

    rows
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
