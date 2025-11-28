use std::{collections::HashMap, fs::read_to_string, iter::zip};

use regex::Regex;

fn main() {
    println!("ans for part1 test: {}", part1("../input/day1_test.txt"));
    println!("ans for part1: {}", part1("../input/day1.txt"));
    println!("ans for part2 test: {}", part2("../input/day1_test.txt"));
    println!("ans for part2: {}", part2("../input/day1.txt"));
}

fn parse_input(input_file: &str) -> (Vec<i32>, Vec<i32>) {
    let mut seq1 = vec![];
    let mut seq2 = vec![];
    let s = read_to_string(input_file).unwrap();
    let re = Regex::new(r"\d+").unwrap();

    for line in s.lines() {
        let nums: Vec<i32> = re
            .find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        seq1.push(nums[0]);
        seq2.push(nums[1]);
    }
    (seq1, seq2)
}

fn part1(input_file: &str) -> i32 {
    let (mut seq1, mut seq2) = parse_input(input_file);
    seq1.sort();
    seq2.sort();
    zip(seq1, seq2).map(|(a, b)| (a - b).abs()).sum()
}

fn part2(input_file: &str) -> u64 {
    let (seq1, seq2) = parse_input(input_file);
    let mut counter: HashMap<i32, u64> = HashMap::new();
    for num in seq2 {
        *counter.entry(num).or_default() += 1;
    }
    let mut ans = 0;
    for num in seq1 {
        let freq = counter.get(&num).unwrap_or(&0);
        ans += num as u64 * freq;
    }
    ans
}
