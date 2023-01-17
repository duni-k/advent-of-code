use core::fmt;
use std::collections::{HashMap, HashSet, VecDeque};

use super::{Direction, Point};

#[derive(Debug)]
struct Game {
    elves: HashSet<Point>,
    direction_prio: VecDeque<Direction>,
}

impl Game {
    fn new(elves: HashSet<Point>) -> Self {
        use Direction::*;
        let direction_prio = VecDeque::from(vec![North, South, West, East]);

        Self {
            elves,
            direction_prio,
        }
    }

    fn consider_move(&self) -> HashMap<Point, Vec<Point>> {
        use Direction::*;

        let mut considerations = HashMap::new();
        let mut considered = HashSet::new();
        for &elf in &self.elves {
            let n = elf + Point::from(North);
            let s = elf + Point::from(South);
            let w = elf + Point::from(West);
            let e = elf + Point::from(East);

            let nw = w + Point::from(North);
            let sw = w + Point::from(South);
            let ne = e + Point::from(North);
            let se = e + Point::from(South);

            if !(self.elves.contains(&n)
                || self.elves.contains(&s)
                || self.elves.contains(&w)
                || self.elves.contains(&e)
                || self.elves.contains(&nw)
                || self.elves.contains(&sw)
                || self.elves.contains(&ne)
                || self.elves.contains(&se))
            {
                continue;
            }
            for dir in &self.direction_prio {
                if let Some(pt) = match dir {
                    North
                        if !(self.elves.contains(&n)
                            || self.elves.contains(&nw)
                            || self.elves.contains(&ne))
                            && !considered.contains(&elf) =>
                    {
                        Some(n)
                    }
                    South
                        if !(self.elves.contains(&s)
                            || self.elves.contains(&sw)
                            || self.elves.contains(&se))
                            && !considered.contains(&elf) =>
                    {
                        Some(s)
                    }
                    West if !(self.elves.contains(&w)
                        || self.elves.contains(&nw)
                        || self.elves.contains(&sw))
                        && !considered.contains(&elf) =>
                    {
                        Some(w)
                    }
                    East if !(self.elves.contains(&e)
                        || self.elves.contains(&ne)
                        || self.elves.contains(&se))
                        && !considered.contains(&elf) =>
                    {
                        Some(e)
                    }
                    _ => None,
                } {
                    considered.insert(elf);
                    considerations.entry(pt).or_insert(Vec::new()).push(elf);
                }
            }
        }

        considerations
    }

    fn actually_move(&mut self, considerations: HashMap<Point, Vec<Point>>) {
        for (pos, elves) in considerations {
            if elves.len() == 1 {
                self.elves.remove(&elves[0]);
                self.elves.insert(pos);
            }
        }
    }

    fn change_prio(&mut self) {
        let prev = self.direction_prio.pop_front().unwrap();
        self.direction_prio.push_back(prev);
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let y_max = self.elves.iter().map(|p| p.y).max().unwrap() + 2;
        let y_min = self.elves.iter().map(|p| p.y).min().unwrap() - 2;
        let x_max = self.elves.iter().map(|p| p.x).max().unwrap() + 2;
        let x_min = self.elves.iter().map(|p| p.x).min().unwrap() - 2;
        writeln!(
            f,
            "{}",
            String::from("-").repeat(isize::abs_diff(x_min, x_max) as usize)
        )?;
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                write!(
                    f,
                    "{}",
                    if self.elves.contains(&Point::new(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        writeln!(
            f,
            "{}",
            String::from("-").repeat(isize::abs_diff(x_min, x_max) as usize)
        )?;
        Ok(())
    }
}

pub fn game_of_elf(input: &str) -> isize {
    let mut game = Game::new(parse_elves(input));

    for i in 0..10 {
        game.actually_move(game.consider_move());
        game.change_prio();
    }

    let (x_min, x_max, y_min, y_max) = game.elves.iter().fold(
        (
            isize::max_value(),
            isize::min_value(),
            isize::max_value(),
            isize::min_value(),
        ),
        |(x_min, x_max, y_min, y_max), elf| {
            (
                isize::min(x_min, elf.x),
                isize::max(x_max, elf.x),
                isize::min(y_min, elf.y),
                isize::max(y_max, elf.y),
            )
        },
    );

    ((isize::abs_diff(x_min, x_max) + 1) * (isize::abs_diff(y_min, y_max) + 1) - game.elves.len())
        as isize
}

pub fn first_standstill(input: &str) -> usize {
    let mut game = Game::new(parse_elves(input));

    let mut round = 0;
    loop {
        round += 1;
        let considerations = game.consider_move();
        if considerations.iter().all(|(_, elves)| elves.len() > 1) {
            println!("{game}");
            return round;
        }
        game.actually_move(considerations);
        game.change_prio();
    }
}

fn parse_elves(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, ch)| {
                    if ch == '#' {
                        Some(Point::new(x as isize, y as isize))
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>()
        })
        .collect()
}
