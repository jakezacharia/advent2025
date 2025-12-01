use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("should be able to read");

    let mut counter: i32 = 0;
    let mut current: i32 = 50;

    for line in contents.lines() {
        let (dir, num_str) = line.split_at(1);
        let mut steps: i32 = num_str.parse().expect("invalid num");

        steps = steps % 100;
        let delta = match dir {
            "R" => steps,
            "L" => -steps,
            _ => panic!("invalid direction"),
        };

        current = (current + delta).rem_euclid(100);

        if current == 0 {
            counter += 1
        }
    }
    println!("{counter}");
}
