use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    board: HashMap<i32, Vec<u32>>,
    mask: HashMap<i32, Vec<bool>>,
}

type Boards = HashMap<usize, Board>;
type BingoData = (Vec<u32>, Boards);

impl Board {
    fn from_str(s: &str) -> Board {
        let (board, mask): (HashMap<i32, Vec<u32>>, HashMap<i32, Vec<bool>>) =
            s.split('\n').enumerate().fold(
                (HashMap::new(), HashMap::new()),
                |(mut board, mut mask), (ir, r)| {
                    r.trim()
                        .split(' ')
                        .filter(|s| !s.is_empty())
                        .enumerate()
                        .map(|(ic, i)| (ic, i.trim().parse::<u32>().unwrap()))
                        .for_each(|(ic, i)| {
                            let (col, row) = (-((ic as i32) + 1), (ir + 1) as i32);
                            board.entry(col).or_default().push(i);
                            board.entry(row).or_default().push(i);
                            mask.entry(col).or_default().push(false);
                            mask.entry(row).or_default().push(false);
                        });
                    (board, mask)
                },
            );

        Board { board, mask }
    }

    fn mark_num(&mut self, num: &u32) {
        self.board.iter().for_each(|(k, v)| {
            if v.contains(num) {
                let index = v.iter().position(|&i| &i == num).unwrap();
                self.mask.get_mut(k).unwrap()[index] = true;
            }
        })
    }

    fn has_won(&mut self) -> bool {
        self.mask
            .iter()
            .map(|(_, v)| v.iter().all(|b| *b))
            .collect::<Vec<bool>>()
            .iter()
            .any(|b| *b)
    }

    fn calc_result(&mut self, last_num: &u32) -> u32 {
        self.board
            .iter()
            .filter(|(k, _v)| k > &&0)
            .fold(0u32, |acc, (k, v)| {
                acc + v
                    .iter()
                    .enumerate()
                    .filter(|(i, _n)| !self.mask.get(k).unwrap()[*i])
                    .map(|(_i, n)| n)
                    .sum::<u32>()
            })
            * last_num
    }
}

fn to_nums(s: &str) -> Vec<u32> {
    s.split(',')
        .map(|n| n.trim().parse::<u32>().unwrap())
        .collect()
}

fn to_boards(input: &[&str]) -> Boards {
    input
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, b)| {
            acc.insert(i, Board::from_str(b));
            acc
        })
}

fn get_bingo_data(s: &str) -> BingoData {
    s.split_once("\n\n")
        .map(|t| {
            (
                to_nums(t.0),
                to_boards(&t.1.split("\n\n").collect::<Vec<&str>>()),
            )
        })
        .unwrap()
}

fn get_winner_data(s: &str) -> (Vec<usize>, Vec<u32>) {
    let mut data: BingoData = get_bingo_data(s);
    let mut win_order: Vec<usize> = Vec::new();
    let mut win_result: Vec<u32> = Vec::new();
    for num in data.0.iter_mut() {
        let winners = win_order.to_vec();
        for (k, v) in data.1.iter_mut().filter(|(k, _v)| !winners.contains(k)) {
            v.mark_num(num);
            if v.has_won() {
                win_result.push(v.calc_result(num));
                win_order.push(*k);
            }
        }
    }
    (win_order, win_result)
}

pub fn announce_winners(s: &str) {
    let winners = get_winner_data(s);
    let last = winners.0.len() - 1;
    println!(
        "The first winner is board no.: {} with a score of {}.",
        winners.0[0], winners.1[0]
    );
    println!(
        "The last to win is board no.: {} with a score of {}",
        winners.0[last], winners.1[last]
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Board {
        fn test_setup() -> Board {
            Board {
                board: HashMap::from([
                    (1i32, vec![12u32, 75, 58, 21, 87]),
                    (2i32, vec![55u32, 80, 14, 63, 17]),
                    (3i32, vec![37u32, 35, 76, 92, 56]),
                    (4i32, vec![72u32, 68, 51, 19, 38]),
                    (5i32, vec![91u32, 60, 34, 30, 88]),
                    (-1i32, vec![12u32, 55, 37, 72, 91]),
                    (-2i32, vec![75u32, 80, 35, 68, 60]),
                    (-3i32, vec![58u32, 14, 76, 51, 34]),
                    (-4i32, vec![21u32, 63, 92, 19, 30]),
                    (-5i32, vec![87u32, 17, 56, 38, 88]),
                ]),
                mask: HashMap::from([
                    (1i32, vec![false; 5]),
                    (2i32, vec![false; 5]),
                    (3i32, vec![false; 5]),
                    (4i32, vec![false; 5]),
                    (5i32, vec![false; 5]),
                    (-1i32, vec![false; 5]),
                    (-2i32, vec![false; 5]),
                    (-3i32, vec![false; 5]),
                    (-4i32, vec![false; 5]),
                    (-5i32, vec![false; 5]),
                ]),
            }
        }
    }

    #[test]
    fn test_new_board() {
        let input = "12 75 58 21 87
        55 80 14 63 17
        37 35 76 92 56
        72 68 51 19 38
        91 60 34 30 88";
        let expected = Board::test_setup();
        assert_eq!(Board::from_str(&input), expected);
    }

    #[test]
    fn test_number_flipped() {
        let mut expected = Board::test_setup();
        expected.mask.get_mut(&-4).unwrap()[2] = true;
        expected.mask.get_mut(&3).unwrap()[3] = true;
        let mut result = Board::test_setup();
        result.mark_num(&92);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_has_won() {
        let mut input = Board::test_setup();
        [55u32, 80, 14, 63, 17]
            .iter()
            .for_each(|i| input.mark_num(i));
        assert_eq!(input.has_won(), true);
    }

    #[test]
    fn test_calc_result() {
        let mut input = Board::test_setup();
        [55u32, 80, 14, 63, 17]
            .iter()
            .for_each(|i| input.mark_num(i));
        assert_eq!(input.calc_result(&17), 18700)
    }
}
