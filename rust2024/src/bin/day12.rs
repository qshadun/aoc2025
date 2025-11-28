use std::collections::{HashSet, VecDeque};

use rust2024::{read_grid, DELTAS};

fn main() {
    let day = 12;
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

fn part1(input_file: &str) -> usize {
    let grid = read_grid(input_file);
    let mut ans = 0;
    let m = grid.len();
    let n = grid[0].len();
    let mut visited = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            if visited[i][j] == 0 {
                let mut q: VecDeque<(usize, usize)> = VecDeque::new();
                q.push_back((i, j));
                visited[i][j] = 1;
                let mut area = 0;
                let mut perimeter = 0;
                let cc = grid[i][j];
                while !q.is_empty() {
                    let (x, y) = q.pop_front().unwrap();
                    let (x, y) = (x as i32, y as i32);
                    area += 1;
                    for [dx, dy] in DELTAS {
                        let (nx, ny) = (x + dx, y + dy);
                        let mut same = false;
                        if nx >= 0 && ny >= 0 {
                            let (nx, ny) = (nx as usize, ny as usize);
                            if nx < m && ny < n && grid[nx][ny] == cc {
                                same = true;
                                if visited[nx][ny] == 0 {
                                    visited[nx][ny] = 1;
                                    q.push_back((nx, ny));
                                }
                            }
                        }
                        if !same {
                            perimeter += 1;
                        }
                    }
                }
                ans += perimeter * area;
            }
        }
    }
    ans
}

fn part2(input_file: &str) -> usize {
    let grid = read_grid(input_file);
    let mut ans = 0;
    let m = grid.len();
    let n = grid[0].len();
    let mut visited = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            if visited[i][j] == 0 {
                let mut q: VecDeque<(usize, usize)> = VecDeque::new();
                q.push_back((i, j));
                visited[i][j] = 1;
                let mut area = 0;
                let mut sides = 0;
                let mut perimeter: HashSet<(i32, i32, i32, i32)> = HashSet::new();
                let cc = grid[i][j];
                while !q.is_empty() {
                    let (x, y) = q.pop_front().unwrap();
                    let (x, y) = (x as i32, y as i32);
                    area += 1;
                    for [dx, dy] in DELTAS {
                        let (nx, ny) = (x + dx, y + dy);
                        let mut same = false;
                        if nx >= 0 && ny >= 0 {
                            let (nx, ny) = (nx as usize, ny as usize);
                            if nx < m && ny < n && grid[nx][ny] == cc {
                                same = true;
                                if visited[nx][ny] == 0 {
                                    visited[nx][ny] = 1;
                                    q.push_back((nx, ny));
                                }
                            }
                        }
                        if !same {
                            perimeter.insert((dx, dy, nx, ny));
                            if dx != 0 {
                                if !(perimeter.contains(&(dx, dy, nx, ny - 1))
                                    || perimeter.contains(&(dx, dy, nx, ny + 1)))
                                {
                                    sides += 1;
                                }
                            } else if !(perimeter.contains(&(dx, dy, nx - 1, ny))
                                || perimeter.contains(&(dx, dy, nx + 1, ny)))
                            {
                                sides += 1;
                            }
                        }
                    }
                }
                ans += sides * area;
            }
        }
    }
    ans
}
