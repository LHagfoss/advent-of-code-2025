use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut total_joltage = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        'search: for tens in (1..=9).rev() {
            for ones in (1..=9).rev() {
                let tens_char = char::from_digit(tens, 10).unwrap();
                let ones_char = char::from_digit(ones, 10).unwrap();

                let first_pos = line.find(tens_char);
                let last_pos = line.rfind(ones_char);

                match (first_pos, last_pos) {
                    (Some(f), Some(l)) => {
                        if f < l {
                            total_joltage += tens * 10 + ones;
                            break 'search;
                        }
                    }
                    _ => continue,
                }
            }
        }
    }

    println!("Total output joltage: {}", total_joltage);
}
