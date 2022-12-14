use regex::Regex;
use std::collections::HashSet;

type ValveIndex = usize;

#[derive(Debug, Default)]
struct Valve {
    name: String,
    flow_rate: usize,
    parents: HashSet<ValveIndex>,
    children: HashSet<ValveIndex>,
    open: bool,
}

impl Valve {
    fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
struct ValveSystem {
    valves: Vec<Valve>,
    released: usize,
    minute: usize,
}

impl ValveSystem {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn add_sink(&mut self) {
        let super_sink = self.insert_or_get("SUPASINKU".into());
        self.valves[super_sink].parents = HashSet::from_iter(0..super_sink);
        for valve_idx in 0..super_sink {
            self.valves[valve_idx].children.insert(super_sink);
        }
    }

    fn insert_or_get(&mut self, name: String) -> ValveIndex {
        for (idx, valve) in self.valves.iter().enumerate() {
            if valve.name == name {
                return idx;
            }
        }

        let idx = self.valves.len();
        self.valves.push(Valve::new(name));
        idx
    }

    fn max_flow(&mut self, source: ValveIndex, deadline: usize) -> usize {
        let mut flow = 0;

        flow
    }
}

pub fn max_release(input: &str) -> isize {
    const DEADLINE: usize = 30;
    let mut sys = parse_valves(input);

    sys.max_flow(0, DEADLINE) as isize
}

fn parse_valves(input: &str) -> ValveSystem {
    let valve_re = Regex::new(r"Valve (\w{2}).*rate=(\d+).*valves?\s(.*)").unwrap();
    let mut sys = ValveSystem::new();

    for line in input.lines() {
        let groups = valve_re.captures(line).unwrap();

        let idx = sys.insert_or_get(groups[1].into());
        sys.valves[idx].flow_rate = groups[2].parse::<usize>().unwrap();
        for child in groups[3].split(", ") {
            let child_idx = sys.insert_or_get(String::from(child));
            sys.valves[child_idx].parents.insert(idx);
            sys.valves[idx].children.insert(child_idx);
        }
    }

    sys
}
