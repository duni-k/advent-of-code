pub mod d_i;
pub mod d_ii;
pub mod d_iii;
pub mod d_iv;
pub mod d_v;

use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Flatten,
};

use anyhow::{anyhow, Context, Result};
use numerals::roman::Roman;

const INP_PATH: &'static str = "/home/dnk/Projects/advent-of-code/2022/rust/inputs/d_";

pub fn filter_map_then_sum<L, F>(lines: L, f: F) -> String
where
    F: Fn(&'_ str) -> Option<usize>,
    L: Iterator<Item = String>,
{
    lines.filter_map(|line| f(&line)).sum::<usize>().to_string()
}

pub fn read_lines(day: usize) -> Result<Flatten<Lines<BufReader<File>>>> {
    let input_file = format!("{INP_PATH}{:x}", Roman::from(day as i16));
    let f = File::open(&input_file).context(format!(
        "Failed opening input file \"{}\", is the day really implemented?",
        &input_file
    ))?;
    let reader = BufReader::new(f);
    Ok(reader.lines().flatten())
}

pub struct Argument {
    pub day: usize,
    pub part: usize,
}

impl Argument {
    pub fn new(args: Vec<String>) -> Result<Argument> {
        if args.len() < 3 {
            return Err(anyhow!(
                "Not enough arguments, specify day and input file in that order."
            ));
        } else if let (Ok(day), Ok(part)) = (args[1].parse::<usize>(), args[2].parse::<usize>()) {
            Ok(Argument { day, part })
        } else {
            return Err(anyhow!("Could not parse arguments."));
        }
    }
}
