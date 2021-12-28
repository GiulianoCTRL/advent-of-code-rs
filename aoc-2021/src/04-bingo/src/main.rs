/* Each board 5x5
Input
    - bingo input: [X]
        - first line is ,-separated bingo input≈
    - bingo boards: [X]
        - boards separated by empty line
        - numbers in board separated by single ' '
Processing of input: [X]
    - bingo input into vector
    - boards into 3D vector -> board[line[number]]
Selecting winning board
    - loop through each number and check if number has been called
    - lookup table with bool = number has been marked (init with false)
    - if value of all bools in line is true, break loop and select board
Result
    - pop number from bingo input number (to later return it when loop breaks)
    - sum of all unmarked numbers on board multiplied by last called number (false in lookup)
        - each value that is false shall be summed (.reduce with .filter)

*/
mod bingo;

#[allow(unused_variables)]
fn main() {
    let input = helpers::input();
    let bingo_nums: Vec<u32> = bingo::get_bingo_nums(&input[0]);
    let mut bingo_boards: Vec<bingo::BingoBoard> = bingo::get_bingo_boards(&input[2..]);

    let result_i = bingo::get_result(&mut bingo_boards, &bingo_nums, true);
    let result_ii = bingo::get_result(&mut bingo_boards, &bingo_nums, false);

    if let Some(i) = result_i {
        println!("{}", i);
    } else {
        println!("No winner found in part i.");
    }

    if let Some(i) = result_ii {
        println!("{}", i);
    } else {
        println!("No winner found in part ii.");
    }
}
