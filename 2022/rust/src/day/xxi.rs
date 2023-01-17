use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Op {
    Mul,
    Add,
    Sub,
    Div,
}

impl TryFrom<&str> for Op {
    type Error = &'static str;

    fn try_from(op: &str) -> Result<Self, Self::Error> {
        use Op::*;
        match op {
            "+" => Ok(Add),
            "-" => Ok(Sub),
            "*" => Ok(Mul),
            "/" => Ok(Div),
            _ => Err("Couldn't parse char as operation."),
        }
    }
}

impl Op {
    fn apply(&self, operand1: isize, operand2: isize) -> isize {
        match self {
            Op::Mul => operand1 * operand2,
            Op::Sub => operand1 - operand2,
            Op::Add => operand1 + operand2,
            Op::Div => operand1 / operand2,
        }
    }
}

type MonkeyName = String;
#[derive(Debug, Clone)]
enum Job {
    Operation((MonkeyName, Op, MonkeyName)),
    Result(isize),
}

enum Expr {
    Result(isize),
    Terms(Vec<(Op, isize)>),
}

pub fn equality_satisfier(input: &str) -> isize {
    let mut monkeys = parse_monkes(&input);

    0
}

pub fn root_monkey_nr(input: &str) -> isize {
    compute_monke("root", &parse_monkes(input))
}

fn parse_monkes(input: &str) -> HashMap<MonkeyName, Job> {
    input
        .lines()
        .map(|line| {
            let (name, raw_job) = line.split_once(':').unwrap();
            let raw_job = raw_job.split_ascii_whitespace().collect::<Vec<_>>();
            if let Ok(nr) = raw_job[0].parse::<isize>() {
                (name.into(), Job::Result(nr))
            } else {
                let job = Job::Operation((
                    raw_job[0].into(),
                    Op::try_from(raw_job[1]).unwrap(),
                    raw_job[2].into(),
                ));
                (name.into(), job)
            }
        })
        .collect()
}

fn compute_monke(name: &str, monkeys: &HashMap<MonkeyName, Job>) -> isize {
    match monkeys.get(name).unwrap() {
        Job::Operation((operand1, op, operand2)) => {
            let res1 = compute_monke(&operand1, monkeys);
            let res2 = compute_monke(&operand2, monkeys);
            op.apply(res1, res2)
        }
        Job::Result(n) => *n,
    }
}
