use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

pub struct Day7 {
    all: HashMap<Hand, u64>,
}
fn main() {
    let day7 = read("input");
    for h in &day7.all {
        println!("{:?} {:?}", h, h.0.get_type());
    }
    let mut all_hands = day7.all.iter().map(|t| t.0.clone()).collect::<Vec<Hand>>();
    all_hands.sort_by(|h1, h2| {
        if h1.get_type() == h2.get_type() {
            // same type, char-by-char comparison
            let mut result = Ordering::Equal;
            for i in 0..5 {
                if card_value(&h1.h.chars().collect::<Vec<char>>()[i])
                    < card_value(&h2.h.chars().collect::<Vec<char>>()[i])
                {
                    result = Ordering::Less;
                    break;
                } else if card_value(&h1.h.chars().collect::<Vec<char>>()[i])
                    > card_value(&h2.h.chars().collect::<Vec<char>>()[i])
                {
                    result = Ordering::Greater;
                    break;
                }
            }
            result
        } else {
            h1.get_type().ord().cmp(&h2.get_type().ord())
        }
    });
    println!("Sorted {:?}", all_hands);
    let mut rank: u64 = all_hands.len() as u64;
    let mut total = 0;
    for h in &all_hands {
        total = total + day7.all.get(&h).unwrap() * rank;
        rank = rank - 1;
    }
    println!("{}", total);
}
pub fn card_value(c: &char) -> u64 {
    if *c == 'A' {
        0
    } else if *c == 'K' {
        1
    } else if *c == 'Q' {
        2
    } else if *c == 'J' {
        3
    } else if *c == 'T' {
        4
    } else {
        let z = (*c).to_digit(10).unwrap() as u64;
        5 + 9 - z
    }
}
pub fn read(filename: &str) -> Day7 {
    let contents: String = fs::read_to_string(filename).unwrap();
    let mut all: HashMap<Hand, u64> = HashMap::new();
    for line in contents.lines() {
        let spl = line.split_once(" ").unwrap();
        all.insert(Hand::of(spl.0), spl.1.parse().unwrap());
    }
    Day7 { all: all }
}
#[derive(Debug, PartialEq)]
pub enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    HighCard,
}
impl HandType {
    pub fn ord(&self) -> i8 {
        match self {
            HandType::Five => 0,
            HandType::Four => 1,
            HandType::FullHouse => 2,
            HandType::Three => 3,
            HandType::TwoPair => 4,
            HandType::OnePair => 5,
            HandType::HighCard => 6,
        }
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Hand {
    pub h: String,
}
impl Hand {
    pub fn of(str: &str) -> Hand {
        Hand { h: str.to_string() }
    }
    // pub fn sorted(&self) -> String {
    //     let mut v = self.h.clone().chars().collect::<Vec<char>>();
    //     v.sort();
    //     v.join("")
    // }
    pub fn get_type(&self) -> HandType {
        let m = self.map_draws();
        if m.len() == 1 {
            HandType::Five
        } else if m.len() == 2 {
            let mut has_four = false;
            for (_, value) in m {
                if value == 4 {
                    has_four = true;
                }
            }
            if has_four {
                HandType::Four
            } else {
                HandType::FullHouse
            }
        } else if m.len() == 3 {
            let mut has_three = false;
            for (_, value) in m {
                if value == 3 {
                    has_three = true;
                }
            }
            if has_three {
                HandType::Three
            } else {
                HandType::TwoPair
            }
        } else if m.len() == 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
    fn map_draws(&self) -> HashMap<char, u8> {
        let mut m: HashMap<char, u8> = HashMap::new();
        for c in self.h.chars() {
            if m.contains_key(&c) {
                let current = m.get(&c).unwrap();
                m.insert(c, current + 1);
            } else {
                m.insert(c, 1);
            }
        }
        m
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hand_type() {
        assert_eq!(HandType::Five, Hand::of("11111").get_type());
        assert_eq!(HandType::Four, Hand::of("11112").get_type());
        assert_eq!(HandType::FullHouse, Hand::of("11122").get_type());
        assert_eq!(HandType::Three, Hand::of("11123").get_type());
        assert_eq!(HandType::TwoPair, Hand::of("11223").get_type());
        assert_eq!(HandType::OnePair, Hand::of("11234").get_type());
        assert_eq!(HandType::HighCard, Hand::of("12345").get_type());
    }
}
