mod lib;
use lib::*;

use std::env;

use anyhow::Result;

fn main() -> Result<()> {
    let args = core::Argument::new(env::args().collect())?;
    let lines = core::read_lines(format!(
        "/home/dnk/Projects/advent-of-code/2022/rust/inputs/d_{0}",
        match args.day {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            _ => {
                println!("Day not implemented.");
                std::process::exit(1);
            }
        },
    ))?;

    println!(
        "Answer for day {}, part {}: {}",
        args.day,
        args.part,
        match (args.day, args.part) {
            (_, d) if d > 2 => {
                println!("A day only has two parts, got {}", args.part);
                std::process::exit(1);
            }
            (1, 1) => d_one::get_top_calories(lines, 1),
            (1, 2) => d_one::get_top_calories(lines, 3),
            (2, 1) => core::filter_map_then_sum(lines, d_two::count_score),
            (2, 2) => core::filter_map_then_sum(lines, d_two::infer_score),
            (3, 1) => d_three::sacks_sum(lines),
            (3, 2) => d_three::badges_sum(lines),
            (4, 1) => d_four::count(lines, d_four::includes_superset),
            (4, 2) => d_four::count(lines, d_four::includes_intersection),
            _ => unreachable!(), // args.day > n is already covered in first match expression
        }
    );

    Ok(())
}
