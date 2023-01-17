use regex::Regex;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&self, d: Self) -> Self {
        use Direction::*;
        match (self, d) {
            (Up, Right) => Right,
            (Up, Left) => Left,
            (Down, Right) => Left,
            (Down, Left) => Right,
            (Right, Right) => Down,
            (Right, Left) => Up,
            (Left, Right) => Up,
            (Left, Left) => Down,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, ()> {
        use Direction::*;
        match s {
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Move {
    Traversal(usize),
    Rotation(Direction),
}

impl TryFrom<&str> for Move {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, ()> {
        use Move::*;
        if let Ok(n) = s.parse::<usize>() {
            Ok(Traversal(n))
        } else {
            Ok(Rotation(Direction::try_from(s).unwrap()))
        }
    }
}

pub fn pathword(input: &str) -> isize {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let x_max = map.iter().map(|row| row.len()).max().unwrap();
    let map = map
        .iter_mut()
        .map(|row| {
            row.append(&mut vec![' '; x_max - row.len()]);
            row.clone()
        })
        .collect::<Vec<_>>();

    print!(" ");
    for x in 0..map[0].len() {
        print!("{:2}", x);
    }
    println!();
    for (i, row) in map.iter().enumerate() {
        println!("{i}{}", row.iter().intersperse(&' ').collect::<String>());
    }

    let instruction_re = Regex::new(r"(\d+)|(\w)").unwrap();
    let path = instruction_re
        .captures_iter(input.lines().last().unwrap())
        .map(|cap| Move::try_from(&cap[0]).unwrap());

    let mut pos = position_init(&map).unwrap();
    let mut dir = Direction::Right;
    for instr in path {
        match instr {
            Move::Traversal(n) => {
                for _ in 0..n {
                    pos = move_in(&dir, pos, &map);
                }
            }
            Move::Rotation(d) => dir = dir.rotate(d),
        }
    }

    ((pos.1 + 1) * 1_000
        + 4 * (pos.0 + 1)
        + match dir {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }) as isize
}

fn position_init(map: &[Vec<char>]) -> Option<(usize, usize)> {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '.' {
                return Some((x, y));
            }
        }
    }

    None
}

fn move_in(dir: &Direction, old: (usize, usize), map: &[Vec<char>]) -> (usize, usize) {
    use Direction::*;

    let (height, width) = (map.len(), map[0].len());
    let (mut x, mut y) = (old.0 as isize, old.1 as isize);

    let (x, y) = match dir {
        Up => {
            y -= 1;
            if out_of_bounds((x, y), map) {
                let mut y = height - 1;
                while map[y][x as usize] == ' ' {
                    y -= 1;
                }
                (x as usize, y)
            } else {
                (x as usize, y as usize)
            }
        }
        Down => {
            y += 1;
            if out_of_bounds((x, y), map) {
                let mut y = 0;
                while map[y][x as usize] == ' ' {
                    y += 1;
                }
                (x as usize, y)
            } else {
                (x as usize, y as usize)
            }
        }
        Left => {
            x -= 1;
            if out_of_bounds((x, y), map) {
                let mut x = width - 1;
                while map[y as usize][x] == ' ' {
                    x -= 1;
                }
                (x, y as usize)
            } else {
                (x as usize, y as usize)
            }
        }
        Right => {
            x += 1;
            if out_of_bounds((x, y), map) {
                let mut x = 0;
                while map[y as usize][x] == ' ' {
                    x += 1;
                }
                (x, y as usize)
            } else {
                (x as usize, y as usize)
            }
        }
    };

    if map[y][x] == '#' {
        old
    } else {
        (x, y)
    }
}

fn out_of_bounds((x, y): (isize, isize), map: &[Vec<char>]) -> bool {
    if x < 0 || y < 0 {
        true
    } else {
        let (x, y) = (x as usize, y as usize);
        x >= map[0].len() || y >= map.len() || map[y][x] == ' '
    }
}
