use std::fs::read_to_string;

const DELTAS: [[i32; 2]; 8] = [
    [-1, 0],
    [1, 0],
    [0, -1],
    [0, 1],
    [-1, -1],
    [-1, 1],
    [1, 1],
    [1, -1],
];
const TARGET: &str = "XMAS";

fn main() {
    println!("ans for part1 test: {}", part1("../input/day4_test.txt"));
    println!("ans for part1: {}", part1("../input/day4.txt"));
    println!("ans for part2 test: {}", part2("../input/day4_test.txt"));
    println!("ans for part2: {}", part2("../input/day4.txt"));
}

fn parse_input(input_file: &str) -> Vec<Vec<char>> {
    let s = read_to_string(input_file).unwrap();
    let mut ans = vec![];
    for line in s.lines() {
        ans.push(line.chars().collect());
    }
    ans
}

fn check_one_direction(grid: &[Vec<char>], i: usize, j: usize, delta: &[i32; 2]) -> bool {
    if grid[i][j] != TARGET.chars().next().unwrap() {
        return false;
    }
    let m = grid.len();
    let n = grid[0].len();

    let [di, dj] = delta;
    for x in 1..4 {
        let nx = di * x + i as i32;
        let ny = dj * x + j as i32;
        if nx < 0 || ny < 0 {
            return false;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        if nx >= m || ny >= n {
            return false;
        }
        if grid[nx][ny] != TARGET.chars().nth(x as usize).unwrap() {
            return false;
        }
    }
    true
}

fn check(grid: &[Vec<char>], i: usize, j: usize) -> u32 {
    let mut ans = 0;
    for delta in DELTAS.iter() {
        if check_one_direction(grid, i, j, delta) {
            ans += 1;
        }
    }
    ans
}

fn part1(input_file: &str) -> u32 {
    let grid = parse_input(input_file);
    let mut ans = 0;
    let m = grid.len();
    let n = grid[0].len();
    for i in 0..m {
        for j in 0..n {
            ans += check(&grid, i, j);
        }
    }
    ans
}

fn part2(input_file: &str) -> u32 {
    let grid = parse_input(input_file);
    let mut ans = 0;
    let m = grid.len();
    let n = grid[0].len();
    for i in 1..m - 1 {
        for j in 1..n - 1 {
            if grid[i][j] == 'A' {
                let s1 = format!("{}{}", grid[i - 1][j - 1], grid[i + 1][j + 1]);
                let s2 = format!("{}{}", grid[i - 1][j + 1], grid[i + 1][j - 1]);
                if (s1 == "MS" || s1 == "SM") && (s2 == "MS" || s2 == "SM") {
                    ans += 1;
                }
            }
        }
    }
    ans
}
