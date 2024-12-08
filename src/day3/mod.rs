test_each_file! { for ["in", "out"] in "./src/day3/input" as test1 => test1 }
test_each_file! { for ["in2", "out2"] in "./src/day3/input" as test2 => test2 }

use regex::Regex;

fn part1(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    re.captures_iter(input)
        .map(|caps| {
            let (_, [x, y]) = caps.extract();
            x.parse::<usize>().unwrap() * y.parse::<usize>().unwrap()
        })
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    re.captures_iter(input)
        .map(|caps| match caps.get(0).unwrap().as_str() {
            "do()" => {
                enabled = true;
                0
            }
            "don't()" => {
                enabled = false;
                0
            }
            _ => {
                if !enabled {
                    0
                } else {
                    caps.iter()
                        .skip(1)
                        .map(|n| n.unwrap().as_str().parse::<usize>().unwrap())
                        .product()
                }
            }
        })
        .sum::<usize>()
}

// fn parse_input(input: &str) -> &str {
//     input
// }

fn parse_output(output: &str) -> usize {
    output.parse().unwrap()
}

fn test1([input, output]: [&str; 2]) {
    assert_eq!(part1(input), parse_output(output))
}

fn test2([input, output]: [&str; 2]) {
    assert_eq!(part2(input), parse_output(output))
}
