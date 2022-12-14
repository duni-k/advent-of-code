use core::fmt;

enum Token {
    LPar,
    RPar,
    Integer(u32),
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::LPar => "(".to_string(),
                Self::RPar => ")".to_string(),
                Self::Integer(n) => n.to_string(),
            }
        )
    }
}

pub fn indices_sum(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, pair)| {
            if let Some((fst, snd)) = pair.split_once('\n') {
                let ord = ordered(lex(fst), lex(snd));
                // dbg!(((i + 1), ord));
                Some((i + 1) * ord as usize)
            } else {
                dbg!(&pair);
                None
            }
        })
        .sum()
}

fn lex(input: &str) -> Vec<Token> {
    use Token::*;
    input
        .to_string()
        .chars()
        .filter_map(|ch| {
            if ch == ',' || ch == '\n' {
                None
            } else {
                Some(match ch {
                    '[' => LPar,
                    ']' => RPar,
                    n => Integer(n.to_digit(10).unwrap()),
                })
            }
        })
        .rev()
        .collect()
}

fn ordered(mut left: Vec<Token>, mut right: Vec<Token>) -> bool {
    use Token::*;
    loop {
        // println!("left: {:?}", &left.iter().rev().collect::<Vec<_>>());
        // println!("right: {:?}", &right.iter().rev().collect::<Vec<_>>());
        match (left.pop(), right.pop()) {
            (Some(Integer(n1)), Some(Integer(n2))) => {
                if n1 == n2 {
                    continue;
                }
                return n1 < n2;
            }
            (Some(LPar), Some(LPar)) | (Some(RPar), Some(RPar)) => (),
            (Some(RPar) | None, Some(_)) => return true,
            (Some(LPar), Some(int @ Integer(_))) => {
                left.push(LPar);
                right.append(&mut vec![RPar, int, LPar]);
            }
            (Some(int @ Integer(_)), Some(LPar)) => {
                right.push(LPar);
                left.append(&mut vec![RPar, int, LPar]);
            }
            _ => return false,
        }
    }
}
