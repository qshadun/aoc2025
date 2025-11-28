#![allow(clippy::collapsible_else_if)]
use std::fs::read_to_string;

fn main() {
    let day = 17;
    println!(
        "ans for part1 test: {}",
        part1(&format!("../input/day{}_test.txt", day))
    );
    println!(
        "ans for part1: {}",
        part1(&format!("../input/day{}.txt", day))
    );
    // println!(
    //     "ans for part2 test: {}",
    //     part2(&format!("../input/day{}_test.txt", day))
    // );
    println!(
        "ans for part2: {}",
        part2(&format!("../input/day{}.txt", day))
    );
}

#[derive(Debug, Default)]
struct Computer {
    program: Vec<usize>,
    reg: [usize; 3],
    pos: usize,
    outputs: Vec<usize>,
}

impl Computer {
    fn new(program: Vec<usize>, reg: [usize; 3]) -> Self {
        Self {
            program,
            reg,
            ..Default::default()
        }
    }

    fn get_combo(&self, oprand: usize) -> usize {
        if oprand <= 3 {
            oprand
        } else {
            self.reg[oprand - 4]
        }
    }

    fn run_one_line(&mut self) {
        let op = self.program[self.pos];
        let oprand = self.program[self.pos + 1];
        match op {
            0 => {
                let norm = self.reg[0];
                let denorm = 2_usize.pow(self.get_combo(oprand) as u32);
                self.reg[0] = norm / denorm;
            }
            1 => self.reg[1] ^= oprand,
            2 => self.reg[1] = self.get_combo(oprand) % 8,
            3 => {
                if self.reg[0] != 0 {
                    self.pos = oprand;
                    return;
                }
            }
            4 => self.reg[1] ^= self.reg[2],
            5 => self.outputs.push(self.get_combo(oprand) % 8),
            6 => {
                let norm = self.reg[0];
                let denorm = 2_usize.pow(self.get_combo(oprand) as u32);
                self.reg[1] = norm / denorm;
            }
            7 => {
                let norm = self.reg[0];
                let denorm = 2_usize.pow(self.get_combo(oprand) as u32);
                self.reg[2] = norm / denorm;
            }
            _ => panic!("wrong op"),
        }
        self.pos += 2;
    }

    fn execute(&mut self) -> &Vec<usize> {
        while self.pos < self.program.len() {
            self.run_one_line();
        }
        &self.outputs
    }

    // fn check(&mut self) -> bool {
    //     while self.pos < self.program.len() {
    //         self.run_one_line();
    //         if !self.outputs.is_empty() && *self.outputs.last().unwrap() != self.program[self.outputs.len() - 1] {
    //             return false;
    //         }
    //     }
    //     self.outputs == self.program
    // }
}

fn parse_input(input_file: &str) -> Computer {
    let s = read_to_string(input_file).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    let re = regex::Regex::new(r"\d+").unwrap();
    let mut reg = [0_usize; 3];
    for i in 0..3 {
        let num = re.find_iter(lines[i]).next().unwrap();
        reg[i] = num.as_str().parse().unwrap();
    }
    let program: Vec<usize> = re
        .find_iter(lines[4])
        .map(|m| m.as_str().parse().unwrap())
        .collect();
    Computer::new(program, reg)
}

fn part1(input_file: &str) -> String {
    let mut computer = parse_input(input_file);
    let res = computer.execute();
    res.iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn calc(a: usize) -> (usize, usize) {
    let b = (a % 8) ^ 3;
    let c = a / (2_usize.pow(b as u32));
    let a = a / 8;
    let b = b ^ 5;
    let b = b ^ c;
    (a, b % 8)
}

fn recur(cur: usize, pos: usize, seq: &[usize]) -> usize {
    let n = seq.len();
    if pos == n {
        return cur;
    }
    let cur = cur * 8;
    for i in 0..8 {
        if calc(cur + i).1 == seq[pos] {
            let x = recur(cur + i, pos + 1, seq);
            if x != 0 {
                return x;
            }
        }
    }
    0
}
fn part2(_input_file: &str) -> usize {
    let mut seq = vec![2, 4, 1, 3, 7, 5, 0, 3, 1, 5, 4, 1, 5, 5, 3, 0];
    seq.reverse();
    recur(0, 0, &seq)
}
