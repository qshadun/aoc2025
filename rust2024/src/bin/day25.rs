#![allow(clippy::collapsible_else_if)]
use std::fs::read_to_string;

fn main() {
    let day = 25;
    println!(
        "ans for part1 test: {}",
        part1(&format!("../input/day{}_test.txt", day))
    );
    println!(
        "ans for part1: {}",
        part1(&format!("../input/day{}.txt", day))
    );
}

#[allow(clippy::needless_range_loop)]
fn parse_input(input_file: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut locks = vec![];
    let mut keys = vec![];

    let s = read_to_string(input_file).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    for i in (0..lines.len()).step_by(8) {
        let grid: Vec<Vec<char>> = lines[i..i + 7]
            .iter()
            .map(|line| line.chars().collect())
            .collect();
        if grid[0][0] == '#' {
            let mut heights = vec![0; 5];
            for j in 0..5 {
                for k in 1..7 {
                    if grid[k][j] == '.' {
                        heights[j] = (k - 1) as u8;
                        break;
                    }
                }
            }
            locks.push(heights)
        } else {
            let mut heights = vec![0; 5];
            for j in 0..5 {
                for k in 1..7 {
                    if grid[k][j] == '#' {
                        heights[j] = (6 - k) as u8;
                        break;
                    }
                }
            }
            keys.push(heights)
        }
    }
    (locks, keys)
}

fn part1(input_file: &str) -> i64 {
    let (locks, keys) = parse_input(input_file);
    let mut ans = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            let mut fit = true;
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    fit = false;
                    break;
                }
            }
            if fit {
                ans += 1;
            }
        }
    }
    ans
}
