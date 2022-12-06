mod day;
mod reader;

use day::*;
use reader as r;

use std::path::PathBuf;

use anyhow::Result;
use chrono::Datelike;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    /// [default: current day of the month].
    day: Option<u32>,
    #[arg(short, long, default_value_t = 1)]
    part: usize,
    #[arg(short, long)]
    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let day = args.day.unwrap_or(chrono::Utc::now().day());

    let lines = match args.file {
        Some(fname) => r::read_file(fname)?,
        None => r::read_default_input(day as usize)?,
    };

    let answer: String = match (day, args.part) {
        (_, d) if d > 2 => {
            println!("A day only has two parts, got {}", args.part);
            std::process::exit(1);
        }
        (1, 1) => i::get_top_calories(lines, 1),
        (1, 2) => i::get_top_calories(lines, 3),
        (2, 1) => ii::filter_map_then_sum(lines, ii::count_score),
        (2, 2) => ii::filter_map_then_sum(lines, ii::infer_score),
        (3, 1) => iii::sacks_sum(lines).to_string(),
        (3, 2) => iii::badges_sum(lines).to_string(),
        (4, 1) => iv::count(lines, iv::includes_superset).to_string(),
        (4, 2) => iv::count(lines, iv::includes_intersection).to_string(),
        (5, 1) => v::stack_tops(lines, 9000)?,
        (5, 2) => v::stack_tops(lines, 9001)?,
        (6, 1) => vi::start_marker_position(lines, 4).to_string(),
        (6, 2) => vi::start_marker_position(lines, 14).to_string(),
        _ => todo!(),
    };

    Ok(println!(
        "Answer for day {}, part {}: {}",
        day, args.part, answer
    ))
}
