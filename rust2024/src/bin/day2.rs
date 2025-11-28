use std::fs::read_to_string;

use regex::Regex;

fn main() {
    println!("ans for part1 test: {}", part1("../input/day2_test.txt"));
    println!("ans for part1: {}", part1("../input/day2.txt"));
    println!("ans for part2 test: {}", part2("../input/day2_test.txt"));
    println!("ans for part2: {}", part2("../input/day2.txt"));
}

fn parse_input(input_file: &str) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let s = read_to_string(input_file).unwrap();
    let re = Regex::new(r"\d+").unwrap();

    for line in s.lines() {
        let nums: Vec<i32> = re
            .find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        ans.push(nums);
    }
    ans
}

fn is_safe(nums: &[i32]) -> bool {
    let incr = nums[0] < nums[1];
    for i in 0..nums.len() - 1 {
        if incr {
            if nums[i + 1] < nums[i] + 1 || nums[i + 1] > nums[i] + 3 {
                return false;
            }
        } else if nums[i + 1] < nums[i] - 3 || nums[i + 1] > nums[i] - 1 {
            return false;
        }
    }
    true
}

fn part1(input_file: &str) -> usize {
    let reports = parse_input(input_file);
    let mut ans = 0;
    for nums in reports.iter() {
        if is_safe(nums) {
            ans += 1;
        }
    }
    ans
}

fn is_safe2(nums: &[i32]) -> bool {
    if is_safe(nums) {
        return true;
    }
    for i in 0..nums.len() {
        let mut new_vec = vec![];
        new_vec.extend_from_slice(&nums[0..i]);
        new_vec.extend_from_slice(&nums[i + 1..]);
        if is_safe(&new_vec) {
            return true;
        }
    }
    false
}
fn part2(input_file: &str) -> u64 {
    let reports = parse_input(input_file);
    let mut ans = 0;
    for nums in reports.iter() {
        if is_safe2(nums) {
            ans += 1;
        }
    }
    ans
}
