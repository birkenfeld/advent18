use advtools::prelude::*;

fn main() {
    let changes = advtools::iter_input::<i32>().collect_vec();
    let freq1 = changes.iter().fold(0, |v, n| v + n);
    println!("First round: {}", freq1);

    let (mut freq2, mut seen) = (0, HashSet::default());
    changes.into_iter().cycle().find(|d| { freq2 += d; !seen.insert(freq2) });
    println!("Second round: {:?}", freq2);
}
