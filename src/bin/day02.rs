use advtools::prelude::*;
use advtools::itertools::izip;

fn main() {
    let ids = advtools::iter_input::<String>().collect_vec();
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

    // This part makes heavy use of the goodies in itertools.
    // tuple_combinations() example: [a, b, c] -> (a, b), (a, c), (b, c)
    for (id, id2) in ids.iter().tuple_combinations() {
        // izip! lets us zip more than one iterator without nesting pairs.
        let chars = izip!(0.., id.chars(), id2.chars());
        // collect_tuple() returns `Some` only if there is the exact number of
        // items.  In this case, we want to find two IDs with exactly *one*
        // differing position.
        if let Some(((n, ..),)) = chars.filter(|&(_, a, b)| a != b).collect_tuple() {
            // Take out the differing item and reconstruct a string.
            let others: String = id.chars().take(n).chain(id.chars().skip(n+1)).collect();
            println!("Common ID: {}", others);
            return;
        }
    }
}
