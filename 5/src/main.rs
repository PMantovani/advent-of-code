use std::{collections::HashSet, fs, ops::RangeInclusive};

fn main() {
    // part_1();
    part_2();
}

#[allow(dead_code)]
fn part_1() {
    let contents = fs::read_to_string("./assets/input.txt")
    .expect("Should have been able to read the file");

    let lines = contents.lines();

    let mut fresh_ranges_str = Vec::new();
    let mut ingredients = HashSet::new();

    lines.for_each(|line| {
        if line.contains("-") {
            fresh_ranges_str.push(line);
        } else if line != "" {
            ingredients.insert(u64::from_str_radix(line, 10).unwrap());
        }
    });

    let ranges = convert_from_ranges_str(fresh_ranges_str);

    let count = ingredients.iter().filter(|i| {
        ranges.iter().any(|r| r.contains(i))
    }).count();

    println!("{:?}", count);

}

fn part_2() {
    let contents = fs::read_to_string("./assets/input.txt")
    .expect("Should have been able to read the file");

    let lines = contents.lines();

    let mut fresh_ranges_str = Vec::new();

    lines.for_each(|line| {
        if line.contains("-") {
            fresh_ranges_str.push(line);
        }
    });

    let ranges = convert_from_ranges_str(fresh_ranges_str);
    let mut jointed_sets: Vec<Option<RangeInclusive<u64>>> = ranges.iter().cloned().map(Some).collect();

    ranges.into_iter().enumerate().for_each(|(idx, r)| {
        let mut cmp_set = r;

        for j in 0..idx {
            let Some(j_set) = jointed_sets[j].clone() else { continue };

            if cmp_set.end() >= j_set.start() && cmp_set.start() <= j_set.end() {
                // sets are connected, merge them!
                let new_end = *cmp_set.end().max(j_set.end());
                let new_start = *cmp_set.start().min(j_set.start());
                cmp_set = new_start..=new_end;
                jointed_sets[j] = None;
            }
        }
        jointed_sets[idx] = Some(cmp_set);
    });

    let count: u64 = jointed_sets.iter()
        .map(|s| s.as_ref().map_or(0, |r| r.end() - r.start() + 1))
        .sum();

    println!("{:?}", count);
}


fn convert_from_ranges_str(ranges_str: Vec<&str>) -> Vec<RangeInclusive<u64>> {
    let mut ranges = Vec::new();
    ranges_str.iter().for_each(|str| {
        let [start_str, end_str] = str.split('-').collect::<Vec<&str>>().try_into().expect("Not valid array");
        let start = u64::from_str_radix(start_str, 10).unwrap();
        let end = u64::from_str_radix(end_str, 10).unwrap();
        ranges.push(start..=end);
    });
    return ranges;
}