use advtools::prelude::{Itertools, HashSet};
use advtools::input::input_string;
use advtools::grid::{Grid, Pos};

const N: usize = 100;

#[derive(Clone, Copy)]
struct Doors { down: bool, right: bool }

// Recursively follow a path from given position and assign doors where necessary.
fn follow<'p>(grid: &mut Grid<Doors>, seen: &mut HashSet<(Pos, &'p str)>,
              mut pos: Pos, path: &'p str) -> Vec<Pos> {
    // If we've started at the same position with the same path, nothing to do.
    if !seen.insert((pos, path)) {
        return vec![];
    }
    let mut char_iter = path.chars();
    while let Some(ch) = char_iter.next() {
        match ch {
            // End of the full path, there's nowhere left to go.
            '$' => return vec![],
            // We have a direction where there must be a door.
            'N' => { pos.step_up(); grid[pos].down = true }
            'S' => { grid[pos].down = true; pos.step_down() }
            'W' => { pos.step_left(); grid[pos].right = true }
            'E' => { grid[pos].right = true; pos.step_right() }
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
                    follow(grid, seen, pos, &rest[i..j]).into_iter().flat_map(|pos1| {
                        follow(grid, seen, pos1, &rest[closing+1..])
                    }).collect_vec()
                }).collect();
            }
            // We cannot hit '|' or ')' as they are caught by the branch handling.
            _ => unreachable!("unhandled char: {}", ch)
        }
    }
    // Our (sub-)path has ended, but we didn't hit the end, so return final position.
    vec![pos]
}

// Recursively find the shortest door count to reach each room.
fn count_doors(grid: &Grid<Doors>, doors: &mut Grid<u16>, pos: Pos, n: u16) {
    if n < doors[pos] {
        doors[pos] = n;
        if grid[pos].down {
            count_doors(grid, doors, pos.down(), n+1);
        }
        if grid[pos].right {
            count_doors(grid, doors, pos.right(), n+1);
        }
        if pos.y > 0 && grid[pos.up()].down {
            count_doors(grid, doors, pos.up(), n+1);
        }
        if pos.x > 0 && grid[pos.left()].right {
            count_doors(grid, doors, pos.left(), n+1);
        }
    }
}

fn main() {
    let full_path = input_string();
    let start = Pos(N as i32/2, N as i32/2 + 1);
    // Grid that records for each room whether there is a door to the south
    // and the east direction.  (The other doors are covered by the adjacent
    // rooms to the north and west.)
    let mut grid = Grid::new(vec![vec![Doors { down: false, right: false }; N]; N]);

    follow(&mut grid, &mut HashSet::new(), start, &full_path.trim()[1..]);

    let mut doors = Grid::new(vec![vec![u16::max_value(); N]; N]);
    count_doors(&grid, &mut doors, start, 0);

    let max_doors = doors.iter().flat_map(|line| line.iter().max()).max().unwrap();
    advtools::verify("Maximum needed doors", max_doors, 4155);

    let many_doors = doors.iter().map(|line| {
        line.iter().filter(|&&d| d >= 1000).count()
    }).sum::<usize>();
    advtools::verify("Rooms >= 1000 doors", many_doors, 8434);
}
