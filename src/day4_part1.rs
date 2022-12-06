use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use regex::Regex;

pub fn run() {
    let mut current_sum:u64 = 0;

    if let Ok(lines) = read_lines("inputs/day4.txt") {
        for line in lines {
            if let Ok(cal) = line {
                let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
                let values = re.captures_iter(cal.as_str()).map(|x| x.as_str().parse::<i32>().unwrap()).collect();

                println!("Month: {} Day: {} Year: {}", &values[2], &values[3], &values[1]);
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