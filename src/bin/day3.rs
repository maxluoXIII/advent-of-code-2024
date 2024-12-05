use advent_of_code_2024::Args;
use clap::Parser;

use std::fs::File;
use std::io::{BufReader, Read};

use regex::Regex;

#[derive(Debug)]
enum Command {
    Mul(u32, u32),
    Do,
    Dont,
}

fn main() {
    let args = Args::parse();
    let input_file = File::open(args.input).expect("Could not open input file");
    let mut reader = BufReader::new(input_file);

    let mul_re = Regex::new(r"mul\((?P<op1>\d{1,3}),(?P<op2>\d{1,3})\)")
        .expect("Could not compile 'mul' regex");
    let do_re = Regex::new(r"do\(\)").expect("Could not compile 'do' regex");
    let dont_re = Regex::new(r"don't\(\)").expect("Could not compile 'dont' regex");

    let mut input = String::new();
    reader
        .read_to_string(&mut input)
        .expect("Could not read file to string");

    let simple_sum: u32 = mul_re
        .captures_iter(&input)
        .map(|caps| &caps["op1"].parse::<u32>().unwrap() * &caps["op2"].parse::<u32>().unwrap())
        .sum();
    println!("simple sum: {}", simple_sum);

    let mul_iter = mul_re.captures_iter(&input).map(|caps| {
        (
            caps.get(0).unwrap().start(),
            Command::Mul(
                caps["op1"].parse::<u32>().unwrap(),
                caps["op2"].parse::<u32>().unwrap(),
            ),
        )
    });
    let do_iter = do_re.find_iter(&input).map(|m| (m.start(), Command::Do));
    let dont_iter = dont_re.find_iter(&input).map(|m| (m.start(), Command::Dont));

    let mut commands = mul_iter.chain(do_iter).chain(dont_iter).collect::<Vec<(usize, Command)>>();
    commands.sort_by_key(|k| k.0);
    let acc_sum = commands.iter().map(|(_, command)| command).fold((true, 0), |(enabled, sum), command| {
        match command {
            Command::Do => (true, sum),
            Command::Dont => (false, sum),
            Command::Mul(op1, op2) => {
                if enabled {
                    (enabled, sum + op1 * op2)
                } else {
                    (enabled, sum)
                }
            }
        }
    }).1;
    println!("Accurate sum: {}", acc_sum);
}
