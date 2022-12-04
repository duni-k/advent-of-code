type Range = (usize, usize);

pub fn includes_superset((lo1, hi1): &Range, (lo2, hi2): &Range) -> bool {
    (lo1 >= lo2 && hi1 <= hi2) || (lo2 >= lo1 && hi2 <= hi1)
}

pub fn includes_intersection((lo1, hi1): &Range, (lo2, hi2): &Range) -> bool {
    (lo1 >= lo2 && lo1 <= hi2)
        || (hi1 >= lo2 && hi1 <= hi2)
        || (lo2 >= lo1 && lo2 <= hi1)
        || (hi2 >= lo1 && hi2 <= hi1)
}

pub fn count<F, L>(lines: L, f: F) -> usize
where
    L: Iterator<Item = String>,
    F: Fn(&Range, &Range) -> bool,
{
    lines
        .map(|line| {
            let (sec1, sec2) = line.split_once(',').expect("Failed splitting pair.");
            let (r1, r2) = (parse_range(sec1), parse_range(sec2));
            if f(&r1, &r2) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn parse_range(range: &str) -> (usize, usize) {
    let (lo, hi) = range.split_once('-').expect("Failed splitting range.");
    let lo = lo.parse::<usize>().expect("Failed parsing as number.");
    let hi = hi.parse::<usize>().expect("Failed parsing as number.");
    (lo, hi)
}
