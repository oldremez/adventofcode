use std::collections::hash_map::RandomState;
use std::collections::hash_set::Intersection;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::ptr::null;

pub fn run() {
    let mut current_sum:u64 = 0;
    let mut cur_line = 0;
    let mut sets : [HashSet<char>;3] = [HashSet::new(), HashSet::new(), HashSet::new()];
    if let Ok(lines) = read_lines("inputs/day3.txt") {
        for line in lines {
            if let Ok(cal) = line {
                let cur_step = cur_line % 3;
                if cur_step == 0 {
                    sets[0] = HashSet::from_iter(cal.chars());
                }
                if cur_step == 1 {
                    sets[1] = HashSet::from_iter(cal.chars());
                }
                if cur_step == 2 {
                    sets[2] = HashSet::from_iter(cal.chars());

                    let (intersection, others) = sets.split_at_mut(1);
                    let intersection = &mut intersection[0];
                    for other in others {
                        intersection.retain(|e| other.contains(e));
                    }
                    let res = intersection.clone().into_iter().next();
                    let c = res.unwrap();
                    current_sum += char_to_priority(c);
                }
                cur_line += 1;
                println!("{}", current_sum);

            }
        }
    }
    println!("{}", current_sum);

}

fn char_to_priority(c: char) -> u64 {
    if c.is_lowercase(){
        return (c as u64) - ('a' as u64) + 1;
    }
    if c.is_uppercase(){
        return (c as u64) - ('A' as u64) + 27;
    }
    return 0;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}