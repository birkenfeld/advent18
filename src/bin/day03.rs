use advtools::prelude::{Itertools, HashMap, HashSet};
use advtools::input::{iter_input_trim, to_u32};

fn main() {
    // Parse the input into a hash map of claimed squares mapping to the
    // IDs of the claims.
    let mut claimed = HashMap::<_, Vec<u32>>::default();
    let mut all_ids = HashSet::default();
    for line in iter_input_trim::<Vec<String>>("#:") {
        let id = to_u32(&line[0]);
        let (x, y) = line[2].split(',').map(to_u32).collect_tuple().unwrap();
        let (w, h) = line[3].split('x').map(to_u32).collect_tuple().unwrap();
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

    println!("Multiply claimed: {}", multiply);
    println!("Only non-overlapping: {}", single_id);
}
