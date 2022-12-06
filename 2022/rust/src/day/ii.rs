type Score = usize;

#[derive(PartialEq, Eq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(&self) -> Score {
        use Outcome::*;
        match self {
            Win => 6,
            Loss => 0,
            Draw => 3,
        }
    }
}

impl From<char> for Outcome {
    fn from(s: char) -> Self {
        match s {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!(),
        }
    }
}

impl From<(Move, Move)> for Outcome {
    fn from((this, other): (Move, Move)) -> Self {
        use Move::*;
        match (this, other) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Outcome::Win,
            (this, other) if this == other => Outcome::Draw,
            _ => Outcome::Loss,
        }
    }
}

impl From<char> for Move {
    fn from(s: char) -> Self {
        match s {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

impl Move {
    fn beats(&self) -> Self {
        use Move::*;
        match self {
            Rock => Scissors,
            Scissors => Paper,
            Paper => Rock,
        }
    }

    fn loses_to(&self) -> Self {
        use Move::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn score(&self) -> Score {
        use Move::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn move_from(&self, oc: &Outcome) -> Self {
        match (self, oc) {
            (mv, Outcome::Draw) => mv.clone(),
            (mv, Outcome::Loss) => mv.beats(),
            (mv, Outcome::Win) => mv.loses_to(),
        }
    }
}

pub fn count_score(line: &str) -> Option<usize> {
    let mut moves = line.chars().filter_map(|ch| {
        if ch.is_alphabetic() {
            Some(Move::from(ch))
        } else {
            None
        }
    });
    if let (Some(opp_mv), Some(my_mv)) = (moves.next(), moves.next()) {
        Some(my_mv.score() + Outcome::from((my_mv, opp_mv)).score())
    } else {
        None
    }
}

pub fn infer_score(line: &str) -> Option<Score> {
    let mut line = line.chars().filter(|ch| ch.is_alphabetic());
    if let (Some(fst), Some(snd)) = (line.next(), line.next()) {
        let opp_mv = Move::from(fst);
        let outcome = Outcome::from(snd);
        Some(opp_mv.move_from(&outcome).score() + outcome.score())
    } else {
        None
    }
}

pub fn filter_map_then_sum<L, F>(lines: L, f: F) -> String
where
    F: Fn(&'_ str) -> Option<usize>,
    L: Iterator<Item = String>,
{
    lines.filter_map(|line| f(&line)).sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infer_correctly() {
        let games = vec![("A Y", 4), ("B X", 1), ("C Z", 7)];
        for g in games {
            assert_eq!(infer_score(&String::from(g.0)).unwrap(), g.1);
        }
    }
}
