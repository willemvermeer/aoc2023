use std::collections::HashSet;
use std::fs;

type Point = (i16, i16);
pub struct Day10 {
    fields: Vec<Vec<char>>,
}
impl Day10 {
    pub fn find_s(&self) -> Point {
        for (i, f) in self.fields.iter().enumerate() {
            for (j, g) in f.iter().enumerate() {
                if *g == 'S' {
                    return (j as i16, i as i16);
                }
            }
        }
        (0, 0)
    }
    pub fn len(&self) -> i16 {
        self.fields.len() as i16
    }
    pub fn char_at(&self, p: &Point) -> char {
        self.fields[p.1 as usize][p.0 as usize]
    }
    pub fn connectors(&self, p: &Point) -> Vec<Point> {
        let mut neighbors: Vec<Point> = vec![];
        if p.0 > 0 {
            let me = self.char_at(&p);
            let p2 = (p.0 - 1, p.1);
            if vec!['F', 'L', '-'].contains(&self.char_at(&p2))
                && vec!['-', 'J', '7', 'S'].contains(&me)
            {
                neighbors.push(p2);
            }
        }
        if p.0 < self.len() - 1 {
            let me = self.char_at(&p);
            let p2 = (p.0 + 1, p.1);
            if vec!['J', '7', '-'].contains(&self.char_at(&p2))
                && vec!['-', 'F', 'L', 'S'].contains(&me)
            {
                neighbors.push(p2);
            }
        }
        if p.1 > 0 {
            let me = self.char_at(&p);
            let p2 = (p.0, p.1 - 1);
            if vec!['F', '7', '|'].contains(&self.char_at(&p2))
                && vec!['|', 'L', 'J', 'S'].contains(&me)
            {
                neighbors.push(p2);
            }
        }
        if p.1 < self.len() - 1 {
            let me = self.char_at(&p);
            let p2 = (p.0, p.1 + 1);
            if vec!['J', 'L', '|'].contains(&self.char_at(&p2))
                && vec!['|', '7', 'F', 'S'].contains(&me)
            {
                neighbors.push(p2);
            }
        }
        neighbors
    }
}
fn main() {
    let day10 = read_parse("input");
    let p_s = day10.find_s();
    println!("Start {:?} ", p_s);
    // println!("{:?}", day10.connectors(&(109,41)));
    let mut start = p_s.clone();
    let mut path: HashSet<Point> = HashSet::new();
    let mut indx = 0;
    loop {
        let next_two = day10.connectors(&start);
        println!("NExt two {:?}", next_two);
        if next_two.len() == 3 {
            let cs = next_two
                .iter()
                .map(|p| format!("{},{}:{}", p.0, p.1, day10.char_at(p)))
                .collect::<Vec<String>>();
            println!("{:?}", cs);
        }
        if next_two.len() == 1 && path.len() > 1 {
            println!("break1");
            break;
        }
        let next = if path.contains(&next_two[0]) {
            next_two[1]
        } else {
            next_two[0]
        };
        println!("Next = {:?}", next);
        if next == p_s && path.len() > 1 {
            println!("break2");
            break;
        } else {
            path.insert(next);
            start = next;
        }
        println!("{:?} {} {}", next, indx, day10.char_at(&next));
        indx += 1;
    }
    println!("Result is {}", (indx + 1) / 2);
    // println!("{:?}", day10.connectors(&(1 as i16,1 as i16)));
}
pub fn read_parse(filename: &str) -> Day10 {
    let contents: String = fs::read_to_string(filename).unwrap();
    let mut all: Vec<Vec<char>> = vec![];
    for line in contents.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        all.push(chars);
    }
    Day10 { fields: all }
}
