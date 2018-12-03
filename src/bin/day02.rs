use advtools::prelude::{Itertools, HashMap};
use advtools::input::iter_input;

fn main() {
    let ids = iter_input::<String>().collect_vec();
    // Using fold here lets us keep track of the doubles/triples state
    // in the iterator without mutable outer variables.
    let (doubles, triples) = ids.iter().fold((0, 0), |(dbls, tpls), id| {
        let mut freqs = HashMap::<_, u32>::default();
        // Determine frequency of every character in the ID using a hashmap.
        id.chars().for_each(|c| *freqs.entry(c).or_default() += 1);
        // If we find any of the needed frequency, casting the bool to u32
        // gives "+ 0" or "+ 1".
        (dbls + freqs.values().any(|&n| n == 2) as u32,
         tpls + freqs.values().any(|&n| n == 3) as u32)
    });
    println!("Checksum: {}", doubles * triples);

    // tuple_combinations() example: [a, b, c] -> (a, b), (a, c), (b, c)
    for (id, id2) in ids.iter().tuple_combinations() {
        // Make a new string with only common characters.
        let only_commons: String = id.chars().zip(id2.chars()).filter_map(
            |(a, b)| if a == b { Some(a) } else { None }).collect();
        if only_commons.len() == id.len() - 1 {
            println!("Common ID: {}", only_commons);
            return;
        }
    }
}
