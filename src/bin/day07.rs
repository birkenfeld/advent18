use advtools::prelude::{HashMap, Itertools};
use advtools::input::iter_input_parts;
use petgraph::prelude::*;

struct Task {
    id: char,
    worker: Option<u8>,
    time_left: u8,
}

fn main() {
    // Create a directed graph where node weights are the assigned letter.
    let mut graph = StableGraph::new();
    let mut nodes = HashMap::new();
    for (a, b) in iter_input_parts::<(char, char), _>((1, 7)) {
        let na = *nodes.entry(a).or_insert_with(|| graph.add_node(a));
        let nb = *nodes.entry(b).or_insert_with(|| graph.add_node(b));
        graph.add_edge(na, nb, ());
    }
    // Extended graph for part 2 with worker assignment and work time left.
    let mut graph2 = graph.map(|_, n| {
        Task { id: *n, worker: None, time_left: *n as u8 - b'A' + 61 }
    }, |_, _| ());

    // Part 1: extract (and remove) graph roots in alphabetical order.
    let mut order = String::new();
    while graph.node_count() > 0 {
        let root = graph.externals(Incoming).sorted_by_key(|&n| graph[n])[0];
        order.push(graph.remove_node(root).unwrap());
    }
    println!("Work order: {}", order);

    // Part 2: assign tasks to workers and track required time.
    let mut clock = -1;
    let mut free_workers = vec![1, 2, 3, 4, 5];
    while graph2.node_count() > 0 {
        clock += 1;
        // First, check for tasks that are done.
        for node in graph2.externals(Incoming).collect_vec() {
            if let Some(worker) = graph2[node].worker {
                graph2[node].time_left -= 1;
                if graph2[node].time_left == 0 {
                    free_workers.push(worker);
                    graph2.remove_node(node);
                }
            }
        }
        // Second, assign workers to any available tasks (some may have become
        // available due to pruning).
        for node in graph2.externals(Incoming).sorted_by_key(|&n| graph2[n].id) {
            if let None = graph2[node].worker {
                graph2[node].worker = free_workers.pop();
            }
        }
    }
    println!("Seconds required: {}", clock);
}
