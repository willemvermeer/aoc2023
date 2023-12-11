use std::{fs, num};

type Point = (i64, i64);
pub struct Day11 {
    lines: Vec<Vec<char>>,
}

impl Day11 {
    pub(crate) fn expand(&self) -> Day11 {
        let mut row_copy: Vec<Vec<char>> = vec![];
        for row in 0..self.lines.len() {
            row_copy.push(self.lines[row].clone());
            if self.lines[row].iter().find(|&&c| c == '#').is_none() {
                row_copy.push(self.lines[row].clone());
            }
        }
        let mut col_copy: Vec<Vec<char>> = vec![];
        for _ in 0..row_copy.len() {
            col_copy.push(vec![]);
        }
        for col in 0..self.lines[0].len() {
            let mut col_has_galaxy = false;
            for row in 0..row_copy.len() {
                if row_copy[row][col] == '#' {
                    col_has_galaxy = true;
                }
            }
            for row in 0..row_copy.len() {
                col_copy[row].push(row_copy[row][col]);
                if !col_has_galaxy {
                    col_copy[row].push(row_copy[row][col]);
                }
            }
        }
        Day11 { lines: col_copy }
    }
    pub fn print(&self) {
        for l in &self.lines {
            println!(
                "{:?}",
                l.iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            );
        }
    }
    pub fn get_galaxies(self) -> Vec<Point> {
        let mut result: Vec<Point> = vec![];
        for row in 0..self.lines.len() {
            for col in 0..self.lines[0].len() {
                let p = (col as i64, row as i64);
                if self.lines[row][col] == '#' {
                    result.push(p);
                }
            }
        }
        result
    }
}

fn main() {
    let day11 = read_parse("input");
    day11.print();
    let day11 = day11.expand();
    day11.print();
    let p1 = (1, 6);
    let p2 = (5, 11);
    println!("{}", distance(&p1, &p2));
    let galaxies = day11.get_galaxies();
    println!("{:?}", galaxies);
    let mut distances: i64 = 0;
    for g1 in &galaxies {
        for g2 in &galaxies {
            if g1 != g2 {
                distances += distance(&g1, &g2);
            }
        }
    }
    println!("Answer 1 {}", distances / 2);
}
pub fn distance(p1: &Point, p2: &Point) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}
pub fn read_parse(filename: &str) -> Day11 {
    let contents: String = fs::read_to_string(filename).unwrap();
    let mut all: Vec<Vec<char>> = vec![];
    for line in contents.lines() {
        all.push(line.chars().collect());
    }
    Day11 { lines: all }
}
