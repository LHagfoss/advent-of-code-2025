use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    if grid.is_empty() {
        println!("0");
        return;
    }

    let height = grid.len();
    let width = grid[0].len();
    let mut accessible_count = 0;

    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != '@' {
                continue;
            }

            let mut neighbor_count = 0;

            for (dy, dx) in offsets {
                let ny = y as isize + dy;
                let nx = x as isize + dx;

                if ny >= 0 && ny < height as isize && nx >= 0 && nx < width as isize {
                    if grid[ny as usize][nx as usize] == '@' {
                        neighbor_count += 1;
                    }
                }
            }

            if neighbor_count < 4 {
                accessible_count += 1;
            }
        }
    }

    println!("Accessible paper rolls: {}", accessible_count);
}
