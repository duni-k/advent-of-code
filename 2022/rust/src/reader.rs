use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Flatten,
    path::PathBuf,
};

use anyhow::{Context, Result};
use numerals::roman::Roman;

const INP_PATH: &'static str = "/home/dnk/Projects/advent-of-code/2022/rust/inputs/d_";

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
        "Failed opening input file {:#?}, did you provide the correct path?\
         \nNOTE: Pathname should be relative.",
        &fname
    ))?;
    let reader = BufReader::new(f);
    Ok(reader.lines().flatten())
}
