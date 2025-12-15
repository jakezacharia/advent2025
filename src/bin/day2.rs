use std::fs;

fn main() {
    let input = fs::read_to_string("src/inputs/day2.txt").expect("no input");
    let input = input.trim();

    let contents: Vec<&str> = input.split(",").collect();

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for item in contents {
        let item = item.trim();
        // split this into two numerical vals
        let halves: Vec<&str> = item.split("-").collect();
        if halves.len() != 2 {
            panic!("bad range format, check input.txt");
        }
        
        // convert str objects into u32 and assign properly
        let start: u64 = halves[0].trim().parse().expect("bad start num");
        let end: u64 = halves[1].trim().parse().expect("bad end num");

        ranges.push((start,end));

        }
        // println!("{:?}", ranges)
    }

