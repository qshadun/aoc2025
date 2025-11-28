#![allow(clippy::collapsible_else_if)]
use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let day = 19;
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

fn parse_input(input_file: &str) -> (HashSet<String>, Vec<String>) {
    let s = read_to_string(input_file).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    let patterns: HashSet<String> = lines[0].split(", ").map(|x| x.to_string()).collect();
    let designs: Vec<String> = lines.into_iter().skip(2).map(|x| x.to_string()).collect();
    (patterns, designs)
}
fn recur(i: usize, desgin: &str, patterns: &HashSet<String>, memo: &mut [i32]) -> bool {
    if i == desgin.len() {
        return true;
    }
    if memo[i] != -1 {
        return memo[i] == 1;
    }
    for j in i + 1..desgin.len() + 1 {
        if patterns.contains(&desgin[i..j]) && recur(j, desgin, patterns, memo) {
            memo[i] = 1;
            return true;
        }
    }
    memo[i] = 0;
    false
}

fn possible(design: &str, patterns: &HashSet<String>) -> bool {
    let mut memo = vec![-1; design.len()];
    recur(0, design, patterns, &mut memo)
}

fn part1(input_file: &str) -> usize {
    let (patterns, designs) = parse_input(input_file);
    let mut ans = 0;
    for design in designs.iter() {
        if possible(design, &patterns) {
            ans += 1;
        }
    }
    ans
}

fn recur_ways(i: usize, desgin: &str, patterns: &HashSet<String>, memo: &mut [i64]) -> i64 {
    if i == desgin.len() {
        return 1;
    }
    if memo[i] != -1 {
        return memo[i];
    }
    let mut ans = 0;
    for j in i + 1..desgin.len() + 1 {
        if patterns.contains(&desgin[i..j]) {
            ans += recur_ways(j, desgin, patterns, memo)
        }
    }
    memo[i] = ans;
    ans
}

fn ways(design: &str, patterns: &HashSet<String>) -> i64 {
    let mut memo = vec![-1; design.len()];
    recur_ways(0, design, patterns, &mut memo)
}

fn part2(input_file: &str) -> i64 {
    let (patterns, designs) = parse_input(input_file);
    let mut ans = 0;
    for design in designs.iter() {
        ans += ways(design, &patterns)
    }
    ans
}
