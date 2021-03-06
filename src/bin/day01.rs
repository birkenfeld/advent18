use advtools::prelude::{Itertools, HashSet};
use advtools::input::iter_input;

fn main() {
    // Let the helper library parse all lines and collect them.
    let changes = iter_input::<i32>().collect_vec();
    // The first part is perfect for a fold(), nothing special going on.
    let freq1 = changes.iter().fold(0, |v, n| v + n);
    advtools::verify("First round", freq1, 513);

    let (mut freq2, mut seen) = (0, HashSet::new());
    // Here we use the fact that `HashSet::insert` returns false if the
    // key was already in the set. cycle() is another nice iterator method.
    changes.into_iter().cycle().find(|d| { freq2 += d; !seen.insert(freq2) });
    advtools::verify("Second round", freq2, 287);
}
