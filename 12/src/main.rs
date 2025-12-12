use std::fs;

use regex::Regex;

fn main() {
    part_1();
    // part_2();
}

#[derive(Debug, Clone)]
struct Shape {
    shape: Vec<Vec<u8>>,
}

#[derive(Debug, Clone)]
struct ShapeToFit {
    shape: Vec<Vec<u8>>,
    count: usize,
}

#[derive(Debug, Clone)]
struct Region {
    x: usize,
    y: usize,
    shape: Vec<Vec<u8>>,
    shapes_to_fit: Vec<Shape>,
}

#[allow(dead_code)]
fn part_1() {
    let contents =
        fs::read_to_string("./assets/input.txt").expect("Should have been able to read the file");

    let (shapes, regions) = process_input(&contents);

    // print_input(&shapes, &regions);

    let results = regions
        .iter()
        .map(|region| {
            let total_size_of_shapes = region
                .shapes_to_fit
                .iter()
                .map(|shape| {
                    shape
                        .shape
                        .iter()
                        .map(|l| l.iter().filter(|c| **c == ('#' as u8)).count())
                        .sum::<usize>()
                })
                .sum::<usize>();
            let region_area = region.x * region.y;
            total_size_of_shapes <= region_area
        })
        .collect::<Vec<bool>>();

    // let results = regions
    //     .iter()
    //     .map(region_fits_shapes)
    //     .collect::<Vec<bool>>();
    // println!("{:?}", results);
    println!("{:?}", results.iter().filter(|x| **x).count());
}

fn process_input(contents: &str) -> (Vec<Shape>, Vec<Region>) {
    let lines = contents.lines();
    let mut shapes: Vec<Shape> = Vec::new();
    let mut lines_slice: Vec<&str> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();
    let re = Regex::new(r"(\d+)x(\d+): (.+)").unwrap();

    for line in lines {
        if line.is_empty() {
            lines_slice.splice(0..=0, []);
            let without_first = lines_slice.iter().map(|f| f.as_bytes().to_vec()).collect();
            shapes.push(Shape {
                shape: without_first,
            });
            lines_slice.clear();
        } else if re.is_match(line) {
            let captures = re.captures(line).unwrap();
            let x = captures[1].parse::<usize>().unwrap();
            let y = captures[2].parse::<usize>().unwrap();
            let shape_counts = captures[3]
                .split(" ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            regions.push(Region {
                x,
                y,
                shape: vec![vec![0; x]; y]
                    .iter()
                    .map(|f| f.iter().map(|_| ' ' as u8).collect())
                    .collect(),
                shapes_to_fit: shape_counts
                    .iter()
                    .enumerate()
                    .flat_map(|(idx, cnt)| {
                        (0..*cnt)
                            .map(|_| shapes[idx].clone())
                            .collect::<Vec<Shape>>()
                    })
                    .collect(),
            });
        } else {
            lines_slice.push(line);
        }
    }

    return (shapes, regions);
}

fn region_fits_shapes(region: &Region) -> bool {
    let mut region_shape = region.shape.clone();
    let mut x = 0;
    let mut y = 0;

    for shape in &region.shapes_to_fit {
        let result = try_insert_shape_at_pos(&region_shape, &shape, x, y);
        if result.is_none() {
            return false;
        }

        region_shape = result.unwrap();
        if x < region_shape[0].len() {
            x += 1;
        } else {
            x = 0;
            y += 1;
        }
        if y >= region_shape.len() {
            return false;
        }
    }
    true
}

fn try_insert_shape_at_pos(
    region_shape: &Vec<Vec<u8>>,
    shape: &Shape,
    x: usize,
    y: usize,
) -> Option<Vec<Vec<u8>>> {
    if y + shape.shape.len() > region_shape.len()
        || x + shape.shape[0].len() > region_shape[0].len()
    {
        return None;
    }

    let relevant_lines = region_shape.clone()[y..y + shape.shape.len()].to_vec();
    let sub_region = relevant_lines
        .iter()
        .map(|f| f[x..x + shape.shape[0].len()].to_vec())
        .collect::<Vec<_>>();

    let fits = sub_region
        .iter()
        .all(|line| line.iter().all(|c| *c != '#' as u8));

    if !fits {
        return None;
    }

    let mut new_region = region_shape.clone();

    for y_idx in y..y + shape.shape.len() {
        for x_idx in x..x + shape.shape[0].len() {
            new_region[y_idx][x_idx] = shape.shape[y_idx - y][x_idx - x];
        }
    }
    Some(new_region)
}

fn print_input(shapes: &Vec<Shape>, regions: &Vec<Region>) {
    for shape in shapes {
        println!("Shape:");
        for row in &shape.shape {
            println!("{:?}", row.iter().map(|f| *f as char).collect::<String>());
        }
    }

    for region in regions {
        println!("Region: {:?}x{:?}", region.x, region.y);
        for row in &region.shape {
            println!("{:?}", row.iter().map(|f| *f as char).collect::<String>());
        }
        println!("Shapes to fit: {:?}", region.shapes_to_fit);
    }
}
