use std::{fs::File, io::Read, iter::zip};

fn main() {
    let mut file = File::open("inputs/input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let strs = content.lines().collect::<Vec<&str>>();
    for (idx, s1) in strs.iter().enumerate() {
        for s2 in strs[..idx].iter() {
            if zip(s1.chars(), s2.chars()).filter(|(i, j)| i != j).count() == 1 {
                println!(
                    "{}",
                    zip(s1.chars(), s2.chars())
                        .filter(|(i, j)| i == j)
                        .map(|(i, _)| i)
                        .collect::<String>()
                );
                return;
            }
        }
    }
}
