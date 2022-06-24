use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("inputs/input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!(
        "{}",
        content
            .lines()
            .map(|s| s.trim().parse::<i32>().unwrap())
            .sum::<i32>()
    );
}
