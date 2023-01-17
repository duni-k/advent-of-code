use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "R" => Self::Right,
            "L" => Self::Left,
            "D" => Self::Down,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Rope {
    body: Vec<Knot>,
}

impl Rope {
    fn new(knots: usize) -> Self {
        Self {
            body: vec![Knot::new(); knots],
        }
    }

    fn tail(&self) -> &Knot {
        &self.body[self.body.len() - 1]
    }

    fn len(&self) -> usize {
        self.body.len()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Knot {
    x: isize,
    y: isize,
}

impl Knot {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn step_dir(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        };
    }

    fn step_towards(&mut self, other: &Self) {
        use std::cmp::Ordering::*;
        match self.x.cmp(&other.x) {
            Greater => self.x += 1,
            Less => self.x -= 1,
            _ => (),
        }
        match self.y.cmp(&other.y) {
            Greater => self.y += 1,
            Less => self.y -= 1,
            _ => (),
        }
    }

    fn touching(&self, other: &Self) -> bool {
        isize::abs_diff(self.x, other.x) < 2 && isize::abs_diff(self.y, other.y) < 2
    }

    fn pos(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

pub fn visited_by_tail(input: &str, knots: usize) -> isize {
    let mut rope = Rope::new(knots);
    let mut visited = HashSet::new();
    visited.insert(rope.tail().pos());
    for (direction, distance) in input.lines().map(|line| line.split_once(' ').unwrap()) {
        for _ in 0..(distance.parse::<usize>().unwrap()) {
            rope.body[0].step_dir(&Direction::from(direction));
            for (i, j) in (0..rope.len() - 1).zip(1..rope.len()) {
                let (this, mut next) = (rope.body[i], rope.body[j]);
                if !this.touching(&next) {
                    next.step_towards(&this)
                }
                rope.body[j] = next;
            }
            visited.insert(rope.tail().pos());
        }
    }

    visited.len() as isize
}
