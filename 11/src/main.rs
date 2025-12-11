use std::{collections::HashMap, fs};

fn main() {
    part_1();
    // part_2();
}

#[derive(Debug, Clone)]
struct Device {
    label: String,
    outputs: Vec<String>,
}

#[allow(dead_code)]
fn part_1() {
    let contents =
        fs::read_to_string("./assets/input.txt").expect("Should have been able to read the file");

    let devices = process_input(&contents);
    let start = &devices["you"];

    println!("Count: {:?}", dfs_count_start(start, &devices));
}

fn dfs_count_start(device: &Device, devices: &HashMap<String, Device>) -> u64 {
    if device.label == "out" {
        return 1;
    }

    device.outputs.iter().fold(0, |acc, x| {
        let next = &devices[x];
        acc + dfs_count_start(next, devices)
    })
}

fn process_input(contents: &str) -> HashMap<String, Device> {
    let lines = contents.lines();
    let mut map = lines
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let device = Device {
                label: parts[0].to_string(),
                outputs: parts[1].split(" ").map(|x| x.to_string()).collect(),
            };
            (device.label.clone(), device)
        })
        .collect::<HashMap<String, Device>>();

    map.insert(
        "out".to_string(),
        Device {
            label: "out".to_string(),
            outputs: vec![],
        },
    );

    map
}
