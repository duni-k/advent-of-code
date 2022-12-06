pub mod d_i;
pub mod d_ii;
pub mod d_iii;
pub mod d_iv;
pub mod d_v;
pub mod d_vi;

use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Flatten,
    path::PathBuf,
};

use anyhow::{Context, Result};
use numerals::roman::Roman;

const INP_PATH: &'static str = "/home/dnk/Projects/advent-of-code/2022/rust/inputs/d_";

pub fn filter_map_then_sum<L, F>(lines: L, f: F) -> String
where
    F: Fn(&'_ str) -> Option<usize>,
    L: Iterator<Item = String>,
{
    lines.filter_map(|line| f(&line)).sum::<usize>().to_string()
}

pub fn read_default_input(day: usize) -> Result<Flatten<Lines<BufReader<File>>>> {
    let input_file = format!("{INP_PATH}{:x}", Roman::from(day as i16));
    let f = File::open(&input_file).context(format!(
        "Failed opening input file \"{}\", is the day really implemented?",
        &input_file
    ))?;
    let reader = BufReader::new(f);
    Ok(reader.lines().flatten())
}

pub fn read_file(fname: PathBuf) -> Result<Flatten<Lines<BufReader<File>>>> {
    let f = File::open(&fname).context(format!(
        "Failed opening input file \"{:#?}\", did you provide the correct path?",
        &fname
    ))?;
    let reader = BufReader::new(f);
    Ok(reader.lines().flatten())
}
