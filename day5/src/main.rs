use std::fs;

#[derive(Debug, Clone)]
pub struct Day5 {
    pub seeds: Vec<i64>,
    pub maps: Vec<CatMap>,
    pub ranges: Vec<Range>,
}

impl Day5 {
    pub(crate) fn convert_to_range(&mut self) {
        let mut iter = self.seeds.iter();
        let mut left = iter.next();
        while left.is_some() {
            match left  {
                Some(x) => {
                    let right = iter.next().unwrap();
                    self.ranges.push(Range { start: *x, len: *right});
                    left = iter.next();
                },
                None => ()// the end
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CatMap {
    pub mappings: Vec<Mapping>,
}
#[derive(Debug, Clone)]
pub struct Mapping {
    pub dest: i64,
    pub src: i64,
    pub range: i64,
}
impl Mapping {
    pub fn right(&self) -> i64 {
        self.src + self.range - 1
    }
}
#[derive(Debug, Clone)]
pub struct Range {
    start: i64,
    len: i64,
}
impl Range {
  pub fn right(&self) -> i64 {
      self.start + self.len - 1
  }
}
fn main() {
    let contents: String = fs::read_to_string("example").unwrap();
    let mut day5 = read_parse(contents);
    day5.convert_to_range();
    println!("{:?}", day5.seeds);
    println!("{:?}", day5.ranges);

    // let mut all_ranges = vec!();
    let start: Range = day5.ranges[0].clone();

    let mut start: Vec<Range> = vec!(start.clone());
    let mut done: Vec<Range> = vec!();
    let mut keep: Vec<Range> = vec!();
    for cm in day5.maps {
        for r in &start {
            for m in &cm.mappings {
                println!("{:?}", m);
                while keep.i
                let (mut d, mut k) = do_round(&m, &r);
                done.append(&mut d);
                keep.append(&mut k);
                println!("Keep {:?}", keep);
                println!("Done {:?}", done);
            }
        }
        start.clear();
        start.append(&mut done);
        start.append(&mut keep);
        done.clear();
        keep.clear();
    }
    println!("{:?}",start);
}
// return (vec of ranges that have moved, vec of ranges unaffected
pub fn do_round(m: &Mapping, cr: &Range) -> (Vec<Range>, Vec<Range>) {
    let mut done = vec!();
    let mut keep = vec!();
    if m.src >= (cr.start + cr.len) || (m.src + m.range) < cr.start {
        // entire range stays the same
        keep.push(cr.clone());
    } else if m.src <= cr.start && m.src + m.range >= cr.start + cr.len {
        // full coverage, move all
        done.push(Range { start: m.dest - m.src + cr.start, len: cr.len })
    } else if m.src > cr.start && m.src + m.range < cr.start + cr.len {
        // full containment, make three new ranges
        let r1 = Range { start: cr.start, len: m.src - cr.start };
        let r2 = Range { start: m.dest, len: m.range };
        let r3 = Range { start: m.src + m.range, len: cr.right() - m.right() };
        keep.push(r1);
        done.push(r2);
        keep.push(r3);
    } else if m.src < cr.start {
        // two ranges, left overlapping
        let r1 = Range { start: cr.start + m.dest - m.src, len: m.right() - cr.start + 1 };
        let r2 = Range { start: m.right() + 1, len: cr.right() - m.right() };
        done.push(r1);
        keep.push(r2);
    } else {
        // two ranges, right overlapping
        let r1 = Range { start: cr.start, len: m.src - cr.start };
        let r2 = Range { start: m.dest, len: cr.len - (m.src - cr.start) };
        keep.push(r1);
        done.push(r2);
    }
    (done, keep)
}
fn part1() {
    let contents: String = fs::read_to_string("input").unwrap();
    let day5 = read_parse(contents);
    println!("{:?}, {}", day5, day5.maps.len());
    let result = day5.seeds.iter().map(|s| follow(*s, &day5)).min().unwrap();
    println!("{}", result);
}
pub fn read_parse(contents: String) -> Day5 {
    let mut day5 = Day5 {
        seeds: vec![],
        maps: vec![],
        ranges: vec![],
    };
    let mut cat_map = CatMap { mappings: vec![] };
    let mut mapping = Mapping {
        dest: 0,
        src: 0,
        range: 0,
    };
    for line in contents.lines() {
        if line.starts_with("seeds:") {
            let seeds = line.split_once(":").unwrap().1;
            day5.seeds = seeds
                .trim()
                .split(" ")
                .map(|e| e.trim().parse().unwrap())
                .collect::<Vec<i64>>();
        } else if line.len() == 0 {
            // skip
        } else if line.chars().next().unwrap().is_digit(10) {
            let nums = line
                .split(" ")
                .map(|n| n.trim().parse().unwrap())
                .collect::<Vec<i64>>();
            mapping.dest = nums[0];
            mapping.src = nums[1];
            mapping.range = nums[2];
            cat_map.mappings.push(mapping.clone());
        } else {
            if cat_map.mappings.len() > 0 {
                day5.maps.push(cat_map.clone());
                cat_map.mappings.clear();
            }
        }
    }
    day5.maps.push(cat_map);
    day5
}
pub fn follow(start: i64, day5: &Day5) -> i64 {
    let mut end = start;
    for cm in &day5.maps {
        let mut next = end;
        for m in &cm.mappings {
            if next >= m.src && next < m.src + m.range {
                next = m.dest - m.src + next;
                break;
            }
        }
        end = next;
    }
    end
}
