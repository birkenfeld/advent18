use advtools::input::iter_input_parts;
use dlv_list::VecList;

fn play(players: usize, last: usize) -> usize {
    let mut scores = vec![0; players];
    // This is a pretty obvious implementation of the game.
    // The VecList datastructure is a linked list-like array that lets
    // us insert and remove at arbitrary points pretty cheaply.
    let mut marbles = VecList::with_capacity(last);
    let mut ix = marbles.push_front(0);
    for (m, p) in (1..=last).zip((0..players).cycle()) {
        if m % 23 == 0 {
            for _ in 0..6 {
                // Indices need to "roll over" when we arrive at the
                // front of the list.
                ix = marbles.get_previous_index(ix)
                    .or_else(|| marbles.indices().next_back()).unwrap();
            }
            let remove_ix = marbles.get_previous_index(ix).unwrap();
            scores[p] += m + marbles.remove(remove_ix).unwrap();
        } else {
            ix = marbles.get_next_index(ix)
                .or_else(|| marbles.indices().next()).unwrap();
            ix = marbles.insert_after(ix, m);
        }
    }
    scores.into_iter().max().unwrap()
}

fn main() {
    let (players, last) = iter_input_parts((0, 6)).next().unwrap();
    advtools::print("Normal", play(players, last));
    advtools::print("100x longer", play(players, 100*last));
}
