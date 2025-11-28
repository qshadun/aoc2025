use std::fs::read_to_string;

fn main() {
    let day = 9;
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

fn expand(line: &str) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![];
    let mut id = 0;
    for (i, c) in line.chars().enumerate() {
        let cnt: usize = c as usize - '0' as usize;
        if i % 2 == 0 {
            ans.extend(std::iter::repeat(id).take(cnt));
            id += 1;
        } else {
            ans.extend(std::iter::repeat(-1).take(cnt));
        }
    }
    ans
}

fn part1(input_file: &str) -> usize {
    let line = read_to_string(input_file).unwrap();
    let mut disk = expand(&line);
    let mut lo = 0;
    let mut hi = disk.len() - 1;
    let mut ans = 0;
    while lo <= hi {
        if disk[lo] != -1 {
            ans += lo * disk[lo] as usize;
            lo += 1;
        } else {
            while disk[hi] == -1 {
                hi -= 1;
            }
            if hi > lo {
                disk[lo] = disk[hi];
                disk[hi] = -1;
            } else {
                break;
            }
        }
    }
    ans
}

fn expand2(line: &str) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut id = 0;
    for (i, c) in line.chars().enumerate() {
        let cnt = c as i32 - '0' as i32;
        if i % 2 == 0 {
            ans.push(vec![id, cnt]);
            id += 1;
        } else {
            ans.push(vec![-1, cnt]);
        }
    }
    ans
}

fn part2(input_file: &str) -> usize {
    let line = read_to_string(input_file).unwrap();
    let mut disk = expand2(&line);

    let mut hi = disk.len() - 1;

    while hi > 0 {
        if disk[hi][0] == -1 {
            hi -= 1
        }
        let id = disk[hi][0];
        let sz = disk[hi][1];
        let mut moved = false;
        let search_bound = hi;
        for i in 0..search_bound {
            if disk[i][0] == -1 && disk[i][1] >= sz {
                if disk[i][1] > sz {
                    disk[i][1] -= sz;
                    disk.insert(i, vec![id, sz]);
                    disk[hi + 1][0] = -1;
                } else {
                    disk[i][0] = id;
                    disk[hi][0] = -1;
                    hi -= 1;
                }
                moved = true;
                break;
            }
        }
        if !moved {
            hi -= 1;
        }
    }

    let mut ans = 0;
    let mut pos = 0;
    for file in disk {
        let id = file[0];
        let sz = file[1] as usize;
        if id != -1 {
            ans += (pos + pos + sz - 1) * sz / 2 * id as usize;
        }
        pos += sz;
    }
    ans
}
