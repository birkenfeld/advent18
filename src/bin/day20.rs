use advtools::prelude::{Itertools, HashSet};
use advtools::input::input_string;

const N: usize = 100;
type Grid<T> = [[T; N]; N];
type Pos = (usize, usize);

#[derive(Clone, Copy)]
struct Doors { down: bool, right: bool }

// Recursively follow a path from given position and assign doors where necessary.
fn follow<'p>(grid: &mut Grid<Doors>, seen: &mut HashSet<(Pos, &'p str)>,
              (mut y, mut x): Pos, path: &'p str) -> Vec<Pos> {
    // If we've started at the same position with the same path, nothing to do.
    if !seen.insert(((y, x), path)) {
        return vec![];
    }
    let mut char_iter = path.chars();
    while let Some(ch) = char_iter.next() {
        match ch {
            // End of the full path, there's nowhere left to go.
            '$' => return vec![],
            // We have a direction where there must be a door.
            'N' => { y -= 1; grid[y][x].down = true }
            'S' => { grid[y][x].down = true; y += 1 }
            'W' => { x -= 1; grid[y][x].right = true }
            'E' => { grid[y][x].right = true; x += 1 }
            // Hit a branch.
            '(' => {
                // Determine all of the branching sub-paths.  Need to keep track of the
                // parenthesis level because of nested branches.
                let rest = char_iter.as_str();
                let mut paren_level = 1;
                let mut last = 0;
                let mut splits = vec![];
                let closing = rest.char_indices().find(|(i, c)| match c {
                    '(' => { paren_level += 1; false }
                    ')' => { paren_level -= 1; paren_level == 0 }
                    '|' => {
                        if paren_level == 1 {
                            splits.push((last, *i));
                            last = *i + 1;
                        }
                        false
                    }
                    _ => false
                }).unwrap().0;
                splits.push((last, closing));

                // For every sub-path, follow it, and for each of the resulting positions,
                // follow the rest of our path after the branch.  Then we are done, so return.
                return splits.into_iter().flat_map(|(i, j)| {
                    follow(grid, seen, (y, x), &rest[i..j]).into_iter().flat_map(|(y1, x1)| {
                        follow(grid, seen, (y1, x1), &rest[closing+1..])
                    }).collect_vec()
                }).collect();
            }
            // We cannot hit '|' or ')' as they are caught by the branch handling.
            _ => unreachable!("unhandled char: {}", ch)
        }
    }
    // Our (sub-)path has ended, but we didn't hit the end, so return final position.
    vec![(y, x)]
}

// Recursively find the shortest door count to reach each room.
fn count_doors(grid: &Grid<Doors>, doors: &mut Grid<u16>, (y, x): Pos, n: u16) {
    if n < doors[y][x] {
        doors[y][x] = n;
        if grid[y][x].down {
            count_doors(grid, doors, (y+1, x), n+1);
        }
        if grid[y][x].right {
            count_doors(grid, doors, (y, x+1), n+1);
        }
        if y > 0 && grid[y-1][x].down {
            count_doors(grid, doors, (y-1, x), n+1);
        }
        if x > 0 && grid[y][x-1].right {
            count_doors(grid, doors, (y, x-1), n+1);
        }
    }
}

fn main() {
    let full_path = input_string();
    let start = (N/2 + 1, N/2);
    // Grid that records for each room whether there is a door to the south
    // and the east direction.  (The other doors are covered by the adjacent
    // rooms to the north and west.)
    let mut grid = [[Doors { down: false, right: false }; N]; N];

    follow(&mut grid, &mut HashSet::new(), start, &full_path.trim()[1..]);

    let mut doors = [[u16::max_value(); N]; N];
    count_doors(&grid, &mut doors, start, 0);

    let max_doors = doors.iter().flat_map(|line| line.iter().max()).max().unwrap();
    advtools::verify("Maximum needed doors", max_doors, 4155);

    let many_doors = doors.iter().map(|line| {
        line.iter().filter(|&&d| d >= 1000).count()
    }).sum::<usize>();
    advtools::verify("Rooms >= 1000 doors", many_doors, 8434);
}
