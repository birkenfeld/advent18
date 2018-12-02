use advtools::prelude::*;
use advtools::itertools::izip;

fn main() {
    let ids = advtools::iter_input::<String>().collect_vec();
    let (doubles, triples) = ids.iter().fold((0, 0), |(dbls, tpls), id| {
        let mut freqs = HashMap::<_, u32>::default();
        id.chars().for_each(|c| *freqs.entry(c).or_default() += 1);
        (dbls + freqs.values().any(|&n| n == 2) as u32,
         tpls + freqs.values().any(|&n| n == 3) as u32)
    });
    println!("Checksum: {}", doubles * triples);

    for (id, id2) in ids.iter().tuple_combinations() {
        let chars = izip!(0.., id.chars(), id2.chars());
        if let Some(((n, ..),)) = chars.filter(|&(_, a, b)| a != b).collect_tuple() {
            let others: String = id.chars().take(n).chain(id.chars().skip(n+1)).collect();
            println!("Common ID: {}", others);
            return;
        }
    }
}
