use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use regex::Regex;
use std::collections::HashMap;
use crate::day7_part1::Obj::{Dir, Empty};

enum Obj {
    Empty,
    Dir(HashMap<String, Obj>),
    File(usize)
}

pub fn run() {
    let mut current_sum:u64 = 0;

    if let Ok(lines) = read_lines("inputs/day7.txt") {
        let mut current_dir: Vec<String> = Vec::new();
        let root: Obj = Dir(HashMap::new());

        let cd_regex = Regex::new(r"\$ cd (.*)").unwrap();
        let ls_regex = Regex::new(r"\$ ls").unwrap();
        let dir_regex = Regex::new(r"dir (.*)").unwrap();
        let file_regex = Regex::new(r"(\d+) (.*)").unwrap();
        let mut cur_dir: &Obj = &root;
        for line in lines {
            if let Ok(cal) = line {
                // println!("{}", cal);
                if cd_regex.is_match(cal.as_str()){
                    let values = cd_regex.captures_iter(cal.as_str()).next().unwrap();
                    let vvv: Vec<&str> = values.iter().skip(1).map(|x| x.unwrap().as_str()).collect();
                    // println!("{}", vvv[0]);
                    cur_dir = match vvv[0] {
                        "/" => &root,
                        ".." => &Obj::Empty,
                        &_ => {
                            match cur_dir {
                                Empty => &Obj::Empty,
                                Dir(mut m) => {
                                    if !m.contains_key(vvv[0]){
                                        m.insert(vvv[0].to_string(), Obj::Dir(HashMap::new()));
                                    }
                                    &m.get(vvv[0]).unwrap()
                                }
                                Obj::File(_) => &Obj::Empty,
                            }
                        },
                    };
                }
                if ls_regex.is_match(cal.as_str()){
                    // do nothing lol
                }
                if dir_regex.is_match(cal.as_str()){
                    // do nothing lol
                }
                if file_regex.is_match(cal.as_str()){

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