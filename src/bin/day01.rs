use advtools::prelude::HashSet;
use advtools::input;

fn main() {
    // Let the helper library parse all lines and collect them.
    let changes = input::parse_vec::<i32>();
    // The first part is just a sum from 0, nothing special going on.
    let freq1 = changes.iter().sum::<i32>();
    advtools::verify("First round", freq1, 513);

    let (mut freq2, mut seen) = (0, HashSet::new());
    // Here we use the fact that `HashSet::insert` returns false if the
    // key was already in the set. cycle() is another nice iterator method.
    changes.into_iter().cycle().find(|d| { freq2 += d; !seen.insert(freq2) });
    advtools::verify("Second round", freq2, 287);
}
