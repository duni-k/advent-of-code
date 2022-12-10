pub fn signal_strength(input: &str) -> isize {
    let mut register = 1;
    let mut cycle: isize = 0;
    let mut strength = 0;

    for mut ops in input
        .lines()
        .into_iter()
        .map(|line| line.split_whitespace())
    {
        cycle += 1;
        if (cycle - 20) % 40 == 0 {
            strength += cycle * register;
            dbg!((cycle, register));
        }
        match (ops.next(), ops.next()) {
            (Some("addx"), Some(n)) => {
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    strength += cycle * register;
                    dbg!((cycle, register));
                }
                register += n.parse::<isize>().unwrap();
            }
            (Some("noop"), None) => (),
            _ => unreachable!(),
        }
    }

    strength
}

pub fn letters_from_crt(input: &str) -> String {
    let mut register = 1;
    let mut cycle: isize = 0;
    let mut screen = vec!['.'; 240];

    for mut ops in input
        .lines()
        .into_iter()
        .map(|line| line.split_whitespace())
    {
        cycle += 1;
        if isize::abs_diff(cycle % 40, register) < 2 {
            screen[cycle as usize] = '#';
        }
        match (ops.next(), ops.next()) {
            (Some("addx"), Some(n)) => {
                cycle += 1;
                register += n.parse::<isize>().unwrap();
                if isize::abs_diff(cycle % 40, register) < 2 {
                    screen[cycle as usize] = '#';
                }
            }
            (Some("noop"), None) => (),
            _ => unreachable!(),
        }
        dbg!((&cycle, register));
    }

    format!(
        "\n{}",
        screen
            .chunks(40)
            .intersperse(&['\n'])
            .map(|chs| chs.into_iter().collect::<String>())
            .collect::<String>()
    )
}
