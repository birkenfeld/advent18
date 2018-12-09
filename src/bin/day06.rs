use advtools::prelude::{Itertools, itertools::iproduct};
use advtools::input::iter_input_trim;

const SIZE: i32 = 375;

fn manhattan_dist((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn main() {
    let points = iter_input_trim::<(i32, i32)>(",").collect_vec();

    // Part 1: find size of largest finite area nearest to a single point.
    let mut area_sizes = vec![0; points.len()];

    for p in iproduct!(0..SIZE, 0..SIZE) {
        // For every grid point, find distances to all points, and their minimum.
        let dists = points.iter().map(|&pc| manhattan_dist(p, pc)).collect_vec();
        let min_dist = dists.iter().min().unwrap();
        // Only look at points without a tie for minimum.
        if let Some(((i, _),)) = dists.iter().enumerate()
                                             .filter(|j| j.1 == min_dist).collect_tuple() {
            // If area is at the edge...
            if p.0 == 0 || p.0 == SIZE-1 || p.1 == 0 || p.1 == SIZE-1 {
                // ... remove it from consideration, since it is infinite.
                area_sizes[i] = i32::min_value();
            } else {
                area_sizes[i] += 1;
            }
        }
    }

    let max_area_size = area_sizes.into_iter().max().unwrap();
    advtools::print("Largest area", max_area_size);

    // Part 2: find size of region with limited distance to all points.
    let region_size = iproduct!(0..SIZE, 0..SIZE)
        .map(|p| points.iter().map(|&pc| manhattan_dist(p, pc)).sum::<i32>())
        .filter(|&i| i < 10000)
        .count();
    advtools::print("Limited distance region size", region_size);
}
