use advtools::input;

const FORMAT: &str = r"(-?\d+),(-?\d+),(-?\d+),(-?\d+)";

fn dist(p1: [i32; 4], p2: [i32; 4]) -> i32 {
    p1.iter().zip(&p2).map(|(a, b)| (a - b).abs()).sum()
}

fn main() {
    let points: Vec<[i32; 4]> = input::rx_lines(FORMAT).collect();
    let mut constellations: Vec<Vec<_>> = vec![];

    for point in points {
        // For each point, revise the list of constellations by taking
        // any that contain the new point, and merging them into one
        // big new constellation, including the new point.
        let mut new_constellation = vec![point];
        for constellation in std::mem::take(&mut constellations) {
            if constellation.iter().any(|&p| dist(p, point) <= 3) {
                new_constellation.extend(constellation);
            } else {
                constellations.push(constellation);
            }
        }
        constellations.push(new_constellation);
    }
    advtools::verify("Number of constellations", constellations.len(), 327);
}
