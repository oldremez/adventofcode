use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut current_sum = 0;
    let mut current_max = 0;
    if let Ok(lines) = read_lines("inputs/day1.txt") {
        for line in lines {
            if let Ok(cal) = line {
                if cal.len() == 0 {
                    if current_sum >  current_max {
                        current_max = current_sum;
                    }
                    current_sum = 0;
                } else {
                    let my_int = cal.parse::<i32>().unwrap();
                    current_sum += my_int;
                }
            }
        }
    }
    println!("{}", current_max);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}