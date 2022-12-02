use anyhow::{anyhow, Result};
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[allow(dead_code)]
mod d_one;
mod d_two;

fn main() -> Result<()> {
    let relative = "/home/dnk/Projects/advent/2022/rust/src";
    let args = Argument::new(env::args().collect());
    let lines = match args.day {
        1 => read_lines(format!("{relative}/d_one/d_one.input"))?,
        2 => read_lines(format!("{relative}/d_two/d_two.input"))?,
        _ => return Err(anyhow!("Day not implemented.")),
    };

    println!(
        "Answer for day {}, part {}: {}",
        args.day,
        args.part,
        match (args.day, args.part) {
            (_, d) if d > 2 =>
                return Err(anyhow!(format!(
                    "A day only has two parts, got {}",
                    args.part
                ))),
            (1, 1) => d_one::get_top_calories(lines, 1),
            (1, 2) => d_one::get_top_calories(lines, 3),
            (2, 1) => d_two::process_games(lines, d_two::count_score),
            (2, 2) => d_two::process_games(lines, d_two::infer_score),
            _ => unreachable!(), // args.day > n is already covered in first match expression
        }
    );

    Ok(())
}

fn read_lines(fname: String) -> io::Result<impl Iterator<Item = String>> {
    let f = File::open(fname)?;
    let buf_reader = BufReader::new(f);
    Ok(buf_reader.lines().filter_map(|l| l.ok()))
}

struct Argument {
    day: usize,
    part: usize,
}

impl Argument {
    fn new(args: Vec<String>) -> Argument {
        if args.len() < 3 {
            panic!("Not enough arguments, specify day and input file");
        }
        Argument {
            day: args[1]
                .parse::<usize>()
                .expect("First argument could not be parsed as number (for number of day)."),
            part: args[2]
                .parse::<usize>()
                .expect("Second argument could not be parsed as number (for part of day)."),
        }
    }
}
