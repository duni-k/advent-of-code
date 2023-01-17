use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn line_between(self, other: Self) -> HashSet<Self> {
        let mut line = HashSet::new();
        line.insert(self);
        line.insert(other);
        if self.x != other.x {
            for x in usize::min(self.x, other.x)..usize::max(self.x, other.x) {
                line.insert(Point { x, y: self.y });
            }
        } else {
            for y in usize::min(self.y, other.y)..usize::max(self.y, other.y) {
                line.insert(Point { x: self.x, y });
            }
        }

        line
    }

    fn falling(&mut self, rocks: &HashSet<Self>) -> bool {
        if !rocks.contains(&Point {
            x: self.x,
            y: self.y + 1,
        }) {
            self.y += 1;
        } else if !rocks.contains(&Point {
            x: self.x - 1,
            y: self.y + 1,
        }) {
            self.x -= 1;
        } else if !rocks.contains(&Point {
            x: self.x + 1,
            y: self.y + 1,
        }) {
            self.x += 1;
        } else {
            return false;
        }
        true
    }
}

pub fn resting_sand(input: &str, void: bool) -> isize {
    let mut rocks = get_rocks(input);
    let start_len = rocks.len();
    let (_, max) = rocks_min_max(&rocks);
    print_rocks(&rocks);

    const START: Point = Point { x: 500, y: 0 };

    'simulation: loop {
        let mut grain = START;

        while grain.falling(&rocks) {
            if void && grain.y > max.y {
                break 'simulation;
            } else if !void && grain.y == max.y + 1 {
                break;
            }
        }

        rocks.insert(grain);

        if grain == START {
            break;
        }
    }
    print_rocks(&rocks);

    (rocks.len() - start_len) as isize
}

fn get_rocks(input: &str) -> HashSet<Point> {
    input
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .filter_map(|point| {
                    if let Some((x, y)) = point.split_once(',') {
                        Some(Point {
                            x: x.parse().unwrap(),
                            y: y.parse().unwrap(),
                        })
                    } else {
                        None
                    }
                })
                .tuple_windows()
                .flat_map(|(this, next)| this.line_between(next))
        })
        .collect()
}

fn rocks_min_max(rocks: &HashSet<Point>) -> (Point, Point) {
    let x_min = rocks.iter().map(|p| p.x).min().unwrap();
    let x_max = rocks.iter().map(|p| p.x).max().unwrap();
    let y_min = rocks.iter().map(|p| p.y).min().unwrap();
    let y_max = rocks.iter().map(|p| p.y).max().unwrap();

    (Point { x: x_min, y: y_min }, Point { x: x_max, y: y_max })
}

fn print_rocks(rocks: &HashSet<Point>) {
    let (min, max) = rocks_min_max(rocks);

    for y in 0..(max.y + 1) {
        for x in (min.x - 1)..(max.x + 1) {
            if (x, y) == (500, 0) {
                print!("+");
            } else if rocks.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
