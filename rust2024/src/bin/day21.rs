#![allow(clippy::collapsible_else_if)]
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

use rust2024::Move;

fn main() {
    let day = 21;
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

const NUM_PAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];
const DIR_PAD: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];

type Point = (usize, usize);

fn map_key_to_positon(board: &[[char; 3]]) -> HashMap<char, Point> {
    let mut ans = HashMap::new();
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            ans.insert(board[i][j], (i, j));
        }
    }
    ans
}

fn parse_input(input_file: &str) -> Vec<String> {
    let s = read_to_string(input_file).unwrap();
    s.lines().map(|x| x.to_string()).collect()
}

fn shortest_path(board: &[[char; 3]], start: Point, end: Point) -> Vec<String> {
    if start == end {
        return vec![];
    }
    let (m, n) = (board.len(), board[0].len());

    let mut ans = vec![];
    let mut shortest = 100;
    let mut steps = 0;
    let mut q = VecDeque::<(Point, String)>::new();
    let mut visited = HashSet::<(Point, String)>::new();
    q.push_back((start, String::new()));
    visited.insert((start, String::new()));
    while !q.is_empty() && steps < shortest {
        let cur_len = q.len();
        for _ in 0..cur_len {
            let ((x, y), path) = q.pop_front().unwrap();
            for mv in Move::moves() {
                let (nx, ny) = mv.do_move(x, y);
                if nx < 0 || ny < 0 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if nx < m && ny < n && board[nx][ny] != ' ' {
                    let new_path = format!("{path}{}", mv as u8 as char);
                    if (nx, ny) == end {
                        shortest = steps;
                        ans.push(new_path);
                    } else if !visited.contains(&((nx, ny), new_path.clone())) {
                        visited.insert(((nx, ny), new_path.clone()));
                        q.push_back(((nx, ny), new_path));
                    }
                }
            }
        }
        steps += 1;
    }
    ans
}

type CacheKey = (usize, char, char);
#[derive(Debug, Default)]
struct Game {
    bot_cnt: usize,
    num_pad_key_to_position: HashMap<char, Point>,
    dir_pad_key_to_position: HashMap<char, Point>,
    cache: HashMap<CacheKey, usize>,
    sp_cache_num: HashMap<(Point, Point), Vec<String>>,
    sp_cache_dir: HashMap<(Point, Point), Vec<String>>,
}

impl Game {
    fn new(bot_cnt: usize) -> Self {
        let num_pad_key_to_position = map_key_to_positon(&NUM_PAD);
        let dir_pad_key_to_position = map_key_to_positon(&DIR_PAD);
        Self {
            bot_cnt,
            num_pad_key_to_position,
            dir_pad_key_to_position,
            ..Default::default()
        }
    }

    fn get_shortest_path_num_pad(&mut self, start: Point, end: Point) -> &Vec<String> {
        self.sp_cache_num
            .entry((start, end))
            .or_insert_with(|| shortest_path(&NUM_PAD, start, end))
    }

    fn get_shortest_path_dir_pad(&mut self, start: Point, end: Point) -> &Vec<String> {
        self.sp_cache_dir
            .entry((start, end))
            .or_insert_with(|| shortest_path(&DIR_PAD, start, end))
    }

    fn dp(&mut self, cur_bot: usize, start_key: char, end_key: char) -> usize {
        if start_key == end_key {
            return 1;
        }
        let key = (cur_bot, start_key, end_key);
        if self.cache.contains_key(&key) {
            return *self.cache.get(&key).unwrap();
        }
        let path: &Vec<String> = if cur_bot == 0 {
            let start = *self.num_pad_key_to_position.get(&start_key).unwrap();
            let end = *self.num_pad_key_to_position.get(&end_key).unwrap();
            self.get_shortest_path_num_pad(start, end)
        } else {
            let start = *self.dir_pad_key_to_position.get(&start_key).unwrap();
            let end = *self.dir_pad_key_to_position.get(&end_key).unwrap();
            self.get_shortest_path_dir_pad(start, end)
        };
        let path = path.clone();
        if cur_bot == self.bot_cnt {
            self.cache.insert(key, path[0].len() + 1);
            return path[0].len() + 1;
        }
        let mut ans = usize::MAX;
        for p in path {
            let p: Vec<char> = p.chars().collect();
            let mut steps = self.dp(cur_bot + 1, 'A', p[0]);
            for i in 1..p.len() {
                steps += self.dp(cur_bot + 1, p[i - 1], p[i]);
            }
            steps += self.dp(cur_bot + 1, *p.last().unwrap(), 'A');
            ans = ans.min(steps);
        }
        self.cache.insert(key, ans);
        ans
    }
}

fn solve(input_file: &str, bot_cnt: usize) -> usize {
    let seqs = parse_input(input_file);
    let mut ans = 0;
    let mut game = Game::new(bot_cnt);

    for seq in seqs {
        let seq_char: Vec<char> = seq.chars().collect();
        let mut complexity = game.dp(0, 'A', seq_char[0]);
        for i in 1..seq.len() {
            complexity += game.dp(0, seq_char[i - 1], seq_char[i]);
        }
        let num: usize = seq[0..seq.len() - 1].parse().unwrap();
        ans += complexity * num;
    }
    ans
}

fn part1(input_file: &str) -> usize {
    solve(input_file, 2)
}

fn part2(input_file: &str) -> usize {
    solve(input_file, 25)
}
