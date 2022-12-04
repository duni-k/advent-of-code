use std::{collections::HashSet, ops::RangeInclusive};

pub fn count_supersets(lines: impl Iterator<Item = String>) -> usize {
    let anyone_contains =
        |h1: &HashSet<_>, h2: &HashSet<_>| -> bool { h1.is_superset(h2) || h2.is_superset(h1) };
    count_with_func(lines, anyone_contains)
}

pub fn count_intersections(lines: impl Iterator<Item = String>) -> usize {
    let has_intersection =
        |h1: &HashSet<_>, h2: &HashSet<_>| -> bool { h1.intersection(h2).count() > 0 };
    count_with_func(lines, has_intersection)
}

fn count_with_func<F>(lines: impl Iterator<Item = String>, f: F) -> usize
where
    F: Fn(&HashSet<usize>, &HashSet<usize>) -> bool,
{
    let mut count = 0;
    for line in lines {
        let (sec1, sec2) = line.split_once(',').expect("Failed splitting pair.");
        let (r1, r2) = (parse_range(sec1), parse_range(sec2));
        let (h1, h2): (HashSet<usize>, HashSet<usize>) =
            (HashSet::from_iter(r1), HashSet::from_iter(r2));
        if f(&h1, &h2) {
            count += 1;
        }
    }
    count
}

fn parse_range(range: &str) -> RangeInclusive<usize> {
    let (lo, hi) = range.split_once('-').expect("Failed splitting range.");
    let lo = lo.parse::<usize>().expect("Failed parsing as number.");
    let hi = hi.parse::<usize>().expect("Failed parsing as number.");
    lo..=hi
}
