pub fn count_supersets(lines: impl Iterator<Item = String>) -> usize {
    count_with_func(lines, |(a, b), (c, d)| {
        (a >= c && b <= d) || (c >= a && d <= b)
    })
}

pub fn count_intersections(lines: impl Iterator<Item = String>) -> usize {
    count_with_func(lines, has_intersection)
}

fn has_intersection((lo1, hi1): &(usize, usize), (lo2, hi2): &(usize, usize)) -> bool {
    (lo1 >= lo2 && lo1 <= hi2)
        || (hi1 >= lo2 && hi1 <= hi2)
        || (lo2 >= lo1 && lo2 <= hi1)
        || (hi2 >= lo1 && hi2 <= hi1)
}

fn count_with_func<F>(lines: impl Iterator<Item = String>, f: F) -> usize
where
    F: Fn(&(usize, usize), &(usize, usize)) -> bool,
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
