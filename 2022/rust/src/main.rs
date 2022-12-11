#![feature(let_chains, iter_intersperse)]
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
    let today = chrono::Utc::now().day();
    let day = args.day.unwrap_or(today);

    let input = if let Some(fname) = args.file {
        r::read_file(&fname)?
    } else {
        r::read_or_fetch(day)?
    };

    let answer: String = match (day, args.part) {
        (_, d) if d > 2 => {
            println!("A day only has two parts, got {}", args.part);
            std::process::exit(1);
        }
        (1, 1) => i::get_top_calories(&input, 1).to_string(),
        (1, 2) => i::get_top_calories(&input, 3).to_string(),
        (2, 1) => ii::filter_map_then_sum(&input, ii::count_score).to_string(),
        (2, 2) => ii::filter_map_then_sum(&input, ii::infer_score).to_string(),
        (3, 1) => iii::sacks_sum(&input).to_string(),
        (3, 2) => iii::badges_sum(&input).to_string(),
        (4, 1) => iv::count(&input, iv::includes_superset).to_string(),
        (4, 2) => iv::count(&input, iv::includes_intersection).to_string(),
        (5, 1) => v::stack_tops(&input, 9000)?,
        (5, 2) => v::stack_tops(&input, 9001)?,
        (6, 1) => vi::start_marker_position(&input, 4).to_string(),
        (6, 2) => vi::start_marker_position(&input, 14).to_string(),
        (7, 1) => vii::sum_small_dirs(&input).to_string(),
        (7, 2) => vii::smallest_viable_deletion(&input, 30_000_000).to_string(),
        (8, 1) => viii::visible_trees(&input).to_string(),
        (8, 2) => viii::top_scenic_score(&input).to_string(),
        (9, 1) => ix::visited_by_tail(&input, 2).to_string(),
        (9, 2) => ix::visited_by_tail(&input, 10).to_string(),
        (10, 1) => x::signal_strength(&input).to_string(),
        (10, 2) => x::letters_from_crt(&input),
        (11, 1) => xi::part_one(&input).to_string(),
        (11, 2) => xi::part_two(&input).to_string(),
        _ => todo!(),
    };

    Ok(println!(
        "Answer for day {}, part {}: {}",
        day, args.part, answer
    ))
}
