use glam::IVec2;
use pathfinding::prelude::{astar_bag, dijkstra};
use std::collections::HashSet;

test_each_file! { for ["in", "out"] in "./src/day16/input" as test1 => test1 }
test_each_file! { for ["in", "out2"] in "./src/day16/input" as test2 => test2 }

fn part1(input: &str) -> usize {
    let maze = parse_input(input);
    let start = maze.iter().find(|(pos, m)| *m == Maze::Start).unwrap().0;
    let end = maze.iter().find(|(pos, m)| *m == Maze::End).unwrap().0;
    let maze_whales: HashSet<_> = maze
        .iter()
        .filter(|(pos, m)| *m == Maze::Wall)
        .map(|x| x.0)
        .collect();

    let result = dijkstra(
        &(start, IVec2::X),
        |(position, direction)| {
            let mut next: Vec<((IVec2, IVec2), usize)> = vec![];
            if maze_whales.get(&(position + direction)).is_none() {
                next.push(((position + direction, *direction), 1 as usize));
            };
            next.push(((*position, direction.perp()), 1000 as usize));
            next.push(((*position, -direction.perp()), 1000 as usize));
            return next;
        },
        |&(p, _)| p == end,
    );

    result.expect("shortest path found").1
}

fn part2(input: &str) -> usize {
    let maze = parse_input(input);
    let start = maze.iter().find(|(pos, m)| *m == Maze::Start).unwrap().0;
    let end = maze.iter().find(|(pos, m)| *m == Maze::End).unwrap().0;
    let maze_whales: HashSet<_> = maze
        .iter()
        .filter(|(pos, m)| *m == Maze::Wall)
        .map(|x| x.0)
        .collect();

    let result = astar_bag(
        &(start, IVec2::X),
        |(position, direction)| {
            let mut next: Vec<((IVec2, IVec2), usize)> = vec![];
            if maze_whales.get(&(position + direction)).is_none() {
                next.push(((position + direction, *direction), 1 as usize));
            };
            next.push(((*position, direction.perp()), 1000 as usize));
            next.push(((*position, -direction.perp()), 1000 as usize));
            return next;
        },
        |_| 0,
        |&(p, _)| p == end,
    );

    result
        .expect("shortest path found")
        .0
        .flat_map(|node| node.into_iter().map(|x| x.0))
        .collect::<HashSet<IVec2>>()
        .len()
}

#[derive(PartialEq, Eq, Hash)]
enum Maze {
    Start,
    End,
    Node,
    Wall,
}

fn parse_input(input: &str) -> HashSet<(IVec2, Maze)> {
    let maze: HashSet<(IVec2, Maze)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, c)| {
                let pos = IVec2::new(x as i32, y as i32);
                return match c {
                    'S' => vec![(pos, Maze::Start)],
                    'E' => vec![(pos, Maze::End)],
                    '#' => vec![(pos, Maze::Wall)],
                    '.' => vec![(pos, Maze::Node)],
                    _ => panic!(),
                };
            })
        })
        .collect();

    maze
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
