use std::collections::HashSet;

use itertools::Itertools;

type Priority = usize;
type ItemType = char;

pub fn sacks_sum(input: &str) -> Priority {
    input
        .lines()
        .map(|line| {
            let (fst, snd) = line.split_at(line.len() / 2);
            priority(duplicate(vec![fst, snd]).unwrap())
        })
        .sum()
}

pub fn badges_sum(input: &str) -> Priority {
    input
        .lines()
        .tuples()
        .map(|(fst, snd, third)| priority(duplicate(vec![fst, snd, third]).unwrap()))
        .sum()
}

fn duplicate(group: Vec<&str>) -> Option<ItemType> {
    group
        .iter()
        .map(|&s| s.chars().collect::<HashSet<ItemType>>())
        .reduce(|acc, group| acc.intersection(&group).cloned().collect())
        .unwrap()
        .iter()
        .next()
        .cloned()
}

fn priority(it: ItemType) -> Priority {
    let offset = match it {
        'a'..='z' => 96,
        'A'..='Z' => 38,
        _ => unreachable!(),
    };
    (it as usize) - offset
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_duplicate_single() {
        let rucksacks = vec![
            (vec!["vJrwpWtwJgWr", "hcsFMMfFFhFp"], 'p'),
            (vec!["jqHRNqRjqzjGDLG", "LrsFMfFZSrLrFZsSL"], 'L'),
            (vec!["PmmdzqPrV", "vPwwTWBwg"], 'P'),
            (vec!["ttgJtRGJ", "QctTZtZT"], 't'),
        ];
        for (v, dupe) in rucksacks {
            assert_eq!(duplicate(v), Some(dupe));
        }
    }

    #[test]
    fn finds_duplicate_mult() {
        let group = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];
        assert_eq!(duplicate(group), Some('r'));
    }

    #[test]
    fn priority_correct() {
        for (ch, prio) in ('a'..='z').zip(1..=26).chain(('A'..='Z').zip(27..=52)) {
            assert_eq!(priority(ch), prio);
        }
    }
}
