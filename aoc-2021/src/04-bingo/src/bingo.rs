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

pub fn get_bingo_boards(input: &[String]) -> Vec<Vec<Vec<u32>>> {
    //! Iterate over input and create a 3 dimensional vector from it.
    //! Contents are [board][row][number]
    let mut boards: Vec<Vec<Vec<u32>>> = Vec::new();
    let mut board = 0usize;
    let mut row = 0usize;
    for line in input.iter() {
        if line.is_empty() {
            continue;
        }
        if row == 5 {
            row = 0;
            board += 1;
        }
        let nums: Vec<u32> = line
            .trim()
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .collect();
        if row == 0 {
            boards.push(
                (0..5)
                    .map(|_| Vec::with_capacity(5))
                    .collect::<Vec<Vec<u32>>>(),
            )
        }
        boards[board][row] = nums;
        row += 1;
    }
    boards
}

pub fn get_result(boards: &[Vec<Vec<u32>>], bingo_input: &[u32]) -> Option<u32> {
    //! Create a bool mask for each board and flip the previous false value to true
    //! if the number is called. If a row evaluates to true, the board will be selected
    //! as winning board and the result calculation will be returned.
    let mut mask: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 5]; 5]; boards.len()];

    for n in bingo_input.iter() {
        for (ib, board) in boards.iter().enumerate() {
            for (ir, row) in board.iter().enumerate() {
                for (ii, i) in row.iter().enumerate() {
                    if i == n {
                        mask[ib][ir][ii] = true;
                    }
                }
                if mask[ib][ir].iter().all(|i| *i) {
                    return Some(calc_result(*n, &boards[ib], &mask[ib]));
                }
            }
        }
    }
    None
}

fn calc_result(num: u32, board: &[Vec<u32>], mask: &[Vec<bool>]) -> u32 {
    //! Calculate result by multiplying the sum of uncalled number bingo numbers
    //! in the winning board with the last called number.
    let sum: u32 = board
        .iter()
        .enumerate()
        .map(|(ir, r)| {
            r.iter()
                .enumerate()
                .filter(|i| !mask[ir][i.0])
                .map(|i| *i.1)
                .sum::<u32>()
        })
        .sum();
    sum * num
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
        let expected: Vec<Vec<Vec<u32>>> = vec![
            vec![
                vec![12u32, 75, 58, 21, 87],
                vec![55u32, 80, 14, 63, 17],
                vec![37u32, 35, 76, 92, 56],
                vec![72u32, 68, 51, 19, 38],
                vec![91u32, 60, 34, 30, 88],
            ],
            vec![
                vec![0u32, 66, 5, 51, 8],
                vec![45u32, 57, 31, 3, 62],
                vec![7u32, 60, 40, 29, 90],
                vec![80u32, 19, 47, 86, 81],
                vec![95u32, 69, 68, 53, 93],
            ],
        ];
        assert_eq!(get_bingo_boards(&input), expected);
    }

    #[test]
    fn test_calc_result() {
        let num = 88;
        let board: Vec<Vec<u32>> = vec![
            vec![12u32, 75, 58, 21, 87],
            vec![55u32, 80, 14, 63, 17],
            vec![37u32, 35, 76, 92, 56],
            vec![72u32, 68, 51, 19, 38],
            vec![91u32, 60, 34, 30, 88],
        ];
        let mask: Vec<Vec<bool>> = vec![
            vec![false, true, false, true, true],
            vec![true, false, true, true, true],
            vec![true, true, true, true, false],
            vec![false, true, true, true, true],
            vec![false, false, false, false, false],
        ];
        let expected = (12u32 + 58 + 80 + 56 + 72 + 91 + 60 + 34 + 30 + 88) * 88;
        assert_eq!(calc_result(num, &board, &mask), expected);
    }

    #[test]
    fn test_get_result() {
        let boards: Vec<Vec<Vec<u32>>> = vec![
            vec![
                vec![12u32, 75, 58, 21, 87],
                vec![55u32, 80, 14, 63, 17],
                vec![37u32, 35, 76, 92, 56],
                vec![72u32, 68, 51, 19, 38],
                vec![91u32, 60, 34, 30, 88],
            ],
            vec![
                vec![0u32, 66, 5, 51, 8],
                vec![45u32, 57, 31, 3, 62],
                vec![7u32, 60, 40, 29, 90],
                vec![80u32, 19, 47, 86, 81],
                vec![95u32, 69, 68, 53, 93],
            ],
        ];
        let bingo_nums = vec![45u32, 57, 31, 3, 62];
        let expected = Some(
            (66u32
                + 5
                + 51
                + 8
                + 7
                + 60
                + 40
                + 29
                + 90
                + 80
                + 19
                + 47
                + 86
                + 81
                + 95
                + 69
                + 68
                + 53
                + 93)
                * 62,
        );
        assert_eq!(get_result(&boards, &bingo_nums), expected);
    }
}
