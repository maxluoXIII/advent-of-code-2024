use advent_of_code_2024::Args;
use clap::Parser;

use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    let args = Args::parse();
    let input_file = File::open(args.input).expect("Could not open input file");
    let reader = BufReader::new(input_file);

    let re =
        Regex::new(r"mul\((?P<op1>\d{1,3}),(?P<op2>\d{1,3})\)").expect("Could not compile regex");
    for line in reader.lines() {
        let line = line.expect("Could not get line");
        let sum: u32 = re
            .captures_iter(&line)
            .map(|caps| &caps["op1"].parse::<u32>().unwrap() * &caps["op2"].parse::<u32>().unwrap())
            .sum();
        println!("sum: {}", sum);
    }
}
