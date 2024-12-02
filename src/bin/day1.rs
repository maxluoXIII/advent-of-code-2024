use advent_of_code_2024::Args;
use clap::Parser;

use nom::character::complete::{i32, space1};
use nom::sequence::{terminated, tuple};
use nom::IResult;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(input: &str) -> IResult<&str, (i32, i32)> {
    tuple((terminated(i32, space1), i32))(input)
}

fn main() {
    let args = Args::parse();
    println!("Input file: {}", args.input);

    let input_file = File::open(args.input).expect("Could not find file");
    let reader = BufReader::new(input_file);

    let mut locs1 = Vec::new();
    let mut locs2 = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Could not get line");
        if let Ok((_, (loc_id1, loc_id2))) = parse_line(&line) {
            locs1.push(loc_id1);
            locs2.push(loc_id2);
        } else {
            println!("Error parsing line: {}", line);
        }
    }

    locs1.sort();
    locs2.sort();

    let distance: i32 = locs1
        .iter()
        .zip(locs2.iter())
        .map(|(loc1, loc2)| (loc1 - loc2).abs())
        .sum();

    let mut loc2_index = 0;
    let mut score_map: HashMap<i32, i32> = HashMap::new();
    let mut sim_score = 0;
    for loc1 in locs1 {
        if !score_map.contains_key(&loc1) {
            let mut count = 0;
            while loc2_index < locs2.len() && locs2[loc2_index] <= loc1 {
                if loc1 == locs2[loc2_index] {
                    count += 1;
                }
                loc2_index += 1;
            }
            score_map.insert(loc1, loc1 * count);
        }
        sim_score += score_map[&loc1];
    }

    println!("distance: {}", distance);
    println!("similarity score: {}", sim_score);
}
