use std::collections::HashSet;

type ArenaIndex = usize;

#[derive(Debug, Default, Clone)]
struct Dir {
    name: String,
    idx: ArenaIndex,
    size: usize,
    parent: Option<ArenaIndex>,
    // in case input has ls on same dir multiple times
    files_accounted: HashSet<String>,
    accounted: bool,
    children: HashSet<ArenaIndex>,
}

impl Dir {
    fn new(idx: ArenaIndex, name: String, parent: Option<ArenaIndex>) -> Self {
        Self {
            idx,
            name,
            parent,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
struct Graph {
    arena: Vec<Dir>,
}

impl Graph {
    fn new(traversal: Vec<&str>) -> Self {
        let mut dirs = Graph::default();
        dirs.arena.push(Dir::new(0, "/".to_string(), None));
        let mut curr = 0;
        let mut i = 1;

        loop {
            if traversal[i].is_empty() {
                break;
            }
            let line = traversal[i].split_whitespace().collect::<Vec<&str>>();
            let cmd = line[1];
            if cmd == "ls" {
                i += 1;
                loop {
                    if i >= traversal.len() {
                        break;
                    }
                    let line = traversal[i].split_whitespace().collect::<Vec<&str>>();
                    match line[0] {
                        "dir" => (),
                        "$" => {
                            i -= 1;
                            break;
                        }
                        n => {
                            let fname = line[1];
                            if !dirs.arena[curr].files_accounted.contains(fname) {
                                dirs.arena[curr].size += n.parse::<usize>().unwrap();
                            }
                            dirs.arena[curr].files_accounted.insert(fname.to_string());
                        }
                    }
                    i += 1;
                }
                dirs.propagate_size(curr);
            } else {
                // line[1] = cd
                let dest = line[2];
                if dest == ".." {
                    curr = dirs.arena[curr].parent.unwrap();
                } else if dest == "/" {
                    curr = 0;
                } else {
                    let new = dirs.insert_or_get(dest.to_string(), curr);
                    dirs.arena[curr].children.insert(new);
                    curr = new;
                }
            }
            i += 1;
        }

        dirs
    }

    fn insert_or_get(&mut self, name: String, parent: ArenaIndex) -> ArenaIndex {
        let name = format!(
            "{}{}{name}",
            &self.arena[parent].name,
            if parent != 0 { "/" } else { "" }
        );
        for node in &self.arena {
            if node.name == name {
                return node.idx;
            }
        }
        let idx = self.arena.len();
        self.arena.push(Dir::new(idx, name, Some(parent)));
        idx
    }

    fn propagate_size(&mut self, orig: ArenaIndex) {
        if self.arena[orig].accounted {
            return;
        }

        let size = self.arena[orig].size;
        let mut curr = orig;
        while let Some(parent) = self.arena[curr].parent {
            self.arena[parent].size += size;
            curr = parent;
        }

        self.arena[orig].accounted = true;
    }

    fn size(&self) -> usize {
        self.arena[0].size
    }
}

pub fn sum_small_dirs(input: &str) -> isize {
    let graph = Graph::new(input.to_string().split('\n').collect());
    graph
        .arena
        .iter()
        .map(|dir| if dir.size <= 100_000 { dir.size } else { 0 })
        .sum::<usize>() as isize
}

pub fn smallest_viable_deletion(input: &str, required: usize) -> isize {
    const TOTAL: usize = 70_000_000;
    let graph = Graph::new(input.to_string().split('\n').collect());
    let missing = required - (TOTAL - graph.size());
    graph
        .arena
        .iter()
        .filter_map(|dir| {
            if dir.size >= missing {
                Some(dir.size)
            } else {
                None
            }
        })
        .min()
        .unwrap() as isize
}
