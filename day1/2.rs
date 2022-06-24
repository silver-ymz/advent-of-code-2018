use std::{collections::HashSet, fs::File, io::Read};

fn main() {
    let mut file = File::open("inputs/input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let nums: Vec<i32> = content
        .lines()
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();
    let mut total = 0;
    let mut num_set: HashSet<i32> = HashSet::new();
    for i in nums.iter().cycle() {
        total += i;
        if !num_set.insert(total) {
            println!("{}", total);
            return;
        }
    }
}
