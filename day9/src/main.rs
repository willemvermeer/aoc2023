use std::fs;

fn main() {
    let day9 = read_parse("input");
    let mut total = 0;
    for d in day9 {
        let result = solve2(d);
        total = total + result;
    }
    println!("{}", total);
}
pub fn solve1(input: Vec<isize>) -> isize {
    let mut smallers: Vec<Vec<isize>> = vec![];
    smallers.push(input);
    while smallers[smallers.len() - 1]
        .iter()
        .filter(|i| **i != 0)
        .collect::<Vec<&isize>>()
        .len()
        > 0
    {
        let current = &smallers[smallers.len() - 1];
        let mut vec_new: Vec<isize> = vec![];
        for window in current.windows(2) {
            vec_new.push(window[1] - window[0]);
        }
        smallers.push(vec_new);
    }
    // println!("{:?}", smallers);
    let len = smallers.len() - 1;
    smallers[len].push(0);
    for i in 0..(smallers.len() - 1) {
        let index = smallers.len() - 2 - i;
        let x1 = smallers[index + 1][smallers[index + 1].len() - 1];
        let x2 = smallers[index][smallers[index].len() - 1];
        smallers[index].push(x1 + x2);
    }
    println!("{:?}", smallers);
    let z = smallers[0].len() - 1;
    smallers[0][z]
}
pub fn solve2(input: Vec<isize>) -> isize {
    let mut smallers: Vec<Vec<isize>> = vec![];
    smallers.push(input);
    while smallers[smallers.len() - 1]
        .iter()
        .filter(|i| **i != 0)
        .collect::<Vec<&isize>>()
        .len()
        > 0
    {
        let current = &smallers[smallers.len() - 1];
        let mut vec_new: Vec<isize> = vec![];
        for window in current.windows(2) {
            vec_new.push(window[1] - window[0]);
        }
        smallers.push(vec_new);
    }
    // println!("{:?}", smallers);
    let len = smallers.len() - 1;
    smallers[len].push(0);
    for i in 0..(smallers.len() - 1) {
        let index = smallers.len() - 2 - i;
        let x1 = smallers[index + 1][0];
        let x2 = smallers[index][0];
        let mut q = smallers[index].clone();
        q.reverse();
        q.push(x2 - x1);
        q.reverse();
        smallers[index] = q.clone();
    }
    println!("{:?}", smallers);
    smallers[0][0]
}
pub fn read_parse(filename: &str) -> Vec<Vec<isize>> {
    let contents: String = fs::read_to_string(filename).unwrap();
    let mut all: Vec<Vec<isize>> = vec![];
    for line in contents.lines() {
        let nums = line
            .split(" ")
            .map(|n| n.trim().parse().unwrap())
            .collect::<Vec<isize>>();
        all.push(nums);
    }
    all
}
