use advtools::prelude::Itertools;
use advtools::input::iter_input_regex;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Square { Sand, Clay, Flow, Still }
use Square::*;

enum Flow { Down, Side }

// Recursively fill the grid in given direction, starting at given point.
fn fill(grid: &mut [[Square; 600]], py: usize, px: usize, flow: Flow) {
    match flow {
        // Flow down until we hit something that isn't sand
        Flow::Down => for dy in 1.. {
            // Stop at bottom Y coordinate
            if py+dy >= grid.len() {
                return;
            }
            match grid[py+dy][px] {
                Sand => grid[py+dy][px] = Flow,
                // If we hit something already flowing, let it flow
                Flow => break,
                // If we hit bottom or an existing well, spread on top
                Clay | Still => {
                    fill(grid, py+dy-1, px, Flow::Side);
                    break;
                }
            }
        },
        // Flow to both sides until we hit a wall or a drop
        Flow::Side => {
            grid[py][px] = Flow;
            let mut enclosed = [0, 0];
            // Flow to left then right
            for dir in [0, 1] {
                for dx in 1.. {
                    let new_x = if dir == 0 { px - dx } else { px + dx };
                    match grid[py][new_x] {
                        Sand | Flow => {
                            grid[py][new_x] = Flow;
                            // If we hit sand and below is open space,
                            // stop flowing to side and fall down
                            if grid[py+1][new_x] == Sand {
                                fill(grid, py, new_x, Flow::Down);
                                break;
                            }
                        }
                        // If we hit a wall, stop flowing
                        Clay | Still => {
                            enclosed[dir] = dx;
                            break;
                        }
                    }
                }
            }
            // If flow hit a wall at both ends, convert flowing to still
            // water and continue flowing to side one tile above
            if enclosed[0] > 0 && enclosed[1] > 0 {
                for x in px+1-enclosed[0]..px+enclosed[1] {
                    grid[py][x] = Still;
                }
                fill(grid, py-1, px, Flow::Side);
            }
        }
    }
}

fn main() {
    let mut grid = vec![[Sand; 600]; 2000];
    for line in iter_input_regex(r"(x|y)=(\d+), .=(\d+)\.\.(\d+)") {
        let (coord, j, i1, i2): (char, usize, usize, usize) = line;
        for i in i1..=i2 {
            if coord == 'x' {
                grid[i][j] = Clay;
            } else {
                grid[j][i] = Clay;
            }
        }
    }
    // Determine minimum and maximum grid Y coordinates.
    let (min_y, max_y) = (0..grid.len()).filter(|&i| grid[i].iter().any(|&c| c == Clay))
                                        .minmax().into_option().unwrap();
    grid.truncate(max_y + 1);

    // Fill from the source tile.
    fill(&mut grid, 0, 500, Flow::Down);

    let (still_tiles, flowing_tiles) = grid.iter().skip(min_y).map(|line| {
        (line.iter().filter(|&&sq| sq == Still).count(),
         line.iter().filter(|&&sq| sq == Flow).count())
    }).fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    advtools::verify("Water tiles", still_tiles + flowing_tiles, 39367);
    advtools::verify("Non-drying tiles", still_tiles, 33061);
}
