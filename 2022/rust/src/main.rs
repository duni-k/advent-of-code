mod lib;
use lib::*;

use std::env;

use anyhow::Result;

fn main() -> Result<()> {
    let args = lib::Argument::new(env::args().collect())?;
    let lines = lib::read_lines(args.day)?;

    let answer: String = match (args.day, args.part) {
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
        _ => unreachable!(), // args.day > n is already covered in first match expression
    };

    println!(
        "Answer for day {}, part {}: {}",
        args.day, args.part, answer
    );
    Ok(())
}
