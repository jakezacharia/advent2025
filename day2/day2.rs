use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("no input");
    let input = input.trim();

    let contents: Vec<&str> = input.split(",").collect();

    let ranges: Vec<(u32, u32)> = Vec::new();

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
        println!("{start},{end}");

        }
    }

