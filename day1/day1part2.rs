use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("should be able to read");

    let mut counter: i32 = 0;
    let mut current: i32 = 50;

    for line in contents.lines() {
        let (dir, num_str) = line.split_at(1);
        let steps: i32 = num_str.parse().expect("invalid num");

        let delta = match dir {
            "R" => steps,
            "L" => -steps,
            _ => panic!("invalid direction"),
        };

        let direction: i32 = if delta >= 0 { 1 } else { -1 };

        for _ in 0..delta.abs() {
        current = (current + direction).rem_euclid(100);
            if current == 0 {
                counter += 1;
                }
            }
    }
    println!("{counter}");
}
