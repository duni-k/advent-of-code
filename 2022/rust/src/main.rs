mod lib;
use lib::*;

use std::path::PathBuf;

use anyhow::Result;
use chrono::Datelike;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short)]
    day: Option<u32>,
    #[arg(short, default_value_t = 1)]
    part: usize,
    input: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let day = match args.day {
        Some(d) => d,
        None => chrono::Utc::now().day(),
    };
    let lines = match args.input {
        Some(fname) => lib::read_file(fname)?,
        None => lib::read_default_input(day as usize)?,
    };

    let answer: String = match (day, args.part) {
        (_, d) if d > 2 => {
            println!("A day only has two parts, got {}", args.part);
            std::process::exit(1);
        }
        (1, 1) => d_i::get_top_calories(lines, 1),
        (1, 2) => d_i::get_top_calories(lines, 3),
        (2, 1) => lib::filter_map_then_sum(lines, d_ii::count_score),
        (2, 2) => lib::filter_map_then_sum(lines, d_ii::infer_score),
        (3, 1) => d_iii::sacks_sum(lines).to_string(),
        (3, 2) => d_iii::badges_sum(lines).to_string(),
        (4, 1) => d_iv::count(lines, d_iv::includes_superset).to_string(),
        (4, 2) => d_iv::count(lines, d_iv::includes_intersection).to_string(),
        (5, 1) => d_v::stack_tops(lines, 9000)?,
        (5, 2) => d_v::stack_tops(lines, 9001)?,
        (6, 1) => d_vi::start_marker_position(lines, 4)?.to_string(),
        (6, 2) => d_vi::start_marker_position(lines, 14)?.to_string(),
        _ => todo!(),
    };

    println!("Answer for day {}, part {}: {}", day, args.part, answer);
    Ok(())
}
