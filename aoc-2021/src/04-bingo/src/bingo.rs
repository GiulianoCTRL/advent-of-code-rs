#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BingoBoard {
    board: [[u32; 5]; 5],
    mask: [[bool; 5]; 5],
}

impl BingoBoard {
    fn new(rows: &[Vec<u32>]) -> BingoBoard {
        let mut board = [[0u32; 5]; 5];
        for i in 0..5 {
            board[i] = rows[i].to_vec().try_into().unwrap();
        }
        BingoBoard {
            board,
            mask: [[false; 5]; 5],
        }
    }

    fn check_number(mut self, num: &u32) {
        for (i, row) in self.board.iter().enumerate() {
            for (j, n) in row.iter().enumerate() {
                if n == num {
                    self.mask[i][j] = true;
                    println!("{:?}", self.mask);
                }
            }
        }
    }

    fn board_won(self) -> bool {
        for row in self.mask.iter() {
            if row.iter().all(|i| *i) {
                println!("{:?}", self.mask);
                return true;
            }
        }
        false
    }

    fn calc_result(self, num: u32) -> u32 {
        //! Calculate result by multiplying the sum of uncalled number bingo numbers
        //! in the winning board with the last called number.
        let sum: u32 = self
            .board
            .iter()
            .enumerate()
            .map(|(ir, r)| {
                r.iter()
                    .enumerate()
                    .filter(|i| !self.mask[ir][i.0])
                    .map(|i| *i.1)
                    .sum::<u32>()
            })
            .sum();
        sum * num
    }
}

pub fn get_bingo_boards(input: &[String]) -> Vec<BingoBoard> {
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut board: Vec<Vec<u32>> = Vec::new();
    for (i, line) in input.iter().filter(|l| !l.is_empty()).enumerate() {
        board.push(
            line.trim()
                .split(' ')
                .collect::<Vec<&str>>()
                .iter()
                .filter_map(|s| s.trim().parse::<u32>().ok())
                .collect(),
        );
        if i % 5 == 4 {
            boards.push(BingoBoard::new(&board));
            board = Vec::new();
        }
    }
    boards
}

pub fn get_bingo_nums(nums: &str) -> Vec<u32> {
    //! Read a string reference of comma separated numbers and return them
    //! as vector of u32s.
    nums.trim()
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[allow(clippy::if_same_then_else)]
pub fn get_result(boards: &mut [BingoBoard], bingo_input: &[u32], part1: bool) -> Option<u32> {
    //! Create a bool mask for each board and flip the previous false value to true
    //! if the number is called. If a row evaluates to true, the board will be selected
    //! as winning board and the result calculation will be returned.
    let mut count = 0;
    for n in bingo_input.iter() {
        for board in boards.iter() {
            board.check_number(n);
            if board.board_won() {
                println!("Here we are");
                count += 1;
                if part1 {
                    return Some(board.calc_result(*n));
                } else if count == 100 {
                    return Some(board.calc_result(*n));
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bingo_nums() {
        // Test that input of number str is correctly proccessed.
        let nums: &str = " 44, 66, 78, 21 ";
        let expected = vec![44u32, 66, 78, 21];

        assert_eq!(get_bingo_nums(nums), expected);
    }

    #[test]
    fn test_get_bingo_boards() {
        let input: Vec<String> = vec![
            "12 75 58 21 87".to_string(),
            "55 80 14 63 17".to_string(),
            "37 35 76 92 56".to_string(),
            "72 68 51 19 38".to_string(),
            "91 60 34 30 88".to_string(),
            "".to_string(),
            "0 66  5 51  8".to_string(),
            "45 57 31  3 62".to_string(),
            "7 60 40 29 90".to_string(),
            "80 19 47 86 81".to_string(),
            "95 69 68 53 93".to_string(),
            "".to_string(),
        ];
        let expected: Vec<BingoBoard> = vec![
            BingoBoard {
                board: [
                    [12u32, 75, 58, 21, 87],
                    [55u32, 80, 14, 63, 17],
                    [37u32, 35, 76, 92, 56],
                    [72u32, 68, 51, 19, 38],
                    [91u32, 60, 34, 30, 88],
                ],
                mask: [
                    [false, false, false, false, false],
                    [false, false, false, false, false],
                    [false, false, false, false, false],
                    [false, false, false, false, false],
                    [false, false, false, false, false],
                ],
            },
            BingoBoard {
                board: [
                    [0u32, 66, 5, 51, 8],
                    [45u32, 57, 31, 3, 62],
                    [7u32, 60, 40, 29, 90],
                    [80u32, 19, 47, 86, 81],
                    [95u32, 69, 68, 53, 93],
                ],
                mask: [
                    [false, false, false, false, false],
                    [false, false, false, false, false],
                    [false, false, false, false, false],
                    [false, false, false, false, false],
                    [false, false, false, false, false],
                ],
            },
        ];
        assert_eq!(get_bingo_boards(&input), expected);
    }
}
