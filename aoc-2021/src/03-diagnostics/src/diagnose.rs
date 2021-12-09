// I learned a lot about iterators from ljgago's Github.
// Especially the .fold method in mask_lines() was implemented so
// efficiantly thanks to him (function had thrice the length previously).

fn mask_chars(input: &[String]) -> Vec<Vec<i32>> {
    //! Replace each 1 in each line with 1 and each 0 with -1 to prepare
    //! for summing each bits value later on.
    input
        .iter()
        .map(|b_str| {
            b_str
                .chars()
                .map(|c| if c == '1' { 1 } else { -1 })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn mask_lines(char_mask: &[Vec<i32>], bit_size: usize) -> Vec<i32> {
    //! Calculate the value each line string by taking the sum of the char mask.
    char_mask.iter().fold(vec![0; bit_size], |accum, v| {
        accum
            .iter()
            .zip(v.iter())
            .map(|t| t.0 + t.1)
            .collect::<Vec<i32>>()
    })
}

fn comp_to_new_bit(line_mask: &[i32]) -> String {
    //! Iterate through sum of computed values and replace
    //! the value with 1 if it's above 0 and 0 if below.
    line_mask
        .iter()
        .map(|i| if i > &0 { "1" } else { "0" })
        .collect::<String>()
}

fn bit_to_dec(bit: &str, bit_size: usize, reverse: bool) -> usize {
    bit.chars()
        .enumerate()
        .map(|t| {
            if t.1.eq(if reverse { &'0' } else { &'1' }) {
                2usize.pow((bit_size - 1 - t.0).try_into().unwrap())
            } else {
                0usize
            }
        })
        .reduce(|accum, i| accum + i)
        .unwrap()
}

fn mask_from_vert(input: &[String], bit_size: usize, o2: bool) -> String {
    //! Create a char and string mask as in part I but iterate through them
    //! vertically. If o2 is true the most common number will be selected
    //! (1 if equal) and if it is false (co2) then the least common number is
    //! selected (0 if equal.) Return the newly create string of 1s and 0s.
    let mut input: Vec<String> = input.to_vec();
    let mut pos = 0;
    loop {
        if input.len() == 1 {
            break;
        }
        let char_mask = mask_chars(&input);
        let line_mask = mask_lines(&char_mask, bit_size);
        input = char_mask
            .iter()
            .filter(|n| match (line_mask[pos] >= 0, o2) {
                (true, true) | (false, false) => n[pos] > 0,
                (true, false) | (false, true) => n[pos] < 0,
            })
            .map(|n| {
                n.iter()
                    .map(|&d| if d == 1 { '1' } else { '0' })
                    .collect::<Vec<char>>()
            })
            .map(String::from_iter)
            .collect();
        pos += 1;
    }
    String::from(&input[0])
}

pub fn calc_power_cons(input: &[String], bit_size: usize) -> usize {
    //! Calculate gamma and multiply it with it's reverse
    let gamma = comp_to_new_bit(&mask_lines(&mask_chars(input), bit_size));
    bit_to_dec(&gamma, bit_size, false) * bit_to_dec(&gamma, bit_size, true)
}

pub fn calc_life_support(input: &[String], bit_size: usize) -> usize {
    //! Calc life support by multiplying o2 and co2
    let o2 = bit_to_dec(&mask_from_vert(input, bit_size, true), bit_size, false);
    let co2 = bit_to_dec(&mask_from_vert(input, bit_size, false), bit_size, false);
    o2 * co2
}
