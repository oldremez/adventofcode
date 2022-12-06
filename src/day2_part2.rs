use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut current_sum = 0;
    if let Ok(lines) = read_lines("inputs/day2.txt") {
        for line in lines {
            if let Ok(cal) = line {
                current_sum += match cal.as_str() {
                    "A X" => 3, //1 + 3,
                    "A Y" => 4, //2 + 6,
                    "A Z" => 8, //3,
                    "B X" => 1,
                    "B Y" => 2 + 3,
                    "B Z" => 3 + 6,
                    "C X" => 2, //1 + 6,
                    "C Y" => 6, //2,
                    "C Z" => 7, //3 + 3,
                    &_ => 0,
                };
            }
        }
    }
    println!("{}", current_sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}