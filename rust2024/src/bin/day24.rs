#![allow(clippy::collapsible_else_if)]
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    time::Instant,
};

fn main() {
    let day = 24;
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
    let now = Instant::now();
    println!(
        "ans for part2: {}, time used: {} ms",
        part2(&format!("../input/day{}.txt", day)),
        now.elapsed().as_millis()
    );
}
type Gate<'a> = [&'a str; 4];

fn parse_input(input_file: &str) -> (HashMap<&str, u8>, Vec<Gate>) {
    let mut values: HashMap<&str, u8> = HashMap::new();
    let mut gates: Vec<[&str; 4]> = vec![];
    let mut is_gates = false;
    let s = read_to_string(input_file).unwrap();
    let s = Box::leak(s.into_boxed_str());
    for line in s.lines() {
        if line.is_empty() {
            is_gates = true;
        } else if !is_gates {
            let parts: Vec<_> = line.split(": ").collect();
            values.insert(parts[0], parts[1].parse().unwrap());
        } else {
            let parts: Vec<_> = line.split(' ').collect();
            gates.push([parts[0], parts[1], parts[2], parts[4]]);
        }
    }
    (values, gates)
}

fn calc(a: u8, op: &str, b: u8) -> u8 {
    match op {
        "XOR" => a ^ b,
        "AND" => a & b,
        "OR" => a | b,
        _ => panic!("unknown op {}", op),
    }
}
fn part1(input_file: &str) -> i64 {
    let (mut values, gates) = parse_input(input_file);
    let mut ans = 0;
    let mut unsolved = gates;
    while !unsolved.is_empty() {
        let mut remain = vec![];
        for [a, op, b, c] in unsolved {
            if values.contains_key(a) && values.contains_key(b) {
                values.insert(
                    c,
                    calc(*values.get(a).unwrap(), op, *values.get(b).unwrap()),
                );
            } else {
                remain.push([a, op, b, c]);
            }
        }
        unsolved = remain;
    }
    for (k, v) in values {
        if k.starts_with('z') && v == 1 {
            let idx: u8 = k[1..].parse().unwrap();
            ans |= 1 << idx;
        }
    }
    ans
}

fn to_node(prefix: char, i: usize, node_names: &HashMap<(char, usize), String>) -> &str {
    node_names.get(&(prefix, i)).unwrap()
}

fn get_res<'a>(
    a: &str,
    b: &str,
    op: &str,
    ops: &'a HashMap<(&str, &str, &str), &str>,
) -> Option<&'a str> {
    match ops.get(&(a, b, op)) {
        Some(c) => Some(*c),
        None => ops.get(&(b, a, op)).cloned(),
    }
}

fn furthest_made(
    gates: &[Gate],
    node_names: &HashMap<(char, usize), String>,
) -> (usize, HashSet<String>) {
    let mut ops: HashMap<(&str, &str, &str), &str> = HashMap::new();
    for &[a, op, b, c] in gates {
        ops.insert((a, b, op), c);
    }
    let mut carries = [""; 45];
    let mut correct: HashSet<String> = HashSet::new();
    let mut prev_intermediates: Vec<&str> = vec![];

    for i in 0..45usize {
        let pre_digit = get_res(
            to_node('x', i, node_names),
            to_node('y', i, node_names),
            "XOR",
            &ops,
        )
        .unwrap_or_default();
        let pre_carry1 = get_res(
            to_node('x', i, node_names),
            to_node('y', i, node_names),
            "AND",
            &ops,
        )
        .unwrap_or_default();
        if i == 0 {
            carries[i] = pre_carry1;
            continue;
        }
        let digit = get_res(carries[i - 1], pre_digit, "XOR", &ops);
        if digit.is_none() || digit.unwrap() != to_node('z', i, node_names) {
            return (i - 1, correct);
        }
        correct.insert(carries[i - 1].to_string());
        correct.insert(pre_digit.to_string());
        for wire in prev_intermediates {
            correct.insert(wire.to_string());
        }
        let pre_carry2 = get_res(carries[i - 1], pre_digit, "AND", &ops).unwrap_or_default();
        let carry_out = get_res(pre_carry2, pre_carry1, "OR", &ops).unwrap_or_default();
        carries[i] = carry_out;
        prev_intermediates = vec![pre_carry1, pre_carry2];
    }
    (45, correct)
}

use itertools::Itertools;

fn part2(input_file: &str) -> String {
    let (_, mut gates) = parse_input(input_file);
    let mut swaps: Vec<&str> = vec![];
    let mut node_names: HashMap<(char, usize), String> = HashMap::new();
    for i in 0..45usize {
        for prefix in ['x', 'y', 'z'] {
            let name = if i < 10 {
                format!("{prefix}0{i}")
            } else {
                format!("{prefix}{i}")
            };
            node_names.insert((prefix, i), name);
        }
    }
    let (mut base, mut base_used) = furthest_made(&gates, &node_names);
    for _ in 0..4 {
        for combo in (0..gates.len()).combinations(2) {
            let i = combo[0];
            let j = combo[1];
            let [a_i, op_i, b_i, c_i] = gates[i];
            let [a_j, op_j, b_j, c_j] = gates[j];
            if c_i == "z00" || c_j == "z00" {
                continue;
            }
            if base_used.contains(c_i) || base_used.contains(c_j) {
                continue;
            }
            gates[i] = [a_i, op_i, b_i, c_j];
            gates[j] = [a_j, op_j, b_j, c_i];
            let (attempt, attemp_used) = furthest_made(&gates, &node_names);
            if attempt > base {
                println!(
                    "Found a good swap ({},{}). Got to a higher iteration number: {}",
                    c_i, c_j, attempt
                );
                swaps.push(c_i);
                swaps.push(c_j);
                base = attempt;
                base_used = attemp_used;
                break;
            }
            gates[i] = [a_i, op_i, b_i, c_i];
            gates[j] = [a_j, op_j, b_j, c_j];
        }
    }
    swaps.sort();
    swaps.join(",")
}
