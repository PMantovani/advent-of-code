use std::fs;

fn main() {
    let contents = fs::read_to_string("./assets/input.txt")
        .expect("Should have been able to read the file");

    let ranges = contents.split(',');

    let mut invalid_ids: Vec<u64> = [].to_vec();

    for range in ranges {
        let range_iter = range.split('-');
        let range_vec: Vec<&str> = range_iter.collect();
        let start = u64::from_str_radix(range_vec[0], 10).unwrap();
        let end = u64::from_str_radix(range_vec[1], 10).unwrap();

        let mut curr_val = start;
        while curr_val <= end {
            let mut digits = find_number_of_digits(curr_val);
            if digits % 2 != 0 {
                digits += 1;
                curr_val = 10u64.pow(digits - 1);
                continue;
            }
            let half_digits = digits / 2;
            let upper_half = curr_val / 10u64.pow(half_digits);
            let next_repeating = upper_half * 10u64.pow(half_digits) + upper_half;
            if next_repeating <= end && next_repeating >= start {
                invalid_ids.push(next_repeating);
            }
            curr_val = (upper_half + 1) * 10u64.pow(half_digits) + (upper_half + 1);
        }
    }

    // println!("{:?}", invalid_ids);
    let sum = invalid_ids.iter().fold(0, |acc, x| acc + x);
    println!("{sum}");
}

fn find_number_of_digits(number: u64) -> u32 {
    if number == 0 {
        1
    } else {
        ((number as f64).log10().floor() as u32) + 1
    }
}