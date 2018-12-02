use advtools::prelude::*;

fn main() {
    // Let the helper library parse all lines and collect them.
    let changes = advtools::iter_input::<i32>().collect_vec();
    // The first part is perfect for a fold(), nothing special going on.
    let freq1 = changes.iter().fold(0, |v, n| v + n);
    println!("First round: {}", freq1);

    let (mut freq2, mut seen) = (0, HashSet::default());
    // Here we use the fact that `HashSet::insert` returns false if the
    // key was already in the set. cycle() is another nice iterator method.
    changes.into_iter().cycle().find(|d| { freq2 += d; !seen.insert(freq2) });
    println!("Second round: {:?}", freq2);
}
