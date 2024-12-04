use advent_of_code_2024::Args;
use clap::Parser;
use nom::multi::many1;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use nom::bytes::complete::take_while1;
use nom::character::complete::{space0, u32};
use nom::character::is_digit;
use nom::sequence::terminated;
use nom::IResult;

fn parse_level(input: &str) -> IResult<&str, u32> {
    terminated(u32, space0)(input)
}

fn parse_levels(input: &str) -> IResult<&str, Vec<u32>> {
    many1(parse_level)(input)
}

enum LevelStates {
    FirstIncreasing,
    Increasing,
    FirstDecreasing,
    Decreasing,
    Unknown,
}

fn is_safe(levels: &[u32]) -> bool {
    if levels.len() > 0 {
        _is_safe(LevelStates::Unknown, 0, 1, levels)
    } else {
        true
    }
}

fn _is_safe(state: LevelStates, prev: usize, curr: usize, levels: &[u32]) -> bool {
    if curr == levels.len() {
        return true;
    }

    match state {
        LevelStates::Unknown => {
            if levels[curr] > levels[prev] && levels[curr] - levels[prev] <= 3 {
                _is_safe(LevelStates::FirstIncreasing, curr, curr + 1, levels)
            } else if levels[curr] < levels[prev] && levels[prev] - levels[curr] <= 3 {
                _is_safe(LevelStates::FirstDecreasing, curr, curr + 1, levels)
            } else {
                false
            }
        }
        LevelStates::FirstIncreasing => {
            if levels[curr] > levels[prev] && levels[curr] - levels[prev] <= 3 {
                _is_safe(LevelStates::Increasing, curr, curr + 1, levels)
            } else {
                false
            }
        }
        LevelStates::Increasing => {
            if levels[curr] > levels[prev] && levels[curr] - levels[prev] <= 3 {
                _is_safe(LevelStates::Increasing, curr, curr + 1, levels)
            } else {
                false
            }
        }
        LevelStates::FirstDecreasing => {
            if levels[curr] < levels[prev] && levels[prev] - levels[curr] <= 3 {
                _is_safe(LevelStates::Decreasing, curr, curr + 1, levels)
            } else {
                false
            }
        }
        LevelStates::Decreasing => {
            if levels[curr] < levels[prev] && levels[prev] - levels[curr] <= 3 {
                _is_safe(LevelStates::Decreasing, curr, curr + 1, levels)
            } else {
                false
            }
        }
    }
}

fn is_loose_safe(levels: &[u32]) -> bool {
    if levels.len() > 0 {
        _is_loose_safe(LevelStates::Unknown, 0, 1, levels)
    } else {
        true
    }
}

fn _is_loose_safe(state: LevelStates, prev: usize, curr: usize, levels: &[u32]) -> bool {
    if curr == levels.len() {
        return true;
    }

    match state {
        LevelStates::Unknown => {
            if levels[curr] > levels[prev] && levels[curr] - levels[prev] <= 3 {
                _is_loose_safe(LevelStates::FirstIncreasing, curr, curr + 1, levels)
            } else if levels[curr] < levels[prev] && levels[prev] - levels[curr] <= 3 {
                _is_loose_safe(LevelStates::FirstDecreasing, curr, curr + 1, levels)
            } else {
                if prev == 0 {
                    _is_safe(LevelStates::Unknown, prev, curr + 1, levels)
                        || _is_safe(LevelStates::Unknown, curr, curr + 1, levels)
                } else {
                    _is_safe(LevelStates::Unknown, prev - 1, curr, levels)
                        || _is_safe(LevelStates::Unknown, prev, curr + 1, levels)
                }
            }
        }
        LevelStates::FirstIncreasing => {
            if levels[curr] > levels[prev] && levels[curr] - levels[prev] <= 3 {
                _is_loose_safe(LevelStates::Increasing, curr, curr + 1, levels)
            } else {
                _is_safe(LevelStates::Unknown, prev - 1, curr, levels)
                    || _is_safe(LevelStates::Unknown, prev, curr + 1, levels)
            }
        }
        LevelStates::Increasing => {
            if levels[curr] > levels[prev] && levels[curr] - levels[prev] <= 3 {
                _is_loose_safe(LevelStates::Increasing, curr, curr + 1, levels)
            } else {
                _is_safe(LevelStates::Increasing, prev - 1, curr, levels)
                    || _is_safe(LevelStates::Increasing, prev, curr + 1, levels)
            }
        }
        LevelStates::FirstDecreasing => {
            if levels[curr] < levels[prev] && levels[prev] - levels[curr] <= 3 {
                _is_loose_safe(LevelStates::Decreasing, curr, curr + 1, levels)
            } else {
                _is_safe(LevelStates::Unknown, prev - 1, curr, levels)
                    || _is_safe(LevelStates::Unknown, prev, curr + 1, levels)
            }
        }
        LevelStates::Decreasing => {
            if levels[curr] < levels[prev] && levels[prev] - levels[curr] <= 3 {
                _is_loose_safe(LevelStates::Decreasing, curr, curr + 1, levels)
            } else {
                _is_safe(LevelStates::Decreasing, prev - 1, curr, levels)
                    || _is_safe(LevelStates::Decreasing, prev, curr + 1, levels)
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    println!("Input file: {}", args.input);

    let input_file = File::open(args.input).expect("Could not open input file");
    let reader = BufReader::new(input_file);

    let mut levels = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Could not get line");
        if let Ok((_, line_levels)) = parse_levels(&line) {
            levels.push(line_levels);
        }
    }

    let safe_count = levels
        .iter()
        .filter(|line_levels| {
            line_levels
                .iter()
                .is_sorted_by(|a, b| a < b && **b - **a <= 3)
                || line_levels
                    .iter()
                    .is_sorted_by(|a, b| a > b && **a - **b <= 3)
        })
        .count();

    let loose_count = levels
        .iter()
        .filter(|line_levels| is_loose_safe(line_levels))
        .count();

    let alt_safe_count = levels
        .iter()
        .filter(|line_levels| is_safe(line_levels))
        .count();

    println!("safe count: {}", safe_count);
    println!("loose safe count: {}", loose_count);
    println!("alt safe count: {}", alt_safe_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn up_down_loose() {
        let result = is_loose_safe(&[10, 11, 9, 7, 5]);
        assert!(result);
    }

    #[test]
    fn down_up_loose() {
        let result = is_loose_safe(&[10, 9, 11, 13]);
        assert!(result);
    }

    #[test]
    fn up_down_loose_end() {
        let result = is_loose_safe(&[10, 11, 14, 13]);
        assert!(result);
    }

    #[test]
    fn big_gap_loose_end() {
        let result = is_loose_safe(&[10, 11, 15]);
        assert!(result);
    }

    #[test]
    fn begin_dup() {
        let result = is_loose_safe(&[10, 10, 11, 13]);
        assert!(result);
    }

    #[test]
    fn double_dup() {
        let result = is_loose_safe(&[10, 10, 11, 11, 13]);
        assert!(!result);
    }
}
