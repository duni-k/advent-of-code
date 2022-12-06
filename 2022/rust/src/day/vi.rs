use itertools::Itertools;

pub fn start_marker_position<L>(mut lines: L, seq_len: usize) -> usize
where
    L: Iterator<Item = String>,
{
    lines
        .next()
        .unwrap()
        .as_bytes()
        .windows(seq_len)
        .position(|w| w.iter().unique().count() == seq_len)
        .unwrap()
        + seq_len
}
