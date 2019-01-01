use advtools::input::iter_input_regex;
use advtools::rayon::prelude::*;

const FORMAT: &str = r"pos=< *(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)";

fn dist(p1: (i32, i32, i32), p2: (i32, i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()
}

#[derive(Clone, Copy)]
struct Bot { pos: (i32, i32, i32), radius: i32 }

impl Bot {
    fn overlaps(&self, other: &Bot) -> bool {
        dist(self.pos, other.pos) <= self.radius + other.radius
    }
}

fn main() {
    let bots: Vec<Bot> = iter_input_regex(FORMAT).map(
        |(pos, radius)| Bot { pos, radius }
    ).collect();

    // Part 1: Find the strongest bot and the number of bots in its radius.
    let &strongest = bots.iter().max_by_key(|b| b.radius).unwrap();
    let in_radius = bots.iter().filter(
        |b| dist(b.pos, strongest.pos) <= strongest.radius
    ).count();
    advtools::print("In radius", in_radius);

    // Part 2: Find the largest set of bots whose ranges all overlap.
    let largest_set = bots.par_iter().enumerate().map(|(i, b1)| {
        let mut connected: Vec<Bot> = Vec::new();
        for b1 in bots[i+1..].iter().filter(|b2| b2.overlaps(b1)) {
            if connected.iter().all(|b2| b2.overlaps(b1)) {
                connected.push(*b1);
            }
        }
        connected
    }).max_by_key(|v| v.len()).unwrap();

    // Since all bots overlap, the answer is the largest distance to an edge
    // of a range from the origin.
    let d = largest_set.iter().map(|b| dist(b.pos, (0, 0, 0)) - b.radius).max().unwrap();
    advtools::print("Distance to max overlap", d);
}
