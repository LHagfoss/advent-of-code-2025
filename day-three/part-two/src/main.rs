use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut total_output_joltage: u64 = 0;

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.len() < 12 {
            continue;
        }

        let bytes = trimmed.as_bytes();
        let mut current_position = 0;
        let mut result_string = String::new();

        for i in 0..12 {
            let remaining_needed = 12 - i;
            let search_limit = bytes.len() - remaining_needed;

            let mut best_digit = b'0' - 1;
            let mut best_index = 0;

            for j in current_position..=search_limit {
                if bytes[j] > best_digit {
                    best_digit = bytes[j];
                    best_index = j;

                    if best_digit == b'9' {
                        break;
                    }
                }
            }

            result_string.push(best_digit as char);

            current_position = best_index + 1;
        }

        let value: u64 = result_string.parse().unwrap();
        total_output_joltage += value;
    }

    println!("Total output joltage (Del 2): {}", total_output_joltage);
}
