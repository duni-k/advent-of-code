use std::collections::{HashSet, VecDeque};

pub fn shortest_path_from(input: &str, start: char) -> usize {
    let map = input
        .lines()
        .map(|line| line.to_string().chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    find_starting(&map, start)
        .iter()
        .filter_map(|&start| shortest_path(&map, start))
        .reduce(usize::min)
        .unwrap()
}

fn shortest_path(map: &[Vec<char>], start: (usize, usize)) -> Option<usize> {
    const END: char = 'E';
    const ADJ: [(isize, isize); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_front((0, start));
    while let Some((length, (x, y))) = queue.pop_front() {
        let cell = map[y][x];
        if cell == END {
            return Some(length);
        }
        for (x_adj, y_adj) in ADJ {
            let (new_x, new_y) = (x as isize + x_adj, y as isize + y_adj);
            if new_x < 0 || new_y < 0 {
                continue;
            }
            let new_pt = (new_x as usize, new_y as usize);
            if !visited.contains(&new_pt) && allowed(map, (x, y), new_pt) {
                visited.insert(new_pt);
                queue.push_back((length + 1, new_pt));
            }
        }
    }
    None
}

fn find_starting(map: &[Vec<char>], start: char) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == start {
                positions.push((x, y));
            }
        }
    }
    positions
}

fn allowed(map: &[Vec<char>], (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> bool {
    let x_bound = map[0].len();
    let y_bound = map.len();
    if x2 >= x_bound || y2 >= y_bound {
        return false;
    }

    let this = map[y1][x1];
    let other = map[y2][x2];
    other as u8 <= (this as u8) + 1 || this == 'S' || (this == 'z' && other == 'E')
}
