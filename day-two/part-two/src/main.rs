use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let trimmed = input.trim();

    let mut total_sum: u64 = 0;

    for range_raw in trimmed.split(',') {
        if range_raw.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = range_raw.split('-').collect();
        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();

        for num in start..=end {
            if is_invalid(num) {
                total_sum += num;
            }
        }
    }

    println!("Sum av ugyldige ID-er (Del 2): {}", total_sum);
}

fn is_invalid(num: u64) -> bool {
    let s = num.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();

    for k in 1..=(len / 2) {
        if len % k == 0 {
            let mut pattern_match = true;
            for i in k..len {
                if bytes[i] != bytes[i % k] {
                    pattern_match = false;
                    break;
                }
            }

            if pattern_match {
                return true;
            }
        }
    }
    false
}
