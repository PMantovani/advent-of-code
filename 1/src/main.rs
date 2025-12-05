use std::{fs};

fn main() {
    let contents = fs::read_to_string("./assets/input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();

    // process_part1(lines);
    process_part2(lines);
}

/*
fn process_part1(lines: Vec<&str>) {
    let mut password = 0;
    let mut value = 50;

    for line in lines {
        let rotation = &line[0..1];
        let distance_result = i32::from_str_radix(&line[1..], 10);
        
        if distance_result.is_err() {
            println!("Can't parse number");
            return;
        }

        if rotation == "L" {
            value += distance_result.unwrap();
        } else {
            value -= distance_result.unwrap();
        }

        while value < 0 {
            value += 100;
        }

        while value > 99 {
            value -= 100;
        }

        if value == 0 {
            password += 1;
        }
    }

    println!("{password}");
}
*/

fn process_part2(lines: Vec<&str>) {
    let mut password = 0;
    let mut value = 50;

    for line in lines {
        let rotation = &line[0..1];
        let distance_result = i32::from_str_radix(&line[1..], 10);
        
        if distance_result.is_err() {
            println!("Can't parse number");
            return;
        }

        let old_value = value;

        if rotation == "L" {
            value -= distance_result.unwrap();
        } else {
            value += distance_result.unwrap();
        }

        let num_wraps = num_of_wraps(value, old_value);
        let new_val = next_dial(value);
        println!("passincrement {num_wraps} newValue {new_val}");
        password += num_of_wraps(value, old_value);
        value = next_dial(value);

    }

    println!("{password}");
}

fn num_of_wraps(dial: i32, old_dial: i32) -> i32 {
    match dial {
        0 => 1,
        1.. => dial / 100,
        _ => (dial.abs() / 100) + if old_dial == 0 { 0 } else { 1 }
    }
}

fn next_dial(dial: i32) -> i32 {
    if dial < 0 {
        let mult = dial.abs() / 100;
        return ((100 * (mult + 1)) - dial.abs()) % 100;
    }
    return dial % 100
}