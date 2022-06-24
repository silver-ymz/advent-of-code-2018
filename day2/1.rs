use std::{fs::File, io::Read, collections::HashMap};

fn main() {
    let mut file = File::open("inputs/input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let strs = content.lines().collect::<Vec<&str>>();
    let (mut cnt2, mut cnt3) = (0, 0);
    let mut m = HashMap::new();
    for s in strs {
        for ch in s.chars() {
            m.entry(ch).and_modify(|v| *v += 1).or_insert(1);
        }
        cnt2 += m.iter().filter(|(_, v)| v == &&2).next().is_some() as i32;
        cnt3 += m.iter().filter(|(_, v)| v == &&3).next().is_some() as i32;
        m.clear();
    }
    println!("{}", cnt2 * cnt3);
}
