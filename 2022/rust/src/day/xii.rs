use std::collections::{HashSet, VecDeque};

pub fn shortest_path_for(input: &str, start: char) -> usize {
    let map = input
        .lines()
        .map(|line| line.to_string().chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let starting = find_starting(&map, start);

    starting
        .iter()
        .filter_map(|&start| {
            let mut queue = VecDeque::new();
            queue.push_front((0, start));
            shortest_path(&map, queue)
        })
        .reduce(usize::min)
        .unwrap()
}

fn shortest_path(map: &[Vec<char>], mut queue: VecDeque<(usize, (isize, isize))>) -> Option<usize> {
    const END: char = 'E';
    const ADJ: [(isize, isize); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

    let mut visited = HashSet::new();
    while let Some((length, (x, y))) = queue.pop_front() {
        let cell = map[y as usize][x as usize];
        if cell == END {
            return Some(length);
        }
        for (x_adj, y_adj) in ADJ {
            let new_pt = (x + x_adj, y + y_adj);
            if !visited.contains(&new_pt) && allowed(&map, (x, y), new_pt) {
                visited.insert(new_pt);
                queue.push_back((length + 1, new_pt));
            }
        }
    }
    None
}

fn find_starting(map: &[Vec<char>], start: char) -> Vec<(isize, isize)> {
    let mut positions = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == start {
                positions.push((x as isize, y as isize));
            }
        }
    }
    positions
}

fn allowed(map: &[Vec<char>], (x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> bool {
    let x_bound = map[0].len() as isize;
    let y_bound = map.len() as isize;
    if !(x2 >= 0 && x2 < x_bound && y2 >= 0 && y2 < y_bound) {
        return false;
    }

    let this = map[y1 as usize][x1 as usize];
    let other = map[y2 as usize][x2 as usize];
    other as u8 <= (this as u8) + 1 || this == 'S' || (this == 'z' && other == 'E')
}
