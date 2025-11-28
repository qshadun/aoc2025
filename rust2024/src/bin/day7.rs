use regex::Regex;
use std::{fs::read_to_string, num::ParseIntError};

fn main() {
    let day = 7;
    println!(
        "ans for part1 test: {}",
        part1(&format!("../input/day{}_test.txt", day))
    );
    println!(
        "ans for part1: {}",
        part1(&format!("../input/day{}.txt", day))
    );
    println!(
        "ans for part2 test: {}",
        part2(&format!("../input/day{}_test.txt", day))
    );
    println!(
        "ans for part2: {}",
        part2(&format!("../input/day{}.txt", day))
    );
}

fn parse_input(input_file: &str) -> Vec<Vec<u128>> {
    let mut ans = vec![];
    let s = read_to_string(input_file).unwrap();
    let re = Regex::new(r"\d+").unwrap();

    for line in s.lines() {
        let nums: Vec<u128> = re
            .find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        ans.push(nums);
    }
    ans
}

fn recur(equation: &[u128], i: usize, sofar: u128) -> bool {
    if i == equation.len() {
        return sofar == equation[0];
    }
    recur(equation, i + 1, sofar + equation[i]) || recur(equation, i + 1, sofar * equation[i])
}

fn check(equation: &[u128]) -> bool {
    recur(equation, 2, equation[1])
}

fn part1(input_file: &str) -> u128 {
    let equations = parse_input(input_file);
    let mut ans = 0;
    for equation in equations {
        if check(&equation) {
            ans += equation[0];
        }
    }
    ans
}

fn combine(n1: u128, n2: u128) -> Result<u128, ParseIntError> {
    format!("{}{}", n1, n2).parse()
}

fn recur2(equation: &[u128], i: usize, sofar: u128) -> bool {
    if i == equation.len() {
        return sofar == equation[0];
    }
    let ans = recur2(equation, i + 1, sofar + equation[i])
        || recur2(equation, i + 1, sofar * equation[i]);
    if ans {
        true
    } else {
        let sofar = combine(sofar, equation[i]);
        match sofar {
            Ok(sofar) => sofar <= equation[0] && recur2(equation, i + 1, sofar),
            Err(_) => false,
        }
    }
}

fn check2(equation: &[u128]) -> bool {
    recur2(equation, 2, equation[1])
}

fn part2(input_file: &str) -> u128 {
    let equations = parse_input(input_file);
    let mut ans = 0;
    for equation in equations {
        if check2(&equation) {
            ans += equation[0];
        }
    }
    ans
}
