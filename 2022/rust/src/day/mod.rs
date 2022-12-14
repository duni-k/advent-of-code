pub mod i;
pub mod ii;
pub mod iii;
pub mod iv;
pub mod ix;
pub mod v;
pub mod vi;
pub mod vii;
pub mod viii;
pub mod x;
pub mod xi;
pub mod xii;
pub mod xiii;
pub mod xiv;

pub fn get_numbers(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .filter_map(|n| n.strip_suffix(',').unwrap_or(n).parse::<usize>().ok())
        .collect()
}

pub fn get_number(input: &str) -> usize {
    input
        .split_whitespace()
        .find_map(|n| n.parse::<usize>().ok())
        .unwrap()
}
