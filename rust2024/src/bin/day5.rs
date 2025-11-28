use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    println!("ans for part1 test: {}", part1("../input/day5_test.txt"));
    println!("ans for part1: {}", part1("../input/day5.txt"));
    println!("ans for part2 test: {}", part2("../input/day5_test.txt"));
    println!("ans for part2: {}", part2("../input/day5.txt"));
}

type AdjList = HashMap<u8, HashSet<u8>>;

fn parse_input(input_file: &str) -> (AdjList, AdjList, Vec<Vec<u8>>) {
    let s = read_to_string(input_file).unwrap();
    let mut before: AdjList = HashMap::new();
    let mut after: AdjList = HashMap::new();
    let mut updates = vec![];
    let mut is_updates = false;
    for line in s.lines() {
        if line.is_empty() {
            is_updates = true;
        } else if !is_updates {
            let nums: Vec<u8> = line.split('|').map(|x| x.parse().unwrap()).collect();
            let [a, b] = &nums[..2] else {
                panic!("slice do not match")
            };
            before.entry(*b).or_default().insert(*a);
            after.entry(*a).or_default().insert(*b);
        } else {
            let nums: Vec<u8> = line.split(',').map(|x| x.parse().unwrap()).collect();
            updates.push(nums);
        }
    }
    (before, after, updates)
}

fn check(update: &[u8], before: &AdjList) -> bool {
    for (i, num) in update[..update.len() - 1].iter().enumerate() {
        match before.get(num) {
            Some(before_nums) => {
                for num in &update[i + 1..] {
                    if before_nums.contains(num) {
                        return false;
                    }
                }
            }
            None => continue,
        }
    }
    true
}

fn part1(input_file: &str) -> u32 {
    let (before, _, updates) = parse_input(input_file);
    let mut ans: u32 = 0;
    for update in updates {
        if check(&update, &before) {
            ans += update[update.len() / 2] as u32;
        }
    }
    ans
}

fn topology_sort(update: &[u8], before1: &AdjList, after1: &AdjList) -> Vec<u8> {
    let mut all_nums: HashSet<u8> = HashSet::new();
    for &num in update {
        all_nums.insert(num);
    }
    let mut before: AdjList = HashMap::new();
    for (a, bs) in before1.iter() {
        if all_nums.contains(a) {
            before.insert(
                *a,
                bs.iter()
                    .filter(|x| all_nums.contains(x))
                    .copied()
                    .collect(),
            );
        }
    }

    let mut after: AdjList = HashMap::new();
    for (a, bs) in after1.iter() {
        if all_nums.contains(a) {
            after.insert(
                *a,
                bs.iter()
                    .filter(|x| all_nums.contains(x))
                    .copied()
                    .collect(),
            );
        }
    }

    let mut indegree: HashMap<u8, usize> = HashMap::new();
    for (a, bs) in before.iter() {
        indegree.insert(*a, bs.len());
    }

    let mut ans = vec![];
    let mut sorted = 0;
    let n = update.len();
    let mut remain: Vec<u8> = update.to_vec();
    while sorted < n {
        let (cur_level, other): (Vec<u8>, Vec<u8>) = remain
            .into_iter()
            .partition(|x| indegree.get(x).unwrap_or(&0) == &0);
        for x in cur_level.iter() {
            match after.get(x) {
                Some(after_nums) => {
                    for num in after_nums {
                        *indegree.get_mut(num).unwrap() -= 1;
                    }
                }
                None => continue,
            }
        }
        sorted += cur_level.len();
        ans.extend(cur_level);
        remain = other;
    }
    ans
}

fn part2(input_file: &str) -> u32 {
    let (before, after, updates) = parse_input(input_file);
    let mut ans: u32 = 0;
    for update in updates {
        if !check(&update, &before) {
            let update = topology_sort(&update, &before, &after);
            ans += update[update.len() / 2] as u32;
        }
    }
    ans
}
