use advtools::prelude::{HashSet, Itertools};
use advtools::input;

fn sum_pots(state: &[bool], first: i64) -> i64 {
    (first..).zip(state).map(|(i, &b)| i * b as i64).sum()
}

fn main() {
    let mut iter = input::parse_lines::<Vec<&str>>();

    let initial_str = iter.next().unwrap()[2];
    // Parse initial state into a vec of bools.  Since we don't have to simulate
    // many iterations, this is ok, otherwise a vector of bits, packed into a
    // u128, would be much more efficient.
    let mut state = initial_str.chars().map(|c| c == '#').collect_vec();
    // This is the pot index of the first bit in the vector.
    let mut first_pot = 0;

    // Parse rules for new plants into a set of [bool].
    let rules: HashSet<Box<[_]>> = iter.filter(|x| x[2] == "#").map(|x| {
        x[0].chars().map(|c| c == '#').collect()
    }).collect();

    for generation in 0.. {
        // Part 1 completed?
        if generation == 20 {
            advtools::verify("Sum after 20", sum_pots(&state, first_pot), 3276);
        }
        // Make sure we have enough empty pots at the beginning and the end
        // of the vector.  Inserting at the beginning is not optimal, but
        // again, due to the limited number of steps it's not noticeable.
        while state[..3].iter().any(|&x| x) {
            state.insert(0, false);
            first_pot -= 1;
        }
        while state[state.len()-3..].iter().any(|&x| x) {
            state.push(false);
        }
        // Calculate new generation based on 5-len windows in to the old.
        let new_state = state.windows(5).map(|w| rules.contains(w)).collect_vec();
        // If the new generation is the same as the old, just shifted by some
        // amount, we have reached a steady state and can exit the simulation.
        // The effect of the remaining steps on the pot sum is just N * shift.
        if let Some((idx, _)) = state.windows(new_state.len())
                                     .enumerate().find(|(_, w)| *w == new_state)
        {
            let total_shift = (50_000_000_000 - generation) * idx as i64;
            advtools::verify("Sum after 50 billion",
                             sum_pots(&state, first_pot + total_shift), 3750000001113_u64);
            return;
        }
        // The windows() method threw away two entries from the beginning (and
        // the end) of the vector, compensate for that.
        first_pot += 2;
        state = new_state;
    }
}
