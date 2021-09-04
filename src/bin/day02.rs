use advtools::prelude::{Itertools, HashMap};
use advtools::input::iter_lines;

fn main() {
    let ids = iter_lines().collect_vec();
    let id_len = ids[0].len();
    // Using fold here lets us keep track of the doubles/triples state
    // in the iterator without mutable outer variables.
    let (doubles, triples) = ids.iter().fold((0, 0), |(dbls, tpls), id| {
        let mut freqs = HashMap::<_, u32>::new();
        // Determine frequency of every character in the ID using a hashmap.
        id.chars().for_each(|c| *freqs.entry(c).or_default() += 1);
        // If we find any of the needed frequency, casting the bool to u32
        // gives "+ 0" or "+ 1".
        (dbls + freqs.values().any(|&n| n == 2) as u32,
         tpls + freqs.values().any(|&n| n == 3) as u32)
    });
    advtools::verify("Checksum", doubles * triples, 6175);

    // tuple_combinations() example: [a, b, c] -> (a, b), (a, c), (b, c)
    let new_id = ids.iter().tuple_combinations().map(|(id, id2)| {
        // For each combination, make a new string with only common characters.
        id.chars().zip(id2.chars())
                  .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                  .collect::<String>()
    }).find(|v| v.len() == id_len - 1).unwrap();
    advtools::verify("Common ID", new_id, "asgwjcmzredihqoutcylvzinx");
}
