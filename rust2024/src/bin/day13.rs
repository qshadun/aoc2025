use std::{fs::read_to_string, sync::LazyLock};

use regex::Regex;

fn main() {
    let day = 13;
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

fn parse_nums(line: &str) -> Vec<i64> {
    static REGEX: LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(r"\d+").unwrap());
    REGEX
        .find_iter(line)
        .map(|x| x.as_str().parse().unwrap())
        .collect()
}
fn parse_input(input_file: &str) -> Vec<[i64; 6]> {
    let s = read_to_string(input_file).unwrap();
    let mut ans = vec![];
    let lines: Vec<&str> = s.lines().collect();
    for i in (0..lines.len()).step_by(4) {
        let xs: Vec<i64> = parse_nums(lines[i]);
        let ys: Vec<i64> = parse_nums(lines[i + 1]);
        let zs: Vec<i64> = parse_nums(lines[i + 2]);
        ans.push([xs[0], xs[1], ys[0], ys[1], zs[0], zs[1]]);
    }
    ans
}

fn solve(input_file: &str, base: i64) -> i64 {
    let games = parse_input(input_file);
    let mut ans = 0;
    for [ax, ay, bx, by, tx, ty] in games {
        let (tx, ty) = (tx + base, ty + base);
        let b = (ty * ax - tx * ay) / (ax * by - bx * ay);
        let a = (tx - b * bx) / ax;
        if a * ax + b * bx == tx && a * ay + b * by == ty {
            ans += 3 * a + b;
        }
    }
    ans
}
fn part1(input_file: &str) -> i64 {
    solve(input_file, 0)
}

fn part2(input_file: &str) -> i64 {
    solve(input_file, 10000000000000_i64)
}
