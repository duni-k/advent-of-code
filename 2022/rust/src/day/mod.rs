pub mod i;
pub mod ii;
pub mod iii;
pub mod iv;
pub mod ix;
pub mod v;
pub mod vi;
pub mod vii;
pub mod viii;
pub mod x;
pub mod xi;
pub mod xii;
pub mod xiii;
pub mod xiv;
pub mod xv;
pub mod xvi;
pub mod xvii;
pub mod xviii;
pub mod xx;
pub mod xxi;
pub mod xxii;
pub mod xxiii;
pub mod xxiv;

pub fn get_numbers(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .filter_map(|n| n.strip_suffix(',').unwrap_or(n).parse::<usize>().ok())
        .collect()
}

pub fn get_number(input: &str) -> usize {
    input
        .split_whitespace()
        .find_map(|n| n.parse::<usize>().ok())
        .unwrap()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        use Direction::*;
        match ch {
            '>' => Ok(East),
            '<' => Ok(West),
            '^' => Ok(North),
            'v' => Ok(South),
            _ => Err("Could not  recognize direction of char"),
        }
    }
}

impl Into<char> for Direction {
    fn into(self) -> char {
        use Direction::*;
        match self {
            North => '^',
            South => 'v',
            East => '>',
            West => '<',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(&self, other: &Self) -> usize {
        (isize::abs_diff(self.x, other.x) + isize::abs_diff(self.y, other.y)) as usize
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl From<Direction> for Point {
    fn from(dir: Direction) -> Point {
        use Direction::*;
        match dir {
            West => Point::new(-1, 0),
            South => Point::new(0, 1),
            East => Point::new(1, 0),
            North => Point::new(0, -1),
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point::new(x as isize, y as isize)
    }
}
