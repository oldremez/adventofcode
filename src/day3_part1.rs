use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

pub fn run() {
    let mut current_sum:u64 = 0;

    if let Ok(lines) = read_lines("inputs/day3.txt") {
        for line in lines {
            if let Ok(cal) = line {
                let set1: HashSet<char> = HashSet::from_iter(cal.chars().take(cal.len()/2));
                let set2 = HashSet::from_iter(cal.chars().skip(cal.len()/2));
                let res = set1.intersection(&set2).into_iter().next();
                let c = res.unwrap();
                if c.is_lowercase(){
                    current_sum += (*c as u64) - ('a' as u64) + 1;
                }
                if c.is_uppercase(){
                    current_sum += (*c as u64) - ('A' as u64) + 27;
                }
            }
        }
    }
    println!("{}", current_sum);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}