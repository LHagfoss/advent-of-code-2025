use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let trimmed = input.trim();

    let mut total_sum: u64 = 0;

    for range_raw in trimmed.split(',') {
        let parts: Vec<&str> = range_raw.split('-').collect();

        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();

        for num in start..=end {
            if is_double_pattern(num) {
                total_sum += num;
            }
        }
    }

    println!("Sum av ugyldige ID-er: {}", total_sum);
}

fn is_double_pattern(n: u64) -> bool {
    if n == 0 {
        return false;
    }

    let num_digits = n.ilog10() + 1;

    if num_digits % 2 != 0 {
        return false;
    }

    let half_digits = num_digits / 2;

    let divisor = 10_u64.pow(half_digits);

    let first_half = n / divisor;
    let second_half = n % divisor;

    first_half == second_half
}
