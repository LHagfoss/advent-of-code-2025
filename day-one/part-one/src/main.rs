use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut position: i32 = 50;
    let mut password_count = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let direction = &line[0..1];
        let amount: i32 = line[1..].parse().unwrap();

        if direction == "R" {
            position = (position + amount) % 100;
        } else if direction == "L" {
            position = (position - amount).rem_euclid(100);
        }

        if position == 0 {
            password_count += 1;
        }
    }

    println!("Password: {}", password_count);
}
