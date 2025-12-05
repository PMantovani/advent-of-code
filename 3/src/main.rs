use std::{fs};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let contents = fs::read_to_string("./assets/input.txt")
    .expect("Should have been able to read the file");

    let lines = contents.lines();
    let mut banks_joltage: Vec<u32> = Vec::new();

    for line in lines {
        let chars = line.chars();
        let line_length = chars.clone().count();
        let max_first = chars.clone().enumerate().fold(
            ('0', 0 as usize),
            |acc, (idx, val)|  {
                match val > acc.0 && idx < line_length - 1 {
                    true => (val, idx),
                    false => acc,
                }
            } );
        let idx_first_digit = max_first.1;

        let chars_second = chars.as_str()[(idx_first_digit+1)..].chars();
        let max_second = chars_second.max().unwrap_or('0');

        let mut num_in_str = max_first.0.to_string();
        num_in_str.push(max_second);
        banks_joltage.push(u32::from_str_radix(num_in_str.as_str(), 10).unwrap_or(0));
    }
    // println!("{:?}", banks_joltage);
    // let total: u32 = banks_joltage.iter().sum();
    // println!("{:?}", total);
}


fn part_2() {
    let contents = fs::read_to_string("./assets/input.txt")
    .expect("Should have been able to read the file");

    let lines = contents.lines();
    let mut banks_joltage: Vec<u64> = Vec::new();

    
    for line in lines {
        let mut built_number = String::new();
        let chars = line.chars();
        let mut curr_digit = 0;
        let mut pending_chars = chars.clone();

        while curr_digit < 12 {
            let available_length = pending_chars.clone().count();
            let max = pending_chars.clone().enumerate().fold(
                ('0', 0 as usize),
                |acc, (idx, val)|  {
                    match val > acc.0 && idx < available_length - (12 - curr_digit - 1) {
                        true => (val, idx),
                        false => acc,
                    }
                } );
            built_number.push(max.0);
            curr_digit += 1;
            pending_chars = pending_chars.as_str()[(max.1+1)..].chars();
        }

        banks_joltage.push(u64::from_str_radix(built_number.as_str(), 10).unwrap_or(0));
    }
    println!("{:?}", banks_joltage);
    let total: u64 = banks_joltage.iter().sum();
    println!("{:?}", total);
}
