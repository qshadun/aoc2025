#![allow(clippy::collapsible_else_if)]
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use rust2024::{read_grid, Move};

fn main() {
    let day = 16;
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

fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    panic!("no robot")
}

fn do_move_usize(mv: Move, r: usize, c: usize) -> (usize, usize) {
    let (r, c) = mv.do_move(r, c);
    (r as usize, c as usize)
}

fn part1(input_file: &str) -> usize {
    let grid = read_grid(input_file);
    let start = find_start(&grid);
    let mut dist = HashMap::new();
    dist.insert((start, Move::Right), 0);
    let mut hp = BinaryHeap::new();
    hp.push(Reverse((0, start, Move::Right)));
    while let Some(Reverse((cur_dist, (x, y), mv))) = hp.pop() {
        if cur_dist > *dist.get(&((x, y), mv)).unwrap() {
            continue;
        }
        if grid[x][y] == 'E' {
            return cur_dist;
        }
        let (nx, ny) = do_move_usize(mv, x, y);
        if grid[nx][ny] != '#' && *dist.get(&((nx, ny), mv)).unwrap_or(&usize::MAX) > cur_dist + 1 {
            dist.insert(((nx, ny), mv), cur_dist + 1);
            hp.push(Reverse((cur_dist + 1, (nx, ny), mv)));
        }
        let left_turn = mv.reverse_turn();
        if *dist.get(&((x, y), left_turn)).unwrap_or(&usize::MAX) > cur_dist + 1000 {
            dist.insert(((x, y), left_turn), cur_dist + 1000);
            hp.push(Reverse((cur_dist + 1000, (x, y), left_turn)));
        }
        let right_turn = mv.turn();
        if *dist.get(&((x, y), right_turn)).unwrap_or(&usize::MAX) > cur_dist + 1000 {
            dist.insert(((x, y), right_turn), cur_dist + 1000);
            hp.push(Reverse((cur_dist + 1000, (x, y), right_turn)));
        }
    }
    panic!("no route")
}

fn part2(input_file: &str) -> usize {
    let grid = read_grid(input_file);
    let start = find_start(&grid);
    let mut dist = HashMap::new();
    dist.insert((start, Move::Right), 0);
    let mut hp = BinaryHeap::new();
    hp.push(Reverse((0, start, Move::Right, vec![start])));
    let mut best_score = usize::MAX;
    let mut tiles = HashSet::new();
    while let Some(Reverse((cur_dist, (x, y), mv, path))) = hp.pop() {
        if cur_dist > best_score {
            break;
        }
        if cur_dist > *dist.get(&((x, y), mv)).unwrap() {
            continue;
        }
        if grid[x][y] == 'E' {
            best_score = cur_dist;
            for &p in path.iter() {
                tiles.insert(p);
            }
        }
        let (nx, ny) = do_move_usize(mv, x, y);
        if grid[nx][ny] != '#' && *dist.get(&((nx, ny), mv)).unwrap_or(&usize::MAX) > cur_dist {
            dist.insert(((nx, ny), mv), cur_dist + 1);
            let mut new_path = path.clone();
            new_path.push((nx, ny));
            hp.push(Reverse((cur_dist + 1, (nx, ny), mv, new_path)));
        }
        let left_turn = mv.reverse_turn();
        if *dist.get(&((x, y), left_turn)).unwrap_or(&usize::MAX) >= cur_dist + 1000 {
            dist.insert(((x, y), left_turn), cur_dist + 1000);
            hp.push(Reverse((cur_dist + 1000, (x, y), left_turn, path.clone())));
        }
        let right_turn = mv.turn();
        if *dist.get(&((x, y), right_turn)).unwrap_or(&usize::MAX) >= cur_dist + 1000 {
            dist.insert(((x, y), right_turn), cur_dist + 1000);
            hp.push(Reverse((cur_dist + 1000, (x, y), right_turn, path.clone())));
        }
    }
    tiles.len()
}
