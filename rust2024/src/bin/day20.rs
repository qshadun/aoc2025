#![allow(clippy::collapsible_else_if)]
use std::collections::{HashMap, VecDeque};

use rust2024::{read_grid, DELTAS};

fn main() {
    let day = 20;
    println!(
        "ans for part1 test: {}",
        part1(&format!("../input/day{}_test.txt", day), 64)
    );
    println!(
        "ans for part1: {}",
        part1(&format!("../input/day{}.txt", day), 100)
    );
    println!(
        "ans for part2 test: {}",
        part2(&format!("../input/day{}_test.txt", day), 50)
    );
    println!(
        "ans for part2: {}",
        part2(&format!("../input/day{}.txt", day), 100)
    );
}

fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    panic!("no start")
}

fn find_end(grid: &[Vec<char>]) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'E' {
                return (i, j);
            }
        }
    }
    panic!("no end")
}

type PreMap = HashMap<(usize, usize), Option<(usize, usize)>>;
fn find_shortest_path(grid: &[Vec<char>]) -> (usize, PreMap) {
    let mut steps = 0;
    let start = find_start(grid);
    let mut pre = HashMap::new();
    pre.insert(start, None);
    let mut q = VecDeque::new();
    q.push_back(start);
    loop {
        let cur_len = q.len();
        for _ in 0..cur_len {
            let (x, y) = q.pop_front().unwrap();
            let (xx, yy) = (x as i32, y as i32);
            for [dx, dy] in DELTAS {
                let (nx, ny) = (xx + dx, yy + dy);
                if nx >= 0 && ny >= 0 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if grid[nx][ny] != '#' && !pre.contains_key(&(nx, ny)) {
                        q.push_back((nx, ny));
                        pre.insert((nx, ny), Some((x, y)));
                        if grid[nx][ny] == 'E' {
                            return (steps + 1, pre);
                        }
                    }
                }
            }
        }
        steps += 1;
    }
}

fn to_dist(
    pre: HashMap<(usize, usize), Option<(usize, usize)>>,
    end: (usize, usize),
) -> HashMap<(usize, usize), usize> {
    let mut dist = HashMap::new();
    let mut cur = Some(end);
    let mut cur_dist = 0;
    while let Some(cur_node) = cur {
        dist.insert(cur_node, cur_dist);
        cur_dist += 1;
        cur = *pre.get(&cur_node).unwrap();
    }
    dist
}

fn count_cheats(grid: &[Vec<char>], cheat_len: i32, cut: usize) -> usize {
    let (_, pre) = find_shortest_path(grid);
    let end = find_end(grid);
    let dist = to_dist(pre, end);
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let mut ans = 0;
    for (&(x, y), &d) in dist.iter() {
        if d > cut {
            for dx in -cheat_len..cheat_len + 1 {
                let nx = x as i32 + dx;
                if nx >= 1 && nx < m - 1 {
                    let y_bound = cheat_len - dx.abs();
                    for dy in -y_bound..y_bound + 1 {
                        let ny = y as i32 + dy;
                        if ny >= 1 && ny < n - 1 {
                            let dd = dx.abs() + dy.abs();
                            if dd < 2 {
                                continue;
                            }
                            let (nx, ny) = (nx as usize, ny as usize);
                            if let Some(&d1) = dist.get(&(nx, ny)) {
                                if d1 + cut + dd as usize <= d {
                                    ans += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ans
}
fn part1(input_file: &str, cut_steps: usize) -> usize {
    let grid = read_grid(input_file);
    count_cheats(&grid, 2, cut_steps)
}

fn part2(input_file: &str, cut_steps: usize) -> usize {
    let grid = read_grid(input_file);
    count_cheats(&grid, 20, cut_steps)
}
