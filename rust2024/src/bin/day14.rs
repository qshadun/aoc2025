use std::{fs::read_to_string, sync::LazyLock};

use regex::Regex;
use rust2024::print_grid;

fn main() {
    let day = 14;
    println!(
        "ans for part1 test: {}",
        part1(&format!("../input/day{}_test.txt", day), 11, 7)
    );
    println!(
        "ans for part1: {}",
        part1(&format!("../input/day{}.txt", day), 101, 103)
    );
    // println!(
    //     "ans for part2 test: {}",
    //     part2(&format!("../input/day{}_test.txt", day))
    // );
    println!(
        "ans for part2: {}",
        part2(&format!("../input/day{}.txt", day), 101, 103)
    );
}

fn parse_nums(line: &str) -> Vec<i32> {
    static REGEX: LazyLock<Regex> = std::sync::LazyLock::new(|| Regex::new(r"-?\d+").unwrap());
    REGEX
        .find_iter(line)
        .map(|x| x.as_str().parse().unwrap())
        .collect()
}

fn parse_input(input_file: &str) -> Vec<[i32; 4]> {
    let s = read_to_string(input_file).unwrap();
    let mut ans = vec![];

    for line in s.lines() {
        let xs: Vec<i32> = parse_nums(line);
        ans.push([xs[0], xs[1], xs[2], xs[3]]);
    }
    ans
}

fn move_one_axis(p: i32, v: i32, b: i32) -> i32 {
    let p = p + v;
    if p < 0 {
        p + b
    } else if p >= b {
        p - b
    } else {
        p
    }
}

fn move_robot(robot: &mut [i32; 4], bx: i32, by: i32) {
    robot[0] = move_one_axis(robot[0], robot[2], bx);
    robot[1] = move_one_axis(robot[1], robot[3], by);
}

fn safty_score(robots: &[[i32; 4]], bx: i32, by: i32) -> i64 {
    let mut s1 = 0;
    let mut s2 = 0;
    let mut s3 = 0;
    let mut s4 = 0;
    let mx = bx / 2;
    let my = by / 2;
    for [px, py, _, _] in robots {
        let (px, py) = (*px, *py);
        if px == mx || py == my {
            continue;
        }
        if px >= 0 && px < mx && py >= 0 && py < my {
            s1 += 1;
        } else if px >= 0 && px < mx && py > my {
            s2 += 1;
        } else if py < my {
            s3 += 1;
        } else {
            s4 += 1;
        }
    }
    s1 * s2 * s3 * s4
}

fn part1(input_file: &str, bx: i32, by: i32) -> i64 {
    let mut robots = parse_input(input_file);
    for robot in robots.iter_mut() {
        for _ in 0..100 {
            move_robot(robot, bx, by);
        }
    }
    safty_score(&robots, bx, by)
}

fn to_grid(robots: &[[i32; 4]], bx: i32, by: i32) -> Vec<Vec<char>> {
    let mut ans = vec![vec!['.'; bx as usize]; by as usize];
    for [px, py, _, _] in robots {
        ans[*py as usize][*px as usize] = '+';
    }
    ans
}

fn check_tree(grid: &[Vec<char>]) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    let mut straigh_line_each_row = vec![0; m];
    for (i, row) in grid.iter().enumerate() {
        let mut left = 0;
        let mut right = 0;
        while left < n && right < n {
            while left < n && row[left] != '+' {
                left += 1;
            }
            right = left + 1;
            while right < n && row[right] == '+' {
                right += 1;
            }
            straigh_line_each_row[i] = straigh_line_each_row[i].max(right - left);
            left = right;
        }
    }
    *straigh_line_each_row.iter().max().unwrap() > 10
}

fn part2(input_file: &str, bx: i32, by: i32) -> i64 {
    let mut robots = parse_input(input_file);
    let mut ans = 0;
    loop {
        ans += 1;
        for robot in robots.iter_mut() {
            move_robot(robot, bx, by);
        }
        let grid = to_grid(&robots, bx, by);
        if check_tree(&grid) {
            print_grid(&grid);
            return ans;
        }
    }
}
