use std::collections::VecDeque;

pub fn surface_area(input: &str, count_trapped: bool) -> isize {
    let points: Vec<(usize, usize, usize)> = input
        .lines()
        .map(|line| {
            let coords = line
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (coords[0] + 1, coords[1] + 1, coords[2] + 1) // need edges around the matrix
        })
        .collect();

    let (x_bound, y_bound, z_bound) = points
        .clone()
        .into_iter()
        .reduce(|(x_max, y_max, z_max), (x, y, z)| {
            (
                usize::max(x_max, x),
                usize::max(y_max, y),
                usize::max(z_max, z),
            )
        })
        .unwrap();

    let mut m: Vec<Vec<Vec<_>>> = vec![vec![vec![false; z_bound + 2]; y_bound + 2]; x_bound + 2];
    for &(x, y, z) in &points {
        m[x][y][z] = true;
    }

    let mut exposed = 0;
    for p in &points {
        for (x, y, z) in neighbors(p, &m) {
            if !m[x][y][z] && (!count_trapped || !reaches_edge(&(x, y, z), &m)) {
                exposed += 1
            }
        }
    }

    exposed as isize
}

fn reaches_edge(&p: &(usize, usize, usize), m: &[Vec<Vec<bool>>]) -> bool {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![vec![false; m[0][0].len()]; m[0].len()]; m.len()];
    queue.push_back(p);
    while let Some(p) = queue.pop_front() {
        visited[p.0][p.1][p.2] = true;
        if p.0 == 0
            || p.0 == m.len()
            || p.1 == 0
            || p.1 == m[0].len()
            || p.2 == 0
            || p.2 == m[0][0].len()
        {
            return true;
        }

        for (x, y, z) in neighbors(&p, &m) {
            if !m[x][y][z] && !visited[x][y][z] {
                queue.push_front((x, y, z));
            }
        }
    }

    false
}

fn neighbors(p: &(usize, usize, usize), m: &[Vec<Vec<bool>>]) -> Vec<(usize, usize, usize)> {
    const OFFSETS: [(isize, isize, isize); 6] = [
        (0, 0, 1),
        (0, 1, 0),
        (1, 0, 0),
        (0, 0, -1),
        (0, -1, 0),
        (-1, 0, 0),
    ];

    OFFSETS
        .iter()
        .filter_map(|off| neighbor(p, off, &m))
        .collect()
}

fn neighbor(
    &(x, y, z): &(usize, usize, usize),
    &(x_off, y_off, z_off): &(isize, isize, isize),
    m: &[Vec<Vec<bool>>],
) -> Option<(usize, usize, usize)> {
    let (x_adj, y_adj, z_adj) = (x as isize + x_off, y as isize + y_off, z as isize + z_off);
    if (x_adj >= 0)
        && (y_adj >= 0)
        && (z_adj >= 0)
        && (x_adj < m.len() as isize)
        && (y_adj < m[0].len() as isize)
        && (z_adj < m[0][0].len() as isize)
    {
        Some((x_adj as usize, y_adj as usize, z_adj as usize))
    } else {
        None
    }
}
