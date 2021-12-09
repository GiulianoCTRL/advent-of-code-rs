mod diagnose;

fn main() {
    let input: Vec<String> = helpers::input().expect("Couldn't open file.");
    let bit_size = input[0].chars().count();

    let power = diagnose::calc_power_cons(&input, bit_size);
    println!("Power consumption {}", power);

    let life_supp = diagnose::calc_life_support(&input, bit_size);
    println!("Life support rating {}", life_supp);
}
