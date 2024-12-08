test_each_file! { for ["in", "out"] in "./src/day/input" as test1 => test1 }
test_each_file! { for ["in", "out2"] in "./src/day/input" as test2 => test2 }

fn part1(input: &str) -> usize {
    
    1
}

fn part2(input: &str) -> usize {

    1
}

fn parse_input(input: &str) -> () {
    input.split("\r\n")
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