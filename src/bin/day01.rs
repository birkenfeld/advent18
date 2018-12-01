use advtools::prelude::*;

fn main() {
    let changes = advtools::iter_input::<i32>().collect_vec();
    let freq1 = changes.iter().fold(0, |v, n| v + n);
    println!("First round: {}", freq1);

    let (mut accum, mut seen) = (0, HashSet::default());
    changes.iter().cycle().find(|&n| { accum += n; !seen.insert(accum) });
    println!("Second round: {:?}", accum);
}
