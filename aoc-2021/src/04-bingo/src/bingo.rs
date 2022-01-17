use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    board: HashMap<i32, Vec<u32>>,
    mask: HashMap<i32, Vec<bool>>,
}

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
}

pub fn to_boards(input: &[&str]) -> HashMap<usize, Board> {
    input.iter().enumerate().fold(HashMap::new(), | mut acc, (i, b)| {
        acc.insert(i, Board::from_str(b));
        acc
    })
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
}
