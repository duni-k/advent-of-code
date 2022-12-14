use std::collections::{HashSet, VecDeque};

use super::{Direction, Point};

// Pretty sure i need to zobrist hash the states and check for visited states
// before appending to the BFS queue (otherwise it's too slow)
pub fn fastest_path(input: &str) -> isize {
    let map: Vec<Vec<Vec<char>>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    if ch != '.' || ch != '#' {
                        vec!['.', ch]
                    } else {
                        vec![ch]
                    }
                })
                .collect::<Vec<Vec<_>>>()
        })
        .collect();

    let mut blizzard_positions = HashSet::new();
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.len() > 1 {
                blizzard_positions.insert(Point::new(x as isize, y as isize));
            }
        }
    }

    let (x_start, y_start): (usize, usize) = (1, 0);
    let end = Point::new((map[0].len() - 2) as isize, (map.len() - 1) as isize);

    let mut queue = VecDeque::new();
    let mut minute = 0;
    // every minute will have one known map so we use a precomputed one at
    // each node of the BFS tree
    let mut storms = vec![(blizzard_positions, map)];
    queue.push_back((0, Point::new(x_start as isize, y_start as isize)));

    while let Some((curr_min, pt)) = queue.pop_front() {
        minute = curr_min;
        dbg!(minute);
        if pt == end {
            break;
        }
        if minute >= storms.len() {
            // check if map has been computed
            let (prev_blizz, prev_map) = &storms[storms.len() - 1];
            storms.push(storm(prev_blizz, &prev_map));
        }
        let curr_map = &storms[minute].1;

        for neighbour in neighbors(&curr_map, &pt) {
            queue.push_back((minute + 1, neighbour));
        }
        queue.push_back((minute + 1, pt)); // we can opt to stand still
    }

    minute as isize
}

fn storm(
    blizz_pos: &HashSet<Point>,
    map: &[Vec<Vec<char>>],
) -> (HashSet<Point>, Vec<Vec<Vec<char>>>) {
    let (width, height) = (map[0].len() - 1, map.len() - 1);
    let mut next_blizz = HashSet::new();
    let mut next_map = copy_map(&map);
    for pt in blizz_pos.iter() {
        for dir in map[pt.y as usize][pt.x as usize]
            .iter()
            .filter_map(|&ch| Direction::try_from(ch).ok())
        {
            let mut new_pt = *pt + Point::from(dir);
            maybe_wrap(&mut new_pt, width, height);
            next_map[new_pt.y as usize][new_pt.x as usize].push(dir.into());
            next_blizz.insert(new_pt);
        }
    }

    (next_blizz, next_map.to_vec())
}

fn copy_map(map: &[Vec<Vec<char>>]) -> Vec<Vec<Vec<char>>> {
    let mut new_map = vec![vec![vec!['?']; map[0].len()]; map.len()];
    for (y, row) in map.iter().enumerate() {
        for (x, v) in row
            .iter()
            .map(|v| {
                v.iter()
                    .filter_map(|&ch| {
                        if ch == '#' || ch == '.' {
                            Some(ch)
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .enumerate()
        {
            new_map[y][x] = v;
        }
    }

    new_map
}

fn maybe_wrap(point: &mut Point, width: usize, height: usize) {
    if start_or_end(&point, width, height) {
        dbg!(&point);
        return;
    }
    let (width, height) = (width as isize, height as isize);
    if point.y < 1 {
        point.y = height - 2;
    } else if point.y >= height {
        point.y = 1;
    }
    if point.x < 1 {
        point.x = width - 2;
    } else if point.x >= width {
        point.x = 1;
    }
}

fn start_or_end(pt: &Point, width: usize, height: usize) -> bool {
    (pt.y == 0 && pt.x == 1) || (pt.y == height as isize && pt.x == width as isize - 1)
}

fn neighbors(map: &[Vec<Vec<char>>], &point: &Point) -> Vec<Point> {
    use Direction::*;
    let mut neighbors = vec![
        point + Point::from(North),
        point + Point::from(South),
        point + Point::from(West),
        point + Point::from(East),
    ];

    let (width, height) = (map[0].len(), map.len());
    neighbors.retain(|&p| {
        ((p.x >= 0 && p.x < (width as isize - 1) && p.y >= 0 && p.y < (height as isize - 1))
            || start_or_end(&p, width, height))
            && !map[p.y as usize][p.x as usize] // ok because checked above
                .iter()
                .any(|&ch| Direction::try_from(ch).is_ok() || ch == '#')
    });

    neighbors
}
