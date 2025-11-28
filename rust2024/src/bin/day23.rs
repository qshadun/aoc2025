#![allow(clippy::collapsible_else_if)]
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let day = 23;
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

fn parse_input(input_file: &str) -> (HashSet<String>, HashMap<String, HashSet<String>>) {
    let mut nodes = HashSet::new();
    let mut adj: HashMap<String, HashSet<String>> = HashMap::new();
    let s = read_to_string(input_file).unwrap();
    for line in s.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        let a = parts[0];
        let b = parts[1];
        nodes.insert(a.to_string());
        nodes.insert(b.to_string());
        adj.entry(a.to_string()).or_default().insert(b.to_string());
        adj.entry(b.to_string()).or_default().insert(a.to_string());
    }

    (nodes, adj)
}

fn part1(input_file: &str) -> usize {
    let (nodes, adj) = parse_input(input_file);
    let mut triples = HashSet::new();
    for n1 in nodes.iter() {
        for n2 in adj.get(n1).unwrap() {
            for n3 in adj.get(n2).unwrap() {
                if adj.get(n3).unwrap().contains(n1)
                    && (n1.starts_with('t') || n2.starts_with('t') || n3.starts_with('t'))
                {
                    let mut tmp = [n1, n2, n3];
                    tmp.sort();
                    let triple = (tmp[0], tmp[1], tmp[2]);
                    triples.insert(triple);
                }
            }
        }
    }
    triples.len()
}

#[allow(non_snake_case)]
fn bron_kerbosch2(
    R: &mut HashSet<String>,
    P: &mut HashSet<String>,
    X: &mut HashSet<String>,
    adj: &HashMap<String, HashSet<String>>,
    cliques: &mut Vec<Vec<String>>,
) {
    if P.is_empty() && X.is_empty() {
        let mut cliq: Vec<String> = R.iter().cloned().collect();
        cliq.sort();
        cliques.push(cliq);
        return;
    }
    let u = P.union(X).next().unwrap();
    let u_adj = adj.get(u).unwrap();
    let vs: Vec<String> = P.difference(u_adj).cloned().collect();
    for v in vs {
        let mut new_R = R.clone();
        new_R.insert(v.to_string());
        let v_adj = adj.get(&v).unwrap();
        let mut new_P = P.clone();
        new_P = new_P.intersection(v_adj).cloned().collect();
        let mut new_X = X.clone();
        new_X = new_X.intersection(v_adj).cloned().collect();
        bron_kerbosch2(&mut new_R, &mut new_P, &mut new_X, adj, cliques);
        P.remove(&v);
        X.insert(v);
    }
}

#[allow(non_snake_case)]
fn part2(input_file: &str) -> String {
    let (nodes, adj) = parse_input(input_file);
    let mut cliques = vec![];
    let mut R = HashSet::new();
    let mut P = nodes;
    let mut X = HashSet::new();
    bron_kerbosch2(&mut R, &mut P, &mut X, &adj, &mut cliques);
    let mut max_len = 0;
    let mut ans: String = String::new();
    for c in cliques {
        if c.len() > max_len {
            max_len = c.len();
            ans = c.join(",");
        }
    }
    ans
}
