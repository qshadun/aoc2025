use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    let day = 10;
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

fn parse_input(input_file: &str) -> Vec<Vec<u8>> {
    let s = read_to_string(input_file).unwrap();
    let mut ans = vec![];
    for line in s.lines() {
        let row: Vec<u8> = line.chars().map(|c| c as u8 - b'0').collect();
        ans.push(row);
    }
    ans
}

const DELTAS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

fn calc_score(start_x: usize, start_y: usize, grid: &[Vec<u8>]) -> i32 {
    let mut ans = 0;
    let m = grid.len();
    let n = grid[0].len();
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((start_x, start_y));
    let mut visited = vec![vec![0_u8; n]; m];
    visited[start_x][start_y] = 1;
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        if grid[x][y] == 9 {
            ans += 1;
            continue;
        }
        for [dx, dy] in DELTAS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && (nx as usize) < m && ny >= 0 && (ny as usize) < n {
                let nx = nx as usize;
                let ny = ny as usize;
                if grid[nx][ny] == grid[x][y] + 1 && visited[nx][ny] != 1 {
                    visited[nx][ny] = 1;
                    q.push_back((nx, ny));
                }
            }
        }
    }
    ans
}

fn part1(input_file: &str) -> i32 {
    let grid = parse_input(input_file);
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                ans += calc_score(i, j, &grid);
            }
        }
    }
    ans
}

fn dfs(x: usize, y: usize, grid: &Vec<Vec<u8>>, memo: &mut Vec<Vec<i32>>) -> i32 {
    if grid[x][y] == 9 {
        return 1;
    }
    if memo[x][y] != -1 {
        return memo[x][y];
    }
    let mut ans = 0;
    for [dx, dy] in DELTAS {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && (nx as usize) < grid.len() && ny >= 0 && (ny as usize) < grid[0].len() {
            let nx = nx as usize;
            let ny = ny as usize;
            if grid[nx][ny] == grid[x][y] + 1 {
                ans += dfs(nx, ny, grid, memo);
            }
        }
    }
    memo[x][y] = ans;
    ans
}

fn part2(input_file: &str) -> i32 {
    let grid = parse_input(input_file);
    let mut ans = 0;
    let mut memo = vec![vec![-1; grid[0].len()]; grid.len()];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                ans += dfs(i, j, &grid, &mut memo);
            }
        }
    }
    ans
}
