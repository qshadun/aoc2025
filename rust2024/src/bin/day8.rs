use rust2024::read_grid;
use std::collections::{HashMap, HashSet};

fn main() {
    let day = 8;
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

fn collect_freq(grid: &[Vec<char>]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut ans: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '.' {
                ans.entry(grid[i][j]).or_default().push((i, j));
            }
        }
    }
    ans
}

fn count_anti_node(nodes: &[(usize, usize)], m: usize, n: usize) -> HashSet<(usize, usize)> {
    let mut ans = HashSet::new();
    for i in 0..nodes.len() - 1 {
        for j in i + 1..nodes.len() {
            let (x1, y1) = nodes[i];
            let (x1, y1) = (x1 as i32, y1 as i32);
            let (x2, y2) = nodes[j];
            let (x2, y2) = (x2 as i32, y2 as i32);
            let i1 = 2 * x1 - x2;
            let i2 = 2 * x2 - x1;
            let j1 = 2 * y1 - y2;
            let j2 = 2 * y2 - y1;
            if i1 >= 0 && i1 < m as i32 && j1 >= 0 && j1 < n as i32 {
                ans.insert((i1 as usize, j1 as usize));
            }
            if i2 >= 0 && i2 < m as i32 && j2 >= 0 && j2 < n as i32 {
                ans.insert((i2 as usize, j2 as usize));
            }
        }
    }
    ans
}

fn part1(input_file: &str) -> usize {
    let grid = read_grid(input_file);
    let freqs = collect_freq(&grid);
    let mut ans: HashSet<(usize, usize)> = HashSet::new();
    for nodes in freqs.values() {
        ans.extend(count_anti_node(nodes, grid.len(), grid[0].len()));
    }
    ans.len()
}

fn find(x1: i32, y1: i32, x2: i32, y2: i32, m: i32, n: i32) -> HashSet<(i32, i32)> {
    let mut ans = HashSet::new();
    let dx = x2 - x1;
    let dy = y2 - y1;
    let (mut x, mut y) = (x1 + dx, y1 + dy);
    while x >= 0 && x < m && y >= 0 && y < n {
        ans.insert((x, y));
        x += dx;
        y += dy;
    }
    ans
}

fn count_anti_node2(nodes: &[(usize, usize)], m: usize, n: usize) -> HashSet<(i32, i32)> {
    let mut ans = HashSet::new();
    let (m, n) = (m as i32, n as i32);
    for i in 0..nodes.len() - 1 {
        for j in i + 1..nodes.len() {
            let (x1, y1) = nodes[i];
            let (x1, y1) = (x1 as i32, y1 as i32);
            let (x2, y2) = nodes[j];
            let (x2, y2) = (x2 as i32, y2 as i32);
            ans.extend(find(x1, y1, x2, y2, m, n));
            ans.extend(find(x2, y2, x1, y1, m, n));
        }
    }
    ans
}

fn part2(input_file: &str) -> usize {
    let grid = read_grid(input_file);
    let freqs = collect_freq(&grid);
    let mut ans: HashSet<(i32, i32)> = HashSet::new();
    for nodes in freqs.values() {
        ans.extend(count_anti_node2(nodes, grid.len(), grid[0].len()));
    }
    ans.len()
}
