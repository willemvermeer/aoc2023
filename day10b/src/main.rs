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
    pub fn len_y(&self) -> i16 {
        self.fields.len() as i16
    }
    pub fn len_x(&self) -> i16 {
        self.fields[0].len() as i16
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
        if p.0 < self.len_x() - 1 {
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
        if p.1 < self.len_y() - 1 {
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
    let mut path_vec: Vec<Point> = vec![];
    path_vec.push(start.clone());
    let mut indx = 0;
    loop {
        let next_two = day10.connectors(&start);
        if next_two.len() == 1 && path.len() > 1 {
            println!("break1");
            break;
        }
        let next = if path.contains(&next_two[0]) {
            next_two[1]
        } else {
            next_two[0]
        };
        if next == p_s && path.len() > 1 {
            println!("break2");
            break;
        } else {
            path.insert(next);
            path_vec.push(next.clone());
            start = next;
        }
        println!("{:?} {} {}", next, indx, day10.char_at(&next));
        indx += 1;
    }
    println!("The path was {:?}", path);
    println!("Result is {}", (indx + 1) / 2);
    let mut enclosed_left: HashSet<Point> = HashSet::new();
    let mut enclosed_right: HashSet<Point> = HashSet::new();
    for window in path_vec.windows(2) {
        let p1: Point = window[1];
        let p0: Point = window[0];
        let c = day10.char_at(&p1);
        if p1.1 > p0.1 {
            if p1.0 < day10.len_x() - 1 {
                let left = (p1.0 + 1, p1.1);
                enclosed_left.extend(neighbors_not_on_path(&left, &path, &day10));
            }
            if p1.0 > 0 {
                let right = (p1.0 - 1, p1.1);
                enclosed_right.extend(neighbors_not_on_path(&right, &path, &day10));
            }
            if (c == 'L' || c == 'J') && p1.1 < day10.len_y() - 1 {
                let third = (p1.0, p1.1 + 1);
                if c == 'L' {
                    enclosed_right.extend(neighbors_not_on_path(&third, &path, &day10));
                } else {
                    enclosed_left.extend(neighbors_not_on_path(&third, &path, &day10));
                }
            }
        }
        if p1.1 < p0.1 {
            if p1.0 > 0 {
                let left = (p1.0 - 1, p1.1);
                enclosed_left.extend(neighbors_not_on_path(&left, &path, &day10));
            }
            if p1.0 < day10.len_x() - 1 {
                let right = (p1.0 + 1, p1.1);
                enclosed_right.extend(neighbors_not_on_path(&right, &path, &day10));
            }
            if (c == '7' || c == 'F') && p1.1 > 0 {
                let third = (p1.0, p1.1 - 1);
                if c == '7' {
                    enclosed_right.extend(neighbors_not_on_path(&third, &path, &day10));
                } else {
                    enclosed_left.extend(neighbors_not_on_path(&third, &path, &day10));
                }
            }
        }
        if p1.0 > p0.0 {
            if p1.1 > 0 {
                let left = (p1.0, p1.1 - 1);
                enclosed_left.extend(neighbors_not_on_path(&left, &path, &day10));
            }
            if p1.1 < day10.len_y() - 1 {
                let right = (p1.0, p1.1 + 1);
                enclosed_right.extend(neighbors_not_on_path(&right, &path, &day10));
            }
            if (c == '7' || c == 'J') && p1.0 < day10.len_x() - 1 {
                let third = (p1.0 + 1, p1.1);
                if c == 'J' {
                    enclosed_right.extend(neighbors_not_on_path(&third, &path, &day10));
                } else {
                    enclosed_left.extend(neighbors_not_on_path(&third, &path, &day10));
                }
            }
        }
        if p1.0 < p0.0 {
            if p1.1 < day10.len_y() - 1 {
                let left = (p1.0, p1.1 + 1);
                enclosed_left.extend(neighbors_not_on_path(&left, &path, &day10));
            }
            if p1.1 > 0 {
                let right = (p1.0, p1.1 - 1);
                enclosed_right.extend(neighbors_not_on_path(&right, &path, &day10));
            }
            if (c == '7' || c == 'J') && p1.0 > 0 {
                let third = (p1.0 - 1, p1.1);
                if c == 'F' {
                    enclosed_right.extend(neighbors_not_on_path(&third, &path, &day10));
                } else {
                    enclosed_left.extend(neighbors_not_on_path(&third, &path, &day10));
                }
            }
        }
        // println!("Handled {:?}->{:?} left-{:?} right-{:?}", p0, p1, enclosed_left, enclosed_right);
    }
    enclosed_left.remove(&p_s.clone());
    enclosed_right.remove(&p_s.clone());
    println!(
        "left right {:?} {:?}",
        enclosed_left.len(),
        enclosed_right.len()
    );
    // println!("{:?}", enclosed_left);
    // println!("{:?}", enclosed_right);
    for y in 0..day10.len_y() {
        for x in 0..day10.len_x() {
            let p = (x, y);
            if p == p_s {
                print!("S");
            } else if path.contains(&p) {
                print!("{}", day10.fields[y as usize][x as usize])
            } else if enclosed_left.contains(&p) {
                print!("O")
            } else if enclosed_right.contains(&p) {
                print!("I")
            } else {
                print!("{}", day10.fields[y as usize][x as usize])
            }
        }
        println!();
    }
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
pub fn neighbors_not_on_path(
    start: &Point,
    path: &HashSet<Point>,
    day10: &Day10,
) -> HashSet<Point> {
    if path.contains(start) {
        return HashSet::new();
    }
    let mut to_do = HashSet::new();
    let mut result_set = HashSet::new();
    let mut discarded = HashSet::new();
    to_do.insert(start.clone());
    // println!("Starting with {:?}", start);
    loop {
        let current = to_do.iter().next().unwrap().clone();
        to_do.remove(&current);
        result_set.insert(current.clone());
        // println!("Current is now {:?} and todo {:?}", current, to_do);
        let neighbors = neighbors(&current, &day10);
        // println!("Found neighbors of {:?} {:?}", current, neighbors);
        for n in neighbors {
            if path.contains(&n) {
                // println!("Discarded {:?}", n);
                discarded.insert(n);
            } else if result_set.contains(&n) {
                // println!("Skipped {:?}", n);
                // skip
            } else {
                // println!("A new to do {:?}", n);
                to_do.insert(n);
                // println!("To do is now {:?}", to_do);
            }
        }
        if to_do.len() == 0 {
            break;
        } // done
    }
    result_set
}
pub fn neighbors(p: &Point, day10: &Day10) -> HashSet<Point> {
    let mut result = HashSet::new();
    if p.0 > 0 {
        result.insert((p.0 - 1, p.1));
    }
    if p.0 < day10.len_x() - 1 {
        result.insert((p.0 + 1, p.1));
    }
    if p.1 > 0 {
        result.insert((p.0, p.1 - 1));
    }
    if p.1 < day10.len_y() - 1 {
        result.insert((p.0, p.1 + 1));
    }
    result
}
