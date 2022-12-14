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

    print!("Answer for day {day}, part {}: ", args.part);

    if day == 10 || day == 5 {
        println!(
            "{}",
            match (day, args.part) {
                (10, 1) => x::signal_strength(&input),
                (10, 2) => x::letters_from_crt(&input),
                (5, 1) => v::stack_tops(&input, 9000)?,
                (5, 2) => v::stack_tops(&input, 9001)?,
                _ => unreachable!(),
            }
        );
    } else {
        println!(
            "{}",
            match (day, args.part) {
                (_, d) if d > 2 => {
                    println!("A day only has two parts, got {d}");
                    std::process::exit(1);
                }
                (1, 1) => i::get_top_calories(&input, 1),
                (1, 2) => i::get_top_calories(&input, 3),
                (2, 1) => ii::filter_map_then_sum(&input, ii::count_score),
                (2, 2) => ii::filter_map_then_sum(&input, ii::infer_score),
                (3, 1) => iii::sacks_sum(&input),
                (3, 2) => iii::badges_sum(&input),
                (4, 1) => iv::count(&input, iv::includes_superset),
                (4, 2) => iv::count(&input, iv::includes_intersection),
                (6, 1) => vi::start_marker_position(&input, 4),
                (6, 2) => vi::start_marker_position(&input, 14),
                (7, 1) => vii::sum_small_dirs(&input),
                (7, 2) => vii::smallest_viable_deletion(&input, 30_000_000),
                (8, 1) => viii::visible_trees(&input),
                (8, 2) => viii::top_scenic_score(&input),
                (9, 1) => ix::visited_by_tail(&input, 2),
                (9, 2) => ix::visited_by_tail(&input, 10),
                (11, 1) => xi::part_one(&input),
                (11, 2) => xi::part_two(&input),
                (12, 1) => xii::shortest_path_from(&input, 'S'),
                (12, 2) => xii::shortest_path_from(&input, 'a'),
                (13, 1) => xiii::indices_sum(&input),
                (14, 1) => xiv::resting_sand(&input, true),
                (14, 2) => xiv::resting_sand(&input, false),
                (15, 1) => xv::n_beaconless(&input),
                (15, 2) => xv::tuning_freq(&input),
                (16, 1) => xvi::max_release(&input),
                (17, 1) => xvii::tower_height(&input, 2022),
                (17, 2) => xvii::tower_height(&input, 1_000_000_000_000),
                (18, 1) => xviii::surface_area(&input, false),
                (18, 2) => xviii::surface_area(&input, true),
                (20, 1) => xx::sum_coords(&input),
                (21, 1) => xxi::root_monkey_nr(&input),
                (21, 2) => xxi::equality_satisfier(&input),
                (22, 1) => xxii::pathword(&input),
                (23, 1) => xxiii::game_of_elf(&input),
                (24, 1) => xxiv::fastest_path(&input),
                _ => unreachable!(),
            }
        );
    }

    Ok(())
}
