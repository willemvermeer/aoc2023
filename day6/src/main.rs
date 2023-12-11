use std::fs;

#[derive(Debug)]
pub struct Day6 {
    pub times: Vec<usize>,
    pub distances: Vec<usize>,
}
fn main() {
    // part1();
    part2();
}
pub fn part2() {
    let contents: String = fs::read_to_string("input").unwrap();
    let mut iter = contents.lines().into_iter();
    let day6 = Day6 {
        times: iter
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split(" ")
            .into_iter()
            .filter(|n| n.trim().len() > 0)
            .map(|n| n.trim().parse().unwrap())
            .collect::<Vec<usize>>(),
        distances: iter
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split(" ")
            .into_iter()
            .filter(|n| n.trim().len() > 0)
            .map(|n| n.trim().parse().unwrap())
            .collect::<Vec<usize>>(),
    };
    println!("{:?}", day6);
    let t: usize = day6
        .times
        .iter()
        .map(|a| a.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();
    let d: usize = day6
        .distances
        .iter()
        .map(|a| a.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();
    println!("{} {}", t, d);
    let mut num_wins = 0;
    for inc in 0..t {
        if is_win(inc, t, d) {
            num_wins = num_wins + 1;
        };
    }
    println!("{:?}", num_wins);
}
pub fn part1() {
    let contents: String = fs::read_to_string("input").unwrap();
    let mut iter = contents.lines().into_iter();
    let day6 = Day6 {
        times: iter
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split(" ")
            .into_iter()
            .filter(|n| n.trim().len() > 0)
            .map(|n| n.trim().parse().unwrap())
            .collect::<Vec<usize>>(),
        distances: iter
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split(" ")
            .into_iter()
            .filter(|n| n.trim().len() > 0)
            .map(|n| n.trim().parse().unwrap())
            .collect::<Vec<usize>>(),
    };
    println!("{:?}", day6);
    let mut index = 0;
    let mut wins = vec![];
    let mut product = 1;
    for t in day6.times {
        let d = day6.distances[index];
        let mut num_wins = 0;
        for inc in 0..t {
            if is_win(inc, t, d) {
                num_wins = num_wins + 1;
            };
        }
        wins.push(num_wins);
        product = product * num_wins;
        index = index + 1;
    }
    println!("{:?} {}", wins, product);
}
pub fn dist(start_speed: usize, total_time: usize) -> usize {
    (total_time - start_speed) * start_speed
}
pub fn is_win(wait_time: usize, max_time: usize, record: usize) -> bool {
    dist(wait_time, max_time) > record
}
