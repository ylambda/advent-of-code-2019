use std::collections::HashMap;

fn main() {
    let mut total = 0;
    for num in 264793..=803935 {
        if validate_number(num) {
            total += 1;
        }
    }

    println!("Answer: {}", total);
}

fn validate_number(num: isize) -> bool {
    let digits = parse_digits(num);
    let increase = validate_increase(&digits);
    let adjacent = validate_adjacent(&digits);

    increase && adjacent
}

fn parse_digits(num: isize) -> Vec<u32> {
    num.to_string().chars().map(|x| x.to_digit(10).unwrap()).collect()
}

fn validate_adjacent(digits: &Vec<u32>) -> bool {
    let mut store: HashMap<u32, u8> = HashMap::new();

    for &digit in digits.iter() {
        let count = store.entry(digit).or_insert(0);
        *count += 1;
    }

    store.values().find(|&&x| x == 2).is_some()
}

fn validate_increase(digits: &Vec<u32>) -> bool {
    let mut prev = 0;
    let mut increase_order = true;

    for &digit in digits.iter() {
        if prev > digit {
            increase_order = false;
            break;
        }

        prev = digit;
    }

    increase_order
}
