use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn solve() -> u32 {
    let mut sum = 0;
    for line in read_lines("./src/day1/input.txt").unwrap() {
        let line_string = line.unwrap();
        let mut found_first = false;
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        for character in line_string.chars(){
            if let Some(unwrapped_char) = character.to_digit(10){
                if !found_first {
                    first = unwrapped_char;
                    found_first = true;
                }
                last = unwrapped_char
            }
        }
        sum += first * 10 + last;
    }
    return sum;
}