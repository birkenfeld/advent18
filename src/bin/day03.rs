use advtools::prelude::{Itertools, HashMap, HashSet, ArrayVec};
use advtools::input;

fn main() {
    // Parse the input into a hash map of claimed squares mapping to the
    // IDs of the claims.
    let mut claimed = HashMap::<(u32, u32), ArrayVec<u16, 8>>::new();
    let mut all_ids = HashSet::new();
    for (id, x, y, w, h) in input::rx_lines(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)") {
        let (w, h): (u32, u32) = (w, h);
        for i in x..x+w {
            for j in y..y+h {
                claimed.entry((i, j)).or_default().push(id);
            }
        }
        all_ids.insert(id);
    }

    // Calculate the solution for both parts: part 1 is just the number of
    // claim lists with more than one entry.  For part 2, we remove all
    // IDs in such entries from the "all ids" set.
    let multiply = claimed.values().filter(|c| c.len() > 1).inspect(|claim| {
        claim.iter().for_each(|i| { all_ids.remove(i); })
    }).count();
    // This gets an ID out of the set and at the same time asserts len == 1.
    let (single_id,) = all_ids.iter().collect_tuple().unwrap();

    advtools::verify("Multiply claimed", multiply, 118322);
    advtools::verify("Only non-overlapping", single_id, 1178);
}
