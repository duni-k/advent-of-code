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

pub fn indices_sum(input: &str) -> isize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, pair)| {
            if let Some((fst, snd)) =
                pair.split_once('\n') && ordered(lex(fst), lex(snd)) {
                    println!("True");
                Some(i + 1)
            } else {
                    println!("False");
                None
            }
        })
        .sum::<usize>() as isize
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
        println!("left: {:?}", &left.iter().rev().collect::<Vec<_>>());
        println!("right: {:?}", &right.iter().rev().collect::<Vec<_>>());
        match (left.pop(), right.pop()) {
            (Some(Integer(n1)), Some(Integer(n2))) => {
                println!("Comparing {n1} with {n2}");
                if n1 == n2 {
                    continue;
                }
                return n1 < n2;
            }
            (Some(RPar), Some(RPar)) => (),
            (Some(LPar), Some(LPar)) => return ordered(left, right),
            (Some(RPar) | None, Some(_)) => {
                println!("Left side out of items.");
                return true;
            } // left empty
            (Some(_), None | Some(RPar)) => {
                println!("Right side out of items");
                return false;
            } // right empty
            (Some(LPar), Some(int @ Integer(_))) => {
                left.push(LPar);
                right.append(&mut vec![RPar, int, LPar]);
            }
            (Some(int @ Integer(_)), Some(LPar)) => {
                right.push(LPar);
                left.append(&mut vec![RPar, int, LPar]);
            }
            (None, None) => unreachable!(), // assume no twins
        }
    }
}
