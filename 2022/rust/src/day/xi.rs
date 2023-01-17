use gcd::Gcd;
use std::collections::VecDeque;

type WorryLevel = usize;
type MonkeyIndex = usize;

#[derive(Debug)]
struct Monkey<'a> {
    items: VecDeque<WorryLevel>,
    expr: Vec<&'a str>,
    div: usize,
    when_true: MonkeyIndex,
    when_false: MonkeyIndex,
    inspections: usize,
}

impl<'a> Monkey<'a> {
    fn new(
        items: VecDeque<WorryLevel>,
        expr: &'a str,
        divisor: usize,
        when_true: MonkeyIndex,
        when_false: MonkeyIndex,
    ) -> Self {
        Monkey {
            items,
            expr: expr.split_ascii_whitespace().collect(),
            div: divisor,
            when_true,
            when_false,
            inspections: 0,
        }
    }

    fn update(
        &mut self,
        worry_management: impl Fn(WorryLevel) -> WorryLevel,
    ) -> Vec<(WorryLevel, MonkeyIndex)> {
        let mut item_targets = Vec::new();
        while let Some(item) = self.items.pop_front() {
            let new = worry_management(self.operation(item));
            item_targets.push((new, self.test(new)));
            self.inspections += 1;
        }
        item_targets
    }

    fn operation(&self, old: WorryLevel) -> WorryLevel {
        match (self.expr[1], self.expr[2]) {
            ("+", "old") => old + old,
            ("*", "old") => old * old,
            ("+", n) => old + n.parse::<usize>().unwrap(),
            ("*", n) => old * n.parse::<usize>().unwrap(),
            _ => unreachable!(),
        }
    }

    fn test(&self, item: WorryLevel) -> MonkeyIndex {
        if item % self.div == 0 {
            self.when_true
        } else {
            self.when_false
        }
    }
}

pub fn part_one(input: &str) -> isize {
    monkey_business(parse_monkeys(input), 20, |worry| worry / 3) as isize
}

pub fn part_two(input: &str) -> isize {
    let monkeys: Vec<Monkey> = parse_monkeys(input);

    let divs = monkeys.iter().map(|m| m.div);
    let lcm = divs.clone().reduce(std::ops::Mul::mul).unwrap()
        / divs.clone().fold(0, |acc, d| acc.gcd(d));

    monkey_business(monkeys, 10_000, |worry| worry % lcm) as isize
}

fn monkey_business(
    mut monkeys: Vec<Monkey>,
    rounds: usize,
    manager: impl Fn(WorryLevel) -> WorryLevel,
) -> usize {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for (worry, idx) in monkeys[i].update(&manager) {
                monkeys[idx].items.push_back(worry);
            }
        }
    }

    top_mults_sum(monkeys)
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            let monkey: Vec<&str> = monkey.split('\n').collect();
            let items = VecDeque::from(super::get_numbers(monkey[1]));
            let (_, expr) = monkey[2].split_once('=').unwrap();
            let divisor = super::get_number(monkey[3]);
            let when_true = super::get_number(monkey[4]);
            let when_false = super::get_number(monkey[5]);
            Monkey::new(items, expr, divisor, when_true, when_false)
        })
        .collect()
}

fn top_mults_sum(mut monkeys: Vec<Monkey>) -> usize {
    monkeys.sort_by_key(|m| m.inspections);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspections)
        .reduce(std::ops::Mul::mul)
        .unwrap()
}
