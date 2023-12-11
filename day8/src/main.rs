use std::collections::HashMap;
use std::{fs, num};

#[derive(Debug)]
pub struct Day8 {
    directions: String,
    instrs: HashMap<String, Instr>,
}
impl Day8 {
    pub fn get(&self, key: &String) -> Option<&Instr> {
        self.instrs.get(key)
    }
}
#[derive(Debug, Clone)]
pub struct Instr {
    pub left: String,
    pub right: String,
}
fn main() {
    let day8 = read_parse("input");
    println!("{}", part2(&day8));
}

pub fn part2(day8: &Day8) -> usize {
    let mut end_on_a: Vec<String> = vec![];
    for key in day8.instrs.keys().into_iter() {
        if key.ends_with("A") {
            end_on_a.push(key.clone());
        }
    }
    let path_lens = end_on_a
        .iter()
        .map(|s| from_to(s, &day8))
        .collect::<Vec<usize>>();
    println!("{:?} {:?}", end_on_a, path_lens);
    path_lens.iter().fold(path_lens[0], |acc, v| lcm(acc, *v))
}
fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
pub fn from_to(start: &String, day8: &Day8) -> usize {
    let mut loop_var = start.clone();
    let mut num_steps = 0;
    while !loop_var.ends_with("Z") {
        for c in day8.directions.chars() {
            let instr = day8.get(&loop_var).unwrap();
            if c == 'L' {
                loop_var = instr.left.clone();
            } else {
                loop_var = instr.right.clone();
            }
            num_steps = num_steps + 1;
        }
    }
    num_steps
}
pub fn part1(day8: &Day8) -> usize {
    let mut start = String::from("AAA");
    let mut num_steps = 0;
    while start != String::from("ZZZ") {
        for c in day8.directions.chars() {
            let instr = day8.get(&start).unwrap();
            if c == 'L' {
                start = instr.left.clone();
            } else {
                start = instr.right.clone();
            }
            num_steps = num_steps + 1;
        }
    }
    num_steps
}
pub fn read_parse(filename: &str) -> Day8 {
    let contents: String = fs::read_to_string(filename).unwrap();
    let mut instrs: HashMap<String, Instr> = HashMap::new();
    let lines = contents.lines();
    let mut iter = lines.into_iter();
    let top = iter.next().unwrap();
    iter.next(); // skip empty line
    for l in iter {
        let (start, tuple) = l.split_once("=").unwrap();
        let (left, right) = tuple.split_once(",").unwrap();
        let instr = Instr {
            left: left.trim().replace("(", "").to_string(),
            right: right.trim().replace(")", "").to_string(),
        };
        instrs.insert(start.trim().to_string(), instr);
    }
    let day8 = Day8 {
        directions: top.to_string(),
        instrs: instrs,
    };
    day8
}
