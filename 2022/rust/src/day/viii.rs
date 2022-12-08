pub fn visible_trees(lines: impl Iterator<Item = String>) -> usize {
    let heights: Vec<Vec<isize>> = lines
        .map(|line| {
            line.chars()
                .filter_map(|ch| {
                    if let Some(n) = ch.to_digit(10) {
                        Some(n as isize)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();
    let (n_rows, n_cols) = (heights.len(), heights[0].len());
    let mut visibility = vec![false; n_cols * n_rows];

    let idx = |x, y| y * n_cols + x;

    // left2right
    for y in 0..n_rows {
        let mut row_max = -1;
        for x in 0..n_cols {
            let height = heights[y][x];
            visibility[idx(x, y)] = visibility[idx(x, y)] || height > row_max;
            row_max = isize::max(height, row_max);
        }
    }
    // right2left
    for y in 0..n_rows {
        let mut tree_top = -1;
        for x in (0..n_cols).rev() {
            let height = heights[y][x];
            visibility[idx(x, y)] = visibility[idx(x, y)] || height > tree_top;
            tree_top = isize::max(height, tree_top);
        }
    }
    // topdown
    for x in 0..n_cols {
        let mut tree_top = -1;
        for y in 0..n_rows {
            let height = heights[y][x];
            visibility[idx(x, y)] = visibility[idx(x, y)] || height > tree_top;
            tree_top = isize::max(height, tree_top);
        }
    }
    println!();
    // downtop
    for x in 0..n_cols {
        let mut tree_top = isize::min_value();
        for y in (0..n_rows).rev() {
            let height = heights[y][x];
            visibility[idx(x, y)] = visibility[idx(x, y)] || height > tree_top;
            tree_top = isize::max(height, tree_top);
        }
    }

    visibility.iter().map(|&b| if b { 1 } else { 0 }).sum()
}

pub fn top_scenic_score(lines: impl Iterator<Item = String>) -> usize {
    let heights: Vec<Vec<isize>> = lines
        .map(|line| {
            line.chars()
                .filter_map(|ch| {
                    if let Some(n) = ch.to_digit(10) {
                        Some(n as isize)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();
    let (n_rows, n_cols) = (heights.len(), heights[0].len());

    let mut top_score = 0;
    for y in 0..n_rows {
        for x in 0..n_cols {
            let candidate = heights[y][x];
            // left2right
            let (mut lr, mut rl, mut td, mut dt) = (0, 0, 0, 0);
            for i in (x + 1)..n_cols {
                lr += 1;
                if !(candidate > heights[y][i]) {
                    break;
                }
            }
            // right2left
            for i in (0..x).rev() {
                rl += 1;
                if !(candidate > heights[y][i]) {
                    break;
                }
            }
            // topdown
            for i in (y + 1)..n_rows {
                td += 1;
                if !(candidate > heights[i][x]) {
                    break;
                }
            }
            // downtop
            for i in (0..y).rev() {
                dt += 1;
                if !(candidate > heights[i][x]) {
                    break;
                }
            }
            top_score = usize::max(rl * lr * td * dt, top_score);
        }
    }

    top_score
}
