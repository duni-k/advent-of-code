use take_until::TakeUntilExt;

pub fn visible_trees(input: &str) -> usize {
    let heights = construct_matrix(input);
    let (n_rows, n_cols) = (heights.len(), heights[0].len());
    let mut visibility = vec![false; n_cols * n_rows];

    let idx = |x, y| y * n_cols + x;

    // left2right
    for y in 0..n_rows {
        let mut row_max = -1;
        for x in 0..n_cols {
            let height = heights[y][x];
            visibility[idx(x, y)] |= height > row_max;
            row_max = isize::max(height, row_max);
        }
    }
    // right2left
    for y in 0..n_rows {
        let mut tree_top = -1;
        for x in (0..n_cols).rev() {
            let height = heights[y][x];
            visibility[idx(x, y)] |= height > tree_top;
            tree_top = isize::max(height, tree_top);
        }
    }
    // topdown
    for x in 0..n_cols {
        let mut tree_top = -1;
        for y in 0..n_rows {
            let height = heights[y][x];
            visibility[idx(x, y)] |= height > tree_top;
            tree_top = isize::max(height, tree_top);
        }
    }
    println!();
    // downtop
    for x in 0..n_cols {
        let mut tree_top = isize::min_value();
        for y in (0..n_rows).rev() {
            let height = heights[y][x];
            visibility[idx(x, y)] |= height > tree_top;
            tree_top = isize::max(height, tree_top);
        }
    }

    visibility.iter().map(|&b| if b { 1 } else { 0 }).sum()
}

pub fn top_scenic_score(input: &str) -> usize {
    let heights = construct_matrix(input);
    let (n_rows, n_cols) = (heights.len(), heights[0].len());

    let mut top_score = 0;
    for y in 0..n_rows {
        for x in 0..n_cols {
            top_score = usize::max(top_score, scenic_score(x, y, &heights));
        }
    }

    top_score
}

fn construct_matrix(input: &str) -> Vec<Vec<isize>> {
    input
        .split('\n')
        .map(|line| {
            line.chars()
                .filter_map(|ch| ch.to_digit(10).map(|n| n as isize))
                .collect()
        })
        .collect()
}

fn scenic_score(x: usize, y: usize, heights: &Vec<Vec<isize>>) -> usize {
    let candidate = heights[y][x];
    let (n_rows, n_cols) = (heights.len(), heights[0].len());
    // left2right
    let rl = (0..x)
        .rev()
        .take_until(|&i| candidate <= heights[y][i])
        .count();
    // right2left
    let lr = ((x + 1)..n_cols)
        .take_until(|&i| candidate <= heights[y][i])
        .count();

    // topdown
    let td = ((y + 1)..n_rows)
        .take_until(|&i| candidate <= heights[i][x])
        .count();

    // downtop
    let dt = (0..y)
        .rev()
        .take_until(|&i| candidate <= heights[i][x])
        .count();

    rl * lr * td * dt
}
