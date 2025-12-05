use std::{collections::HashSet, fs};

fn main() {
    part_1();
    part_2();
}

fn part_1() {
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

fn part_2() {
    let contents = fs::read_to_string("./assets/input.txt")
    .expect("Should have been able to read the file");

    let ranges = contents.split(',');

    let mut invalid_ids: Vec<u64> = [].to_vec();

    for range in ranges {
        let mut invalid_ids_in_range: HashSet<u64> = HashSet::new();

        let range_iter = range.split('-');
        let range_vec: Vec<&str> = range_iter.collect();
        let start = u64::from_str_radix(range_vec[0], 10).unwrap(); // 10
        let end = u64::from_str_radix(range_vec[1], 10).unwrap(); // 3000

        let mut curr_val = start; // 10
        let start_digits = find_number_of_digits(start); // 2
        let end_digits = find_number_of_digits(end); // 4

        let digits_range = start_digits..=end_digits; // 2..4


        for curr_digit_max in digits_range {
            // 2, 3, 4. starts with 2.
            let mut curr_digit = curr_digit_max;


            while curr_digit > 1 {
                // first: 2 % 2 = 0
                if curr_digit_max % curr_digit != 0 {
                    curr_digit -= 1;
                    continue;
                }

                // 2 / 2 = 1
                let digits_slice = curr_digit_max / curr_digit;

                // get the max numerals of curr_val with curr_digit number of digits.
                // 10 / 10^(2 - 1) = 1
                let mut number_slice = (curr_val / 10u64.pow(curr_digit_max - digits_slice)).max(10u64.pow(digits_slice - 1));
                
                loop {   
                    // repeat that string curr_digits time. "1" + "1" = "11"
                    let invalid_id = u64::from_str_radix(number_slice.to_string().repeat((curr_digit) as usize).as_str(), 10).unwrap();

                    // 11 > 10 and 11 < 3000
                    if invalid_id >= start && invalid_id <= end {
                        // invalid_ids_in_range = [11]
                        invalid_ids_in_range.insert(invalid_id);
                    }
                    
                    // check if all chars of the slice are 9. In this case it's not, since it's 1.
                    if number_slice.to_string().chars().all(|x| x == '9') || invalid_id > end {
                        // if all chars are 9, we've reached the largest number for this quantity of digits, so we can proceed 
                        break;
                    }

                    // number slice now is 2. We're gonna check for 22, etc.
                    number_slice += 1;
                }
                // by the end of this loop, we added [11, 22, 33, ..., 99] to the set of invalid ids.
                

                curr_digit -= 1;
                curr_val = start
            }
        }


        invalid_ids.extend(invalid_ids_in_range.iter());
    }

    println!("{:?}", invalid_ids);
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