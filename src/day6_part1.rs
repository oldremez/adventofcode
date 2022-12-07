use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    if let Ok(mut lines) = read_lines("inputs/day6.txt") {
        if let Ok(line) = lines.next().unwrap() {
            for i in 0..line.len() {
                let mut sss: String = String::new();
                line[i..i + 4].clone_into(&mut sss);
                let s:HashSet<u8> = HashSet::from_iter(sss.as_bytes().iter().map(|x| *x));
                if s.len() == 4 {
                    println!("{}", i+4);
                    break;
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}