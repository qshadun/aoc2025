#![allow(clippy::collapsible_else_if)]
use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

use rust2024::DELTAS;

fn main() {
    let day = 18;
    println!(
        "ans for part1 test: {}",
        part1(&format!("../input/day{}_test.txt", day), 7, 12)
    );
    println!(
        "ans for part1: {}",
        part1(&format!("../input/day{}.txt", day), 71, 1024)
    );
    println!(
        "ans for part2 test: {}",
        part2(&format!("../input/day{}_test.txt", day), 7, 12)
    );
    println!(
        "ans for part2: {}",
        part2(&format!("../input/day{}.txt", day), 71, 1024)
    );
}

fn parse_input(input_file: &str) -> Vec<(usize, usize)> {
    let mut ans = vec![];
    let s = read_to_string(input_file).unwrap();
    let re = regex::Regex::new(r"\d+").unwrap();
    for line in s.lines() {
        let nums: Vec<usize> = re
            .find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        ans.push((nums[0], nums[1]));
    }
    ans
}

fn part1(input_file: &str, grid_size: usize, bytes: usize) -> usize {
    let blocks = parse_input(input_file);
    let mut grid = vec![vec!['.'; grid_size]; grid_size];
    for &(x, y) in blocks.iter().take(bytes) {
        grid[y][x] = '#';
    }

    let start = (0, 0);
    let mut visisted: HashSet<(i32, i32)> = HashSet::new();
    visisted.insert(start);
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    q.push_back(start);
    let mut steps = 0;
    while !q.is_empty() {
        let cur_len = q.len();
        for _ in 0..cur_len {
            let (x, y) = q.pop_front().unwrap();
            for [dx, dy] in DELTAS {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || ny < 0 || nx >= grid_size as i32 || ny >= grid_size as i32 {
                    continue;
                }
                if nx as usize == grid_size - 1 && ny as usize == grid_size - 1 {
                    return steps + 1;
                }
                if grid[ny as usize][nx as usize] == '.' && !visisted.contains(&(nx, ny)) {
                    visisted.insert((nx, ny));
                    q.push_back((nx, ny));
                }
            }
        }
        steps += 1;
    }
    panic!("no route")
}

fn can_reach(grid: &[Vec<char>]) -> bool {
    let start = (0, 0);
    let grid_size = grid.len();
    let mut visisted: HashSet<(i32, i32)> = HashSet::new();
    visisted.insert(start);
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    q.push_back(start);

    while !q.is_empty() {
        let cur_len = q.len();
        for _ in 0..cur_len {
            let (x, y) = q.pop_front().unwrap();
            for [dx, dy] in DELTAS {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || ny < 0 || nx >= grid_size as i32 || ny >= grid_size as i32 {
                    continue;
                }
                if nx as usize == grid_size - 1 && ny as usize == grid_size - 1 {
                    return true;
                }
                if grid[ny as usize][nx as usize] == '.' && !visisted.contains(&(nx, ny)) {
                    visisted.insert((nx, ny));
                    q.push_back((nx, ny));
                }
            }
        }
    }
    false
}

fn part2(input_file: &str, grid_size: usize, bytes: usize) -> String {
    let blocks = parse_input(input_file);
    let mut grid = vec![vec!['.'; grid_size]; grid_size];
    for &(x, y) in blocks.iter().take(bytes) {
        grid[y][x] = '#';
    }
    for &(x, y) in blocks.iter().skip(bytes) {
        grid[y][x] = '#';
        if !can_reach(&grid) {
            return format!("{},{}", x, y);
        }
    }
    panic!("no solution")
}
