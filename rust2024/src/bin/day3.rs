use std::{fs::read_to_string, sync::LazyLock};

use regex::Regex;

fn main() {
    println!("ans for part1 test: {}", part1("../input/day3_test.txt"));
    println!("ans for part1: {}", part1("../input/day3.txt"));
    println!("ans for part2 test: {}", part2("../input/day3_test.txt"));
    println!("ans for part2: {}", part2("../input/day3.txt"));
}

fn parse_input(input_file: &str) -> String {
    read_to_string(input_file).unwrap()
}

fn calc(s: &str) -> u128 {
    let mut ans = 0;
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());
    for cap in RE.captures_iter(s) {
        let num1: u128 = cap[1].parse().unwrap();
        let num2: u128 = cap[2].parse().unwrap();
        ans += num1 * num2;
    }
    ans
}
fn part1(input_file: &str) -> u128 {
    let input = parse_input(input_file);
    calc(&input)
}

fn part2(input_file: &str) -> u128 {
    let input = parse_input(input_file);
    let mut start = 0;
    let mut ans = 0;
    while start < input.len() {
        match &input[start..].find("don't()") {
            Some(i) => {
                ans += calc(&input[start..start + i]);
                start += i;
                match &input[start..].find("do()") {
                    Some(i) => {
                        start += i;
                    }
                    None => break,
                }
            }
            None => {
                ans += calc(&input[start..]);
                break;
            }
        }
    }
    ans
}
