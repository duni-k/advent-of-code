use core::fmt;
use std::{collections::HashMap, ops::Add};

use super::Point;

const CHAMBER_W: usize = 7;

impl Add<Direction> for Point {
    type Output = Self;
    fn add(self, rhs: Direction) -> Self::Output {
        use Direction::*;

        let (x, y) = match rhs {
            Left => (-1, 0),
            Right => (1, 0),
            Down => (0, -1),
        };

        Point {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '>' => Ok(Direction::Right),
            '<' => Ok(Direction::Left),
            _ => Err("Char not recognized."),
        }
    }
}

#[derive(PartialEq, Clone, Eq, Hash)]
struct TetrisBlock {
    pos: Vec<Point>,
}

impl Add<(isize, isize)> for TetrisBlock {
    type Output = Self;
    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Self::Output {
            pos: self
                .pos
                .iter()
                .map(|p| Point {
                    x: p.x + rhs.0,
                    y: p.y + rhs.1,
                })
                .collect(),
        }
    }
}

impl fmt::Display for TetrisBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let y_max = self.pos.iter().map(|p| p.y).max().unwrap();
        for y in 0..(y_max + 1) {
            for x in 0..CHAMBER_W {
                write!(
                    f,
                    "{}",
                    if self
                        .pos
                        .iter()
                        .any(|p| p.x == x as isize && p.y == y as isize)
                    {
                        '@'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl TetrisBlock {
    fn new(pos: &[(isize, isize)]) -> Self {
        Self {
            pos: pos.iter().map(|&(x, y)| Point { x, y }).collect(),
        }
    }

    fn fall(&mut self, tower: &Tower) -> bool {
        let new_pos: Vec<Point> = self.pos.iter().map(|&p| p + Direction::Down).collect();

        if new_pos
            .iter()
            .any(|p| ((p.y as usize) < tower.height && tower.blocks[p.y as usize][p.x as usize]))
        {
            false
        } else {
            self.pos = new_pos;
            true
        }
    }

    fn push(&mut self, dir: Direction, tower: &Tower) {
        let new_pos: Vec<Point> = self.pos.iter().map(|&p| p + dir).collect();

        if !new_pos.iter().any(|p| {
            p.x < 0
                || p.x as usize > (CHAMBER_W - 1)
                || ((p.x as usize) < CHAMBER_W
                    && (p.y as usize) < tower.height
                    && tower.blocks[p.y as usize][p.x as usize])
        }) {
            self.pos = new_pos;
        }
    }
}

#[derive(Clone)]
struct Tower {
    blocks: Vec<Vec<bool>>,
    height: usize,
}

impl Tower {
    fn new() -> Self {
        Tower {
            blocks: vec![vec![true; CHAMBER_W]; 1],
            height: 1,
        }
    }

    fn insert(&mut self, block: TetrisBlock) {
        let max = usize::max(
            self.height,
            (block.pos.iter().map(|p| p.y).max().unwrap() + 1) as usize,
        );

        for _ in 0..usize::abs_diff(max, self.height) {
            self.blocks.push(vec![false; CHAMBER_W]);
        }
        self.height = max;

        for p in &block.pos {
            self.blocks[p.y as usize][p.x as usize] = true;
        }
    }
}

impl fmt::Display for Tower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.blocks.iter() {
            write!(f, "|")?;
            for &block in row {
                write!(f, "{}", if block { '#' } else { '.' })?;
            }
            write!(f, "|\n")?;
        }
        writeln!(f, "+-------+")?;
        Ok(())
    }
}

pub fn tower_height(input: &str, runs: usize) -> isize {
    let proto_blocks = vec![
        TetrisBlock::new(&[(0, 0), (1, 0), (2, 0), (3, 0)]), // _
        TetrisBlock::new(&[(1, 0), (0, 1), (1, 2), (2, 1)]), // +
        TetrisBlock::new(&[(0, 0), (1, 0), (2, 2), (2, 1), (2, 0)]), // _|
        TetrisBlock::new(&[(0, 0), (0, 1), (0, 2), (0, 3)]), // |
        TetrisBlock::new(&[(0, 0), (1, 0), (0, 1), (1, 1)]), // #
    ];

    let mut tower = Tower::new();
    let mut jet = input
        .as_bytes()
        .iter()
        .filter_map(|&b| Direction::try_from(b as char).ok())
        .enumerate()
        .cycle();
    let mut mem: HashMap<(usize, usize), (usize, Tower)> = HashMap::new();
    let mut heights = Vec::new();
    let mut block_i = 0;
    for run in 0..runs {
        let mut rock = proto_blocks[block_i].clone() + (2, tower.blocks.len() as isize + 3);
        loop {
            let (stream_i, stream) = jet.next().unwrap();
            if mem.contains_key(&(stream_i, block_i)) {
                let (old_run, old_twr) = mem.get(&(stream_i, block_i)).unwrap();
                if old_twr.height > 50
                    && old_twr.blocks[old_twr.height - 50..] == tower.blocks[tower.height - 50..]
                {
                    let cycles_left = (runs - old_run) / (run - old_run) - 1; // subtract already processed cycle
                    let delta_height = tower.height - old_twr.height;
                    return (tower.height + cycles_left * delta_height - 1) as isize;
                }
            } else {
                mem.insert((stream_i, block_i), (run, tower.clone()));
            }
            rock.push(stream, &tower);
            if !rock.fall(&tower) {
                tower.insert(rock.clone());
                break;
            }
        }
        heights.push(tower.height - 1);
        block_i = (block_i + 1) % proto_blocks.len();
    }

    tower.height as isize - 1 // ground floor is virtual
}
