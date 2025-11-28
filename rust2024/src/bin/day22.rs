#![allow(clippy::collapsible_else_if)]
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let day = 22;
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

fn parse_input(input_file: &str) -> Vec<i64> {
    let s = read_to_string(input_file).unwrap();
    s.lines().map(|x| x.parse().unwrap()).collect()
}

fn calc(secret: i64) -> i64 {
    let secret = ((secret * 64) ^ secret) % 16777216;
    let secret = ((secret / 32) ^ secret) % 16777216;
    ((secret * 2048) ^ secret) % 16777216
}

fn part1(input_file: &str) -> i64 {
    let seeds = parse_input(input_file);
    let mut ans = 0;
    for secret in seeds {
        let mut sec = secret;
        for _ in 0..2000 {
            sec = calc(sec);
        }
        ans += sec;
    }
    ans
}

fn calc_changes(secret: i64) -> (Vec<i8>, Vec<i8>) {
    let mut changes = vec![];
    let mut nums = vec![secret];
    for _ in 0..2000 {
        nums.push(calc(nums[nums.len() - 1]));
    }
    let prices: Vec<i8> = nums.iter().map(|x| (x % 10) as i8).collect();
    for i in 0..2000 {
        changes.push(prices[i + 1] - prices[i]);
    }
    (prices, changes)
}

type Pattern = (i8, i8, i8, i8);

fn pattern_to_prices(prices: &[i8], changes: &[i8]) -> HashMap<Pattern, i8> {
    let mut ans = HashMap::new();

    for i in 0..changes.len() - 3 {
        let pattern: Pattern = (changes[i], changes[i + 1], changes[i + 2], changes[i + 3]);
        ans.entry(pattern).or_insert(prices[i + 4]);
    }
    ans
}

fn part2(input_file: &str) -> i64 {
    let seeds = parse_input(input_file);
    let prices_changes: Vec<(Vec<i8>, Vec<i8>)> = seeds.into_iter().map(calc_changes).collect();
    let all_pattern_to_prices: Vec<HashMap<Pattern, i8>> = prices_changes
        .iter()
        .map(|(prices, changes)| pattern_to_prices(prices, changes))
        .collect();
    let mut all_patterns: HashSet<Pattern> = HashSet::new();
    for pc in all_pattern_to_prices.iter() {
        for p in pc.keys() {
            all_patterns.insert(*p);
        }
    }
    let mut ans = 0;
    let mut max_pattern: Option<Pattern> = None;
    for p in all_patterns.iter() {
        let v: i64 = all_pattern_to_prices
            .iter()
            .map(|pc| *pc.get(p).unwrap_or(&0) as i64)
            .sum();
        if v > ans {
            ans = v;
            max_pattern = Some(*p);
        }
    }
    println!("max pattern: {:?}", max_pattern.unwrap());
    ans
}
