use std::fs;

#[derive(Debug, Clone)]
pub struct Card {
    nr: usize,
    pub wins: Vec<usize>,
    pub mine: Vec<usize>,
}

impl Card {
    pub fn score(&self) -> i32 {
        let w = self
            .mine
            .iter()
            .filter(|c| self.wins.contains(c))
            .collect::<Vec<&usize>>();
        if w.is_empty() {
            0
        } else {
            i32::pow(2, (w.len() - 1) as u64)
        }
    }
    pub fn num_wins(&self) -> usize {
        self.mine
            .iter()
            .filter(|c| self.wins.contains(c))
            .collect::<Vec<&usize>>()
            .len()
    }
}

fn main() {
    let contents: String = fs::read_to_string("input").unwrap();
    let cards = parse(contents);
    let mut result = 0;
    for card in cards.clone() {
        println!("{:?} score {}", card, card.score());
        result = result + card.score();
    }
    println!("The answer is {}", result);
    let mut with_copies: Vec<Vec<Card>> = vec![];
    for card in cards.clone() {
        with_copies.push(vec![card.clone()]);
    }
    for card in cards.clone() {
        let wins = card.num_wins();
        let nr = card.nr;
        let nr_cards = with_copies[nr - 1].len();
        for i in 0..wins {
            for j in 0..nr_cards {
                let cr = with_copies[i + card.nr][0].clone();
                with_copies[i + card.nr].push(cr);
            }
        }
    }
    println!("asdas");
    for w in &with_copies {
        println!("Card {} copies {}", w[0].nr, w.len());
    }
    println!("asdas");
    let mut total = 0;
    for w in &with_copies {
        if w[0].nr <= 192 {
            println!("asdas");
            total = total + w.len();
        }
    }
    println!("Oplossing {}", total);
    // println!("{:?}", with_copies);
}
fn parse(str: String) -> Vec<Card> {
    let mut result = vec![];
    for line in str.lines() {
        let (crd, draws) = line.split_once(":").unwrap();
        let nr = &crd.trim()[5..].trim();
        let (wins, mine) = draws.trim().split_once("|").unwrap();
        let card = Card {
            nr: nr.parse().unwrap(),
            wins: wins
                .trim()
                .split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| s.trim().parse())
                .filter(|s| s.is_ok())
                .map(|s| s.unwrap())
                .collect::<Vec<usize>>(),
            mine: mine
                .trim()
                .split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| s.trim().parse())
                .filter(|s| s.is_ok())
                .map(|s| s.unwrap())
                .collect::<Vec<usize>>(),
        };
        result.push(card);
    }
    result
}
