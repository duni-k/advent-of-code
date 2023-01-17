type FileIndex = usize;
#[derive(Debug)]
struct Node {
    prev: FileIndex,
    next: FileIndex,
    val: isize,
}

impl Node {
    fn new(val: isize, prev: FileIndex, next: FileIndex) -> Self {
        Node { val, prev, next }
    }
}

pub fn sum_coords(input: &str) -> isize {
    let vals: Vec<isize> = input
        .lines()
        .flat_map(|line| line.parse::<isize>())
        .collect();

    let mut nodes = Vec::with_capacity(vals.len());
    nodes.push(Node::new(vals[0], vals.len() - 1, 2));
    for i in 1..(vals.len() - 2) {
        nodes.push(Node::new(vals[i], i - 1, i + 1));
    }
    nodes.push(Node::new(vals[vals.len() - 1], vals.len() - 2, 0));
    dbg!(&nodes);

    0
}

fn mix_file(file: Vec<Node>) -> Vec<isize> {
    for i in 0..file.len() {
        let val = file.get_mut(i).unwrap();
        file[i].prev = file
    }
}
