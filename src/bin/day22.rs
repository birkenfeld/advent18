use advtools::prelude::{Itertools, HashSet, HashMap};
use advtools::input::{iter_lines, to_u32, to_usize};
use priority_queue::PriorityQueue;

const MODULO: u32 = 20183;
const X_FACTOR: u32 = 16807;
const Y_FACTOR: u32 = 48271;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Equip { Nothing, Lamp, Gear }

#[derive(Clone, Copy)]
enum Type { Rocky, Wet, Narrow }

use {Type::*, Equip::*};

impl Type {
    fn new(erosion: u32) -> Self {
        match (erosion % MODULO) % 3 {
            0 => Rocky,
            1 => Wet,
            2 => Narrow,
            _ => unreachable!()
        }
    }
}

fn transition(g: Equip, r1: Type, r2: Type) -> Equip {
    match (r1, r2) {
        (Rocky, Rocky) | (Wet, Wet) | (Narrow, Narrow) => g,
        (Rocky, Wet) | (Wet, Rocky) => Gear,
        (Rocky, Narrow) | (Narrow, Rocky) => Lamp,
        (Wet, Narrow) | (Narrow, Wet) => Nothing,
    }
}

type Node = (usize, usize, Equip);

// Priority of a node for the priority queue in the A* open set.
fn prio(add: u32, n1: Node, n2: Node) -> u32 {
    u32::max_value() -
        (add +
         (if n1.0 > n2.0 { n1.0 - n2.0 } else { n2.0 - n1.0 }) as u32 +
         (if n1.1 > n2.1 { n1.1 - n2.1 } else { n2.1 - n1.1 }) as u32)
}

// Iterator over (existing) neighbor nodes.  The equipped gear is part of
// the node, so there are two nodes per (y, x) square.
fn neighbors<'a>(risk: &'a [Vec<Type>], (y, x, g): Node) -> impl Iterator<Item=Node> + 'a {
    vec![(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y+1, x), (y, x+1)]
        .into_iter().filter_map(move |(y1, x1)| if y1 < risk.len() && x1 < risk[0].len() {
            Some((y1, x1, transition(g, risk[y][x], risk[y1][x1])))
        } else {
            None
        })
}

// Implementation of A* for finding the shortest path.
fn find_path(risk: &[Vec<Type>], start: Node, target: Node) -> u32 {
    let mut open = PriorityQueue::new();
    let mut openset = HashSet::new();
    let mut closed = HashSet::new();
    let mut distance = HashMap::new();

    distance.insert(start, 0);
    open.push(start, prio(0, start, target));

    loop {
        let (node, _) = open.pop().unwrap();
        openset.remove(&node);
        closed.insert(node);

        if node == target {
            return distance[&node];
        }

        for neigh in neighbors(risk, node) {
            if closed.contains(&neigh) {
                continue;
            }
            let tentative = distance[&node] + if neigh.2 != node.2 { 8 } else { 1 };
            if !openset.contains(&neigh) {
                open.push(neigh, 0);
                openset.insert(neigh);
            } else if tentative > distance[&neigh] {
                continue;
            }
            open.change_priority(&neigh, prio(tentative, neigh, target));
            distance.insert(neigh, tentative);
        }
    }
}

fn main() {
    let mut input = iter_lines();
    let depth = to_u32(input.next().unwrap().split_whitespace().nth(1).unwrap());
    let (tx, ty) = input.next().unwrap().split_whitespace().nth(1).unwrap()
        .split(",").map(to_usize).collect_tuple().unwrap();

    let mut index = vec![vec![0; tx+50]; ty+50];
    for y in 0..index.len() {
        for x in 0..index[0].len() {
            if y == 0 {
                index[y][x] = (x as u32 * X_FACTOR) % MODULO;
            } else if x == 0 {
                index[y][x] = (y as u32 * Y_FACTOR) % MODULO;
            } else {
                index[y][x] = ((index[y-1][x] + depth) *
                               (index[y][x-1] + depth)) % MODULO;
            }
        }
    }
    index[ty][tx] = 0;

    let risk = index.iter().map(|line| {
        line.iter().map(|ix| Type::new(ix + depth)).collect_vec()
    }).collect_vec();

    let total_risk = risk.iter().take(ty+1).flat_map(
        |line| line.iter().take(tx+1).map(|v| *v as u32)
    ).sum::<u32>();
    advtools::verify("Risk level", total_risk, 7743);

    let path_len = find_path(&risk, (0, 0, Lamp), (ty, tx, Lamp));
    advtools::verify("Path length", path_len, 1029);
}
