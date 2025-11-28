#![allow(clippy::collapsible_else_if)]
use std::fs::read_to_string;

use rust2024::Move;

fn main() {
    let day = 15;
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

fn parse_input(input_file: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let s = read_to_string(input_file).unwrap();
    let mut grid: Vec<Vec<char>> = vec![];
    let mut moves: Vec<Move> = vec![];
    let mut after_blank = false;
    for line in s.lines() {
        if line.is_empty() {
            after_blank = true;
        } else {
            if !after_blank {
                let row: Vec<char> = line.chars().collect();
                grid.push(row);
            } else {
                for c in line.chars() {
                    moves.push(Move::from(c).unwrap());
                }
            }
        }
    }
    (grid, moves)
}

fn find_bot(grid: &[Vec<char>]) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                return (i, j);
            }
        }
    }
    panic!("no robot")
}

fn calc_score(r: usize, c: usize) -> usize {
    r * 100 + c
}

fn do_move_usize(mv: Move, r: usize, c: usize) -> (usize, usize) {
    let (r, c) = mv.do_move(r, c);
    (r as usize, c as usize)
}

fn reverse_move_usize(mv: Move, r: usize, c: usize) -> (usize, usize) {
    let (r, c) = mv.reverse_move(r, c);
    (r as usize, c as usize)
}

fn part1(input_file: &str) -> usize {
    let (mut grid, moves) = parse_input(input_file);
    let (mut br, mut bc) = find_bot(&grid);
    for mv in moves {
        let (nr, nc) = do_move_usize(mv, br, bc);

        if grid[nr][nc] == '.' {
            grid[br][bc] = '.';
            grid[nr][nc] = '@';
            br = nr;
            bc = nc;
        } else if grid[nr][nc] == '#' {
            continue;
        } else {
            let (mut er, mut ec) = do_move_usize(mv, nr, nc);
            while grid[er][ec] == 'O' {
                (er, ec) = do_move_usize(mv, er, ec);
            }
            if grid[er][ec] == '#' {
                continue;
            } else {
                while er != nr || ec != nc {
                    grid[er][ec] = 'O';
                    (er, ec) = reverse_move_usize(mv, er, ec);
                }
                grid[br][bc] = '.';
                grid[nr][nc] = '@';
                br = nr;
                bc = nc;
            }
        }
    }
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                ans += calc_score(i, j);
            }
        }
    }
    ans
}

fn transform_grid(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut ans = vec![];
    for row in grid {
        let mut new_row = vec![];
        for &c in row {
            if c == '@' {
                new_row.push('@');
                new_row.push('.');
            } else if c == 'O' {
                new_row.push('[');
                new_row.push(']');
            } else {
                new_row.push(c);
                new_row.push(c);
            }
        }
        ans.push(new_row);
    }
    ans
}

fn part2(input_file: &str) -> usize {
    let (grid, moves) = parse_input(input_file);
    let mut grid = transform_grid(&grid);
    let (mut br, mut bc) = find_bot(&grid);
    for mv in moves {
        let (nr, nc) = do_move_usize(mv, br, bc);

        if grid[nr][nc] == '.' {
            grid[br][bc] = '.';
            grid[nr][nc] = '@';
            br = nr;
            bc = nc;
        } else if grid[nr][nc] == '#' {
            continue;
        } else {
            if matches!(mv, Move::Left | Move::Right) {
                let (mut er, mut ec) = do_move_usize(mv, nr, nc);
                while matches!(grid[er][ec], '[' | ']') {
                    (er, ec) = do_move_usize(mv, er, ec);
                }
                if grid[er][ec] == '#' {
                    continue;
                } else {
                    while er != nr || ec != nc {
                        let (r, c) = reverse_move_usize(mv, er, ec);
                        grid[er][ec] = grid[r][c];
                        (er, ec) = reverse_move_usize(mv, er, ec);
                    }
                    grid[br][bc] = '.';
                    grid[nr][nc] = '@';
                    br = nr;
                    bc = nc;
                }
            } else {
                let mut to_push = vec![];
                if grid[nr][nc] == '[' {
                    to_push.push(vec![(nr, nc), (nr, nc + 1)]);
                } else {
                    to_push.push(vec![(nr, nc - 1), (nr, nc)]);
                }
                let mut blocked = false;
                loop {
                    let mut new_row = vec![];
                    for &(r, c) in to_push.last().unwrap() {
                        let (rr, cc) = do_move_usize(mv, r, c);
                        if !new_row.is_empty() && *new_row.last().unwrap() == (rr, cc) {
                            continue;
                        }
                        if grid[rr][cc] == '#' {
                            blocked = true;
                            break;
                        } else if matches!(grid[rr][cc], '[' | ']') {
                            new_row.push((rr, cc));
                            if grid[rr][cc] == '[' {
                                new_row.push((rr, cc + 1));
                            } else {
                                if new_row.is_empty() || *new_row.last().unwrap() != (rr, cc - 1) {
                                    new_row.pop();
                                    new_row.push((rr, cc - 1));
                                    new_row.push((rr, cc));
                                }
                            }
                        }
                    }
                    if new_row.is_empty() {
                        break;
                    }
                    to_push.push(new_row)
                }
                if blocked {
                    continue;
                }
                to_push.reverse();
                for row in to_push {
                    for (r, c) in row {
                        let (er, ec) = do_move_usize(mv, r, c);
                        grid[er][ec] = grid[r][c];
                        grid[r][c] = '.';
                    }
                }
                grid[br][bc] = '.';
                grid[nr][nc] = '@';
                br = nr;
                bc = nc;
            }
        }
    }
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '[' {
                ans += calc_score(i, j);
            }
        }
    }
    ans
}
