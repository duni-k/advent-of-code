use std::collections::{HashSet, VecDeque};

use anyhow::{anyhow, Result};

pub fn start_marker_position<L>(mut lines: L, seq_len: usize) -> Result<usize>
where
    L: Iterator<Item = String>,
{
    let input = lines.next().unwrap();
    let mut last_four = VecDeque::new();
    for (i, ch) in input.chars().enumerate() {
        if last_four.len() >= seq_len {
            last_four.pop_front();
        }
        last_four.push_back(ch);
        let unique: HashSet<&char> = HashSet::from_iter(&last_four);
        if unique.len() == seq_len {
            return Ok(i + 1);
        }
    }
    Err(anyhow!("Input expected to include start-of-packet marker"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_start_marker_position() {
        let input = vec![
            String::from("nppdvjthqldpwncqszvftbrmjlhg"),
            String::from(""),
        ]
        .into_iter();
        assert_eq!(start_marker_position(input, 4).unwrap(), 6);
        let input = vec![
            String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            String::from(""),
        ]
        .into_iter();
        assert_eq!(start_marker_position(input, 14).unwrap(), 6);
    }
}
