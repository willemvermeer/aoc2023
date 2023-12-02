use std::fs;

fn main() {
    let contents = fs::read_to_string("example.txt").unwrap();
    let limits = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = contents
        .lines()
        .into_iter()
        .map(|l| parse_line(l.to_string()))
        .collect::<Vec<Game>>();
    let result = games.iter().fold(
        0,
        |acc, g| if g.possible(&limits) { acc + g.nr } else { acc },
    );
    println!("Part 1 example {}", result);

    let inputs = fs::read_to_string("input.txt").unwrap();
    let games = inputs
        .lines()
        .into_iter()
        .map(|l| parse_line(l.to_string()))
        .collect::<Vec<Game>>();
    let result = games.iter().fold(
        0,
        |acc, g| if g.possible(&limits) { acc + g.nr } else { acc },
    );
    println!("Part 1 {}", result);

    let games = contents
        .lines()
        .into_iter()
        .map(|l| parse_line(l.to_string()))
        .collect::<Vec<Game>>();

    let res2 = games.iter().map(|g| g.power()).fold(0, |acc, p| p + acc);
    println!("Result part 2 example {}", res2);

    let games = inputs
        .lines()
        .into_iter()
        .map(|l| parse_line(l.to_string()))
        .collect::<Vec<Game>>();
    let res2i = games.iter().map(|g| g.power()).fold(0, |acc, p| p + acc);
    println!("Result part 2 {}", res2i);
}

#[derive(PartialEq, Debug)]
pub struct Game {
    nr: i64,
    pub draws: Vec<Draw>,
}
impl Game {
    pub(crate) fn possible(&self, limits: &Draw) -> bool {
        self.draws
            .iter()
            .fold(true, |acc, d| acc && d.possible(limits))
    }
    pub fn power(&self) -> i64 {
        let max_red = self
            .draws
            .iter()
            .map(|d| d.red)
            .collect::<Vec<i64>>()
            .iter()
            .max()
            .unwrap()
            .clone();
        let max_green = self
            .draws
            .iter()
            .map(|d| d.green)
            .collect::<Vec<i64>>()
            .iter()
            .max()
            .unwrap()
            .clone();
        let max_blue = self
            .draws
            .iter()
            .map(|d| d.blue)
            .collect::<Vec<i64>>()
            .iter()
            .max()
            .unwrap()
            .clone();
        max_red * max_green * max_blue
    }
}
impl Game {
    pub fn of(nr: i64) -> Game {
        Game {
            nr: nr,
            draws: vec![],
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Draw {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
}

impl Draw {
    pub fn of(r: i64, g: i64, b: i64) -> Draw {
        Draw {
            red: r,
            green: g,
            blue: b,
        }
    }
    pub fn empty() -> Draw {
        Draw {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    pub fn possible(&self, limits: &Draw) -> bool {
        self.red <= limits.red && self.green <= limits.green && self.blue <= limits.blue
    }
}

pub fn parse_line(s: String) -> Game {
    //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let game: Vec<&str> = s.split(":").collect();
    let game_nr = game
        .get(0)
        .unwrap()
        .split(" ")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .to_string()
        .parse::<i64>()
        .unwrap();

    let draws = game
        .get(1)
        .unwrap()
        .trim()
        .split(";")
        .collect::<Vec<&str>>();
    let mut game = Game::of(game_nr);
    for draw in draws {
        let mut d = Draw::empty();
        let colors = draw.trim().split(",").collect::<Vec<&str>>();
        for color in colors {
            let col_nr = color.trim().split(" ").collect::<Vec<&str>>();
            let nr = col_nr.get(0).unwrap().trim().parse::<i64>().unwrap();
            match col_nr.get(1).unwrap().trim() {
                "red" => d.red = nr,
                "green" => d.green = nr,
                "blue" => d.blue = nr,
                _ => (),
            }
        }
        game.draws.push(d);
    }
    game
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(
            parse_line(String::from("")),
            Game {
                nr: 0,
                draws: vec!(),
            }
        )
    }
    #[test]
    fn test_line() {
        let test = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(
            parse_line(test),
            Game {
                nr: 1,
                draws: vec!(Draw::of(4, 0, 3), Draw::of(1, 2, 6), Draw::of(0, 2, 0))
            }
        )
    }
}
