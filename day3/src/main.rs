use std::collections::HashSet;
use std::fs;

#[derive(Clone, Copy, PartialOrd, PartialEq, Debug, Eq, Hash)]
pub struct Number {
    row: usize,
    col: usize,
    value: usize,
}
impl Number {
    pub fn empty(row: usize) -> Number {
        Number {
            row: row,
            col: 0,
            value: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.value.to_string().len()
    }
    pub fn bound_rect_on_board(&self, len: usize) -> Vec<(usize, usize)> {
        let mut rec = vec![];
        let start = if self.col > 0 { self.col - 1 } else { 0 };
        let mut finish = self.col + self.len() + 1;
        if finish == len {
            finish = finish - 1
        };
        for i in start..finish {
            if self.row > 0 && i < len {
                rec.push((i, self.row - 1))
            };
            if self.row < len - 1 && i < len {
                rec.push((i, self.row + 1))
            };
        }
        if self.col > 0 {
            rec.push((self.col - 1, self.row))
        };
        if self.col + self.len() < len - 1 {
            rec.push((self.col + self.len(), self.row))
        };
        rec
    }
    pub fn is_part(&self, b: &Board) -> bool {
        let bound_rec = self.bound_rect_on_board(b.len());
        println!("{:?}", bound_rec);
        bound_rec
            .iter()
            .any(|pair| b[pair.1][pair.0] != '.' && !b[pair.1][pair.0].is_digit(10))
    }
}
pub type Board = Vec<Vec<char>>;

fn main() {
    let contents: String = fs::read_to_string("input").unwrap();
    let board: Board = read_board(contents);
    let nb: Vec<Number> = read_numbers(&board);
    let mut sum = 0;
    for n in nb.clone() {
        if n.is_part(&board) {
            sum = sum + n.value;
            println!("{:?}", n);
        }
    }
    println!("Part 1 example {}", sum);
    let mut stars = vec![];
    for row in 0..board.len() {
        for col in 0..board.len() {
            if board[row][col] == '*' {
                stars.push((col, row));
            }
        }
    }
    println!("{:?}", stars);
    let mut the_end = 0;
    for star in stars {
        let aura = around_star(star.0, star.1);
        println!("Aura for star {:?} {:?}", star, aura);
        let mut adjcs = HashSet::new();
        for n in nb.clone() {
            for p in aura.clone() {
                if (n.row == p.1) && (n.col <= p.0) && ((n.col + n.len()) > p.0) {
                    adjcs.insert(n.clone());
                }
            }
        }
        if adjcs.len() == 2 {
            let mut prod = 1;
            for x in adjcs {
                prod = prod * x.value;
            }
            the_end = the_end + prod;
        }
    }
    println!("{}", the_end)
}
pub fn around_star(col: usize, row: usize) -> Vec<(usize, usize)> {
    let mut pos = vec![];
    pos.push((col - 1, row - 1));
    pos.push((col, row - 1));
    pos.push((col + 1, row - 1));
    pos.push((col - 1, row));
    pos.push((col + 1, row));
    pos.push((col - 1, row + 1));
    pos.push((col, row + 1));
    pos.push((col + 1, row + 1));
    pos
}
pub fn read_numbers(b: &Board) -> Vec<Number> {
    let mut numbers = vec![];
    let mut row_index = 0;
    for row in b {
        let mut ns = parse_row(row.clone(), row_index);
        numbers.append(&mut ns);
        row_index = row_index + 1;
    }
    numbers
}
pub fn read_board(contents: String) -> Board {
    let len = contents.lines().next().unwrap().len();
    let mut board: Vec<Vec<char>> = vec![vec![' '; len]; len];

    let mut row = 0;
    for line in contents.lines() {
        let mut col = 0;
        for c in line.chars() {
            board[row][col] = c;
            col = col + 1;
        }
        row = row + 1;
    }

    board
}
pub fn parse_row(row: Vec<char>, row_index: usize) -> Vec<Number> {
    let mut col = 0;
    let mut start = 0;
    let mut in_num = false;
    let mut numbers: Vec<Number> = vec![];
    let mut num_now = Number::empty(row_index);
    while col < row.len() {
        if row[col].is_digit(10) {
            if in_num {
                num_now.value = (10 * num_now.value) + row[col].to_digit(10).unwrap() as usize;
                in_num = true;
            } else {
                start = col;
                num_now.col = start;
                num_now.value = row[col].to_digit(10).unwrap() as usize;
                in_num = true;
            }
        } else {
            if in_num {
                in_num = false;
                numbers.push(num_now.clone());
                num_now = Number::empty(row_index);
            } else {
                // do nothing
            }
        }
        col = col + 1;
    }
    if in_num {
        numbers.push(num_now);
    }
    numbers
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let line = String::from("467..114..");
        let result = parse_row(line.chars().collect::<Vec<char>>(), 0);
        assert_eq!(
            result,
            vec![
                Number {
                    row: 0,
                    col: 0,
                    value: 467
                },
                Number {
                    row: 0,
                    col: 5,
                    value: 114
                }
            ]
        );
    }
    #[test]
    fn test_parse1() {
        let line = String::from("..........");
        let result = parse_row(line.chars().collect::<Vec<char>>(), 0);
        assert_eq!(result, vec![]);
    }
    #[test]
    fn test_parse2() {
        let line = String::from(".467.1.114");
        let result = parse_row(line.chars().collect::<Vec<char>>(), 0);
        assert_eq!(
            result,
            vec![
                Number {
                    row: 0,
                    col: 1,
                    value: 467
                },
                Number {
                    row: 0,
                    col: 5,
                    value: 1
                },
                Number {
                    row: 0,
                    col: 7,
                    value: 114
                }
            ]
        );
    }
    #[test]
    fn test_parse3() {
        let line = String::from("4673311412");
        let result = parse_row(line.chars().collect::<Vec<char>>(), 0);
        assert_eq!(
            result,
            vec![Number {
                row: 0,
                col: 0,
                value: 4673311412
            }]
        );
    }
    #[test]
    fn test_rec() {
        let n = Number {
            col: 1,
            row: 2,
            value: 12,
        };
        let mut result = n.bound_rect_on_board(10);
        assert_eq!(
            result.sort(),
            vec![
                (0, 1),
                (1, 1),
                (2, 1),
                (3, 1),
                (0, 3),
                (1, 3),
                (2, 3),
                (3, 3),
                (0, 2),
                (3, 2)
            ]
            .sort()
        );
    }
    #[test]
    fn test_rec1() {
        let n = Number {
            col: 0,
            row: 0,
            value: 12,
        };
        let mut result = n.bound_rect_on_board(10);
        assert_eq!(result.sort(), vec![(2, 0), (0, 1), (1, 1), (2, 1)].sort());
    }
    #[test]
    fn test_rec2() {
        let n = Number {
            col: 0,
            row: 1,
            value: 12,
        };
        let mut result = n.bound_rect_on_board(10);
        assert_eq!(
            result.sort(),
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (0, 2), (1, 2), (2, 2)].sort()
        );
    }
    #[test]
    fn test_rec3() {
        let n = Number {
            col: 8,
            row: 1,
            value: 12,
        };
        let mut result = n.bound_rect_on_board(10);
        assert_eq!(
            result.sort(),
            vec![(7, 0), (8, 0), (9, 0), (7, 1), (7, 2), (8, 2), (9, 2)].sort()
        );
    }
}
