use advtools::prelude::Itertools;
use advtools::input::iter_input_regex;
use std::iter::once;

const FORMAT: &str = r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>";
const TARGET_HEIGHT: i32 = 10;

fn main() {
    let mut points: Vec<((i32, i32), (i32, i32))> = iter_input_regex(FORMAT).collect();
    for i in 1.. {
        // Process movement of stars.
        for (p, v) in &mut points {
            p.0 += v.0;
            p.1 += v.1;
        }

        // The goal is for the spread in Y coordinates (line height) to be
        // minimal.  At most TARGET_HEIGHT should be reached.
        let (y1, y2) = points.iter().map(|(p, _)| p.1).minmax().into_option().unwrap();
        if y2 - y1 + 1 <= TARGET_HEIGHT {
            // Determine how many grid points we need in X direction as well.
            let (x1, x2) = points.iter().map(|(p, _)| p.0).minmax().into_option().unwrap();
            // Arrange stars into a grid.
            let mut grid = vec![vec![' '; (x2 - x1 + 1) as usize]; TARGET_HEIGHT as usize];
            for (p, _) in &points {
                grid[(p.1 - y1) as usize][(p.0 - x1) as usize] = '#';
            }
            // Format as a string and output the solutions.
            let msg: String = grid.iter().flat_map(|line| once(&'\n').chain(line)).collect();
            advtools::print("Message", msg);
            return advtools::print("Seconds to get there", i);
        }
    }
}
