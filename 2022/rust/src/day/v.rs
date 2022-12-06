use std::collections::HashMap;

use anyhow::{anyhow, Result};
use itertools::Itertools;

type Crate = char;

#[derive(Debug)]
struct Instruction {
    amount: usize,
    src: usize,
    dest: usize,
}

pub fn stack_tops<L>(lines: L, crate_mover_model: usize) -> Result<String>
where
    L: Iterator<Item = String>,
{
    let (mut crate_stacks, instructions) = init_cfg(lines);

    match crate_mover_model {
        9000 => {
            for instr in instructions {
                for _ in 0..instr.amount {
                    let to_move = crate_stacks[instr.src].pop().unwrap();
                    crate_stacks[instr.dest].push(to_move);
                }
            }
        }
        9001 => {
            for instr in instructions {
                let split_idx = crate_stacks[instr.src].len() - instr.amount;
                let mut to_move = crate_stacks[instr.src].split_off(split_idx);
                crate_stacks[instr.dest].append(&mut to_move);
            }
        }
        _ => return Err(anyhow!("Model unknown.")),
    }

    Ok(crate_stacks
        .iter()
        .flat_map(|stack| stack.iter().last())
        .collect())
}

fn init_cfg<L>(mut lines: L) -> (Vec<Vec<Crate>>, Vec<Instruction>)
where
    L: Iterator<Item = String>,
{
    let mut crate_map = HashMap::new();
    'crates: for line in &mut lines {
        let mut col_idx = 0;
        for (i, ch) in line.chars().enumerate() {
            if ch.to_digit(10).is_some() {
                break 'crates;
            } else if ch.is_alphabetic() {
                crate_map.entry(col_idx - 1).or_insert(Vec::new()).push(ch);
            }
            if i % 4 == 0 {
                col_idx += 1;
            }
        }
    }
    let mut crate_stacks: Vec<Vec<_>> = vec![Vec::new(); crate_map.len()];
    for (i, v) in crate_map {
        crate_stacks[i] = v.into_iter().rev().collect();
    }

    let instructions = lines
        .filter_map(|line| {
            if let Some((amount, src, dest)) = line
                .split_ascii_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple::<(usize, usize, usize)>()
            {
                Some(Instruction {
                    amount,
                    src: src - 1,
                    dest: dest - 1,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    (crate_stacks, instructions)
}
