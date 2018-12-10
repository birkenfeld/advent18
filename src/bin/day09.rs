use advtools::prelude::VecDeque;
use advtools::input::iter_input_parts;

fn play(players: usize, last: u32) -> u32 {
    let mut scores = vec![0; players];
    // This is a pretty obvious implementation of the game.  We use a deque, and
    // the "current" marble is always the last item in the back, so to shift
    // the current, we rotate items between front and back.
    let mut marbles = VecDeque::with_capacity(last as usize);
    marbles.push_front(0);
    for (m, p) in (1..=last).zip((0..players).cycle()) {
        if m % 23 == 0 {
            for _ in 0..7 {
                let t = marbles.pop_back().unwrap();
                marbles.push_front(t);
            }
            scores[p] += m + marbles.pop_back().unwrap();
            let t = marbles.pop_front().unwrap();
            marbles.push_back(t);
        } else {
            let t = marbles.pop_front().unwrap();
            marbles.push_back(t);
            marbles.push_back(m);
        }
    }
    scores.into_iter().max().unwrap()
}

fn main() {
    let (players, last) = iter_input_parts([0, 6]).next().unwrap();
    advtools::print("Normal", play(players, last));
    advtools::print("100x longer", play(players, 100*last));
}
