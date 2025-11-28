use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let day = 11;
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

fn parse_input(input_file: &str) -> Vec<u128> {
    let s = read_to_string(input_file).unwrap();
    let mut ans = vec![];
    for p in s.split(' ') {
        ans.push(p.parse().unwrap());
    }
    ans
}

fn blink(nums: &[u128]) -> Vec<u128> {
    let mut ans = vec![];
    for num in nums {
        if num == &0 {
            ans.push(1);
        } else {
            let s = format!("{}", num);
            if s.len() % 2 == 0 {
                let s1 = &s[..s.len() / 2];
                let s2 = &s[s.len() / 2..];
                ans.push(s1.parse().unwrap());
                ans.push(s2.parse().unwrap());
            } else {
                ans.push(num * 2024);
            }
        }
    }
    ans
}

fn dfs(num: u128, times: u8, memo: &mut HashMap<(u128, u8), u128>) -> u128 {
    if times == 0 {
        return 1;
    }
    if let Some(ans) = memo.get(&(num, times)) {
        return *ans;
    }
    let mut ans = 0;
    for x in blink(&[num]) {
        ans += dfs(x, times - 1, memo);
    }
    memo.insert((num, times), ans);
    ans
}

fn part1(input_file: &str) -> usize {
    let mut nums = parse_input(input_file);
    for _ in 0..25 {
        nums = blink(&nums);
    }
    nums.len()
}

fn part2(input_file: &str) -> u128 {
    let nums = parse_input(input_file);
    let mut ans = 0;
    let mut memo = HashMap::new();
    for num in nums {
        ans += dfs(num, 75, &mut memo);
    }
    ans
}
