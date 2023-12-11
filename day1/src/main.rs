use std::collections::HashMap;
use std::fs;

fn main() {
    let input1 = String::from(
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
    );
    println!("{}", input1);

    let s1 = String::from("abc1def3h");

    println!("{}", calibrate(s1));

    let mut total = 0;
    for s in input1.lines() {
        total = total + calibrate(s.to_string())
    }
    println!("{}", total);

    let contents = fs::read_to_string("input.txt").unwrap();
    let mut total2 = 0;
    for s in contents.lines() {
        total2 = total2 + calibrate(s.to_string());
    }
    println!("{}", total2);
    part2()
}
pub fn part2() {
    println!("Start of part2");
    let example = String::from(
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
    );
    // let example = String::from("two1nine");
    let mut sum = 0;
    for line in example.lines() {
        println!("{}", line);
        let res = calibrate2(line.to_string());
        sum = sum + res;
    }
    println!("{}", sum);

    let contents = fs::read_to_string("input.txt").unwrap();
    let mut total2 = 0;
    for s in contents.lines() {
        total2 = total2 + calibrate2(s.to_string());
    }
    println!("{}", total2);
}
pub fn calibrate(s: String) -> u64 {
    let l = s.chars().find(|c| c.is_digit(10));
    let r = s.chars().rev().find(|c| c.is_digit(10));

    let answer = l.unwrap().to_digit(10).unwrap() * 10 + r.unwrap().to_digit(10).unwrap();
    answer
}
pub fn calibrate2(target: String) -> u64 {
    let nums = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
        String::from("five"),
        String::from("six"),
        String::from("seven"),
        String::from("eight"),
        String::from("nine"),
    ];
    let nums_rev = nums
        .iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect::<Vec<String>>();

    let digits = vec![
        String::from("1"),
        String::from("2"),
        String::from("3"),
        String::from("4"),
        String::from("5"),
        String::from("6"),
        String::from("7"),
        String::from("8"),
        String::from("9"),
    ];

    let mut all = nums;
    all.append(&mut digits.clone());
    let mut all_rev = nums_rev;
    all_rev.append(&mut digits.clone());
    let x = all
        .iter()
        .map(|s| (s, target.find(s)))
        .collect::<Vec<(&String, Option<usize>)>>();
    let y = x
        .iter()
        .filter(|t| t.1.is_some())
        .map(|t| (t.0, t.1.unwrap()))
        .collect::<Vec<(&String, usize)>>();
    let mut min = 100000;
    let mut min_s = String::from("");
    for y1 in y {
        if y1.1 < min {
            min = y1.1;
            min_s = y1.0.clone();
        }
    }
    let mut min2 = 100000;
    let mut min_s2 = String::from("");
    let oefen_rev = target.chars().rev().collect::<String>();
    let x3 = all_rev
        .iter()
        .map(|s| (s, oefen_rev.find(s)))
        .collect::<Vec<(&String, Option<usize>)>>();
    let y3 = x3
        .iter()
        .filter(|t| t.1.is_some())
        .map(|t| (t.0, t.1.unwrap()))
        .collect::<Vec<(&String, usize)>>();
    // for z in
    for y4 in y3 {
        if y4.1 < min2 {
            min2 = y4.1;
            min_s2 = y4.0.chars().rev().collect::<String>();
        }
    }

    to_value(&min_s) * 10 + to_value(&min_s2)
}
pub fn to_value(s: &String) -> u64 {
    let values = HashMap::from([
        (String::from("one"), 1),
        (String::from("two"), 2),
        (String::from("three"), 3),
        (String::from("four"), 4),
        (String::from("five"), 5),
        (String::from("six"), 6),
        (String::from("seven"), 7),
        (String::from("eight"), 8),
        (String::from("nine"), 9),
    ]);

    let c0 = s.chars().next().unwrap();
    if c0.is_digit(10) {
        c0.to_digit(10).unwrap()
    } else {
        values.get(s).unwrap().clone()
    }
}
