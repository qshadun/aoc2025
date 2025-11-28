use rust2024::{read_grid, Move};

fn main() {
    println!("ans for part1 test: {}", part1("../input/day6_test.txt"));
    println!("ans for part1: {}", part1("../input/day6.txt"));
    println!("ans for part2 test: {}", part2("../input/day6_test.txt"));
    println!("ans for part2: {}", part2("../input/day6.txt"));
}

fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                return (i, j);
            }
        }
    }
    panic!("no start")
}

fn part1(input_file: &str) -> u32 {
    let mut grid = read_grid(input_file);
    let start = find_start(&grid);
    mark_grid(&mut grid, start);
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if Move::from(grid[i][j]).is_ok() {
                ans += 1;
            }
        }
    }
    ans
}

fn mark_grid(grid: &mut [Vec<char>], start: (usize, usize)) {
    let mut cur_pos = start;
    let (x, y) = cur_pos;
    let mut cur_move = Move::from(grid[x][y]).unwrap();
    loop {
        let (x, y) = cur_pos;
        let (nx, ny) = cur_move.do_move(x, y);
        if nx < 0 || ny < 0 || nx as usize >= grid.len() || ny as usize >= grid[0].len() {
            break;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        if grid[nx][ny] == '#' {
            cur_move = cur_move.turn();
            grid[x][y] = cur_move.into();
        } else {
            grid[nx][ny] = cur_move.into();
            cur_pos = (nx, ny);
        }
    }
}

fn explore(grid: &mut [Vec<char>], start: (usize, usize)) -> bool {
    let mut cur_pos = start;
    let (x, y) = cur_pos;
    let mut cur_move = Move::from(grid[x][y]).unwrap();
    let mut steps = 0;
    let max_steps = grid.len() * grid[0].len();
    loop {
        let (x, y) = cur_pos;
        let (nx, ny) = cur_move.do_move(x, y);
        if nx < 0 || ny < 0 || nx as usize >= grid.len() || ny as usize >= grid[0].len() {
            return false;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        if grid[nx][ny] == '#' {
            cur_move = cur_move.turn();
            let (nx, ny) = cur_move.do_move(x, y);
            if nx >= 0
                && ny >= 0
                && (nx as usize) < grid.len()
                && (ny as usize) < grid[0].len()
                && grid[nx as usize][ny as usize] == cur_move.into()
            {
                return true;
            }

            grid[x][y] = cur_move.into();
        } else {
            steps += 1;
            if steps > max_steps {
                return true;
            }
            grid[nx][ny] = cur_move.into();
            cur_pos = (nx, ny);
        }
    }
}

fn part2(input_file: &str) -> u32 {
    let grid = read_grid(input_file);
    let start = find_start(&grid);
    let mut marked_grid = grid.clone();
    mark_grid(&mut marked_grid, start);
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if (i, j) == start {
                continue;
            }
            if Move::from(marked_grid[i][j]).is_ok() {
                let mut new_grid = grid.clone();
                new_grid[i][j] = '#';
                if explore(&mut new_grid, start) {
                    ans += 1;
                }
            }
        }
    }
    ans
}
