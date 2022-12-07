use std::borrow::Borrow;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::ffi::c_char;
use regex::Regex;

pub fn run() {
    let ls = 8;
    let cs = 9;
    if let Ok(lines) = read_lines("inputs/day5.txt") {
        let llines: Vec<String> = lines.map(|x| x.unwrap()).collect();
        let mut field: Vec<Vec<char>> = Vec::new();
        for i in 0..cs {
            field.push(Vec::new());
        }

        for i in 0..ls {
            for j in 0..cs {
                let z = llines[ls - 1 - i].as_bytes()[j * 4 + 1] as char;
                if z !=' ' {
                    field[j].push(z);
                }
            }
        }

        for line in llines.into_iter().skip(ls + 2) {
            if let cal = line {
                let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
                let values = re.captures_iter(cal.as_str()).next().unwrap();
                let vvv: Vec<i32> = values.iter().skip(1).map(|x| x.unwrap().as_str().parse::<i32>().unwrap()).collect();
                let mut ggg: Vec<char> = Vec::new();
                for i in 0..vvv[0] {
                    let z = field[(vvv[1] - 1) as usize].pop().unwrap();
                    ggg.push(z);
                }
                for i in 0..vvv[0] {
                    let z = ggg.pop().unwrap();
                    field[(vvv[2] - 1) as usize].push(z);
                }


                println!("{}", cal);
                for a in &field {
                    for b in a {
                        print!("{} ", b)
                    }
                    println!();
                }
            }
        }
        for i in 0..cs {
            print!("{}", field[i].pop().unwrap());
        }
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}