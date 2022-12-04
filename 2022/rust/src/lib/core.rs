use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::{anyhow, Result};

pub fn filter_map_then_sum<L, F>(lines: L, f: F) -> usize
where
    F: Fn(&'_ str) -> Option<usize>,
    L: Iterator<Item = String>,
{
    lines.filter_map(|line| f(&line)).sum()
}

pub fn read_lines(fname: String) -> io::Result<impl Iterator<Item = String>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);
    Ok(reader.lines().filter_map(|l| l.ok()))
}

pub struct Argument {
    pub day: usize,
    pub part: usize,
}

impl Argument {
    pub fn new(args: Vec<String>) -> Result<Argument> {
        if args.len() < 3 {
            return Err(anyhow!("Not enough arguments, specify day and input file."));
        } else if let (Ok(day), Ok(part)) = (args[1].parse::<usize>(), args[2].parse::<usize>()) {
            Ok(Argument { day, part })
        } else {
            return Err(anyhow!("Could not parse arguments."));
        }
    }
}
