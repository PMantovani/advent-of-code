use std::{fs};

fn main() {
    let contents = fs::read_to_string("./assets/input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
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
