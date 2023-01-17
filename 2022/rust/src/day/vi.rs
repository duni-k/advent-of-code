use itertools::Itertools;

pub fn start_marker_position(input: &str, seq_len: usize) -> isize {
    (input
        .lines()
        .next()
        .unwrap()
        .as_bytes()
        .windows(seq_len)
        .position(|w| w.iter().unique().count() == seq_len)
        .unwrap()
        + seq_len) as isize
}
