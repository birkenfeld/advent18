use advtools::prelude::Itertools;
use advtools::input;

fn main() {
    let number = input::parse();
    let digits = input::chars().map(|d| d.to_digit(10).unwrap() as u8).collect_vec();
    let ntarget = digits.len();

    let mut stack = vec![3u8, 7];
    let mut pos = (0, 1);
    let mut ten_scores_done = false;

    loop {
        let scores = (stack[pos.0], stack[pos.1]);
        let mut sum_scores = scores.0 + scores.1;
        if sum_scores >= 10 {
            stack.push(1);
            sum_scores -= 10;
        }
        stack.push(sum_scores);

        let n = stack.len();
        pos.0 = (pos.0 + 1 + scores.0 as usize) % n;
        pos.1 = (pos.1 + 1 + scores.1 as usize) % n;

        if !ten_scores_done && n >= number + 10 {
            let ten_scores = stack[number..number+10].iter().format("");
            advtools::verify("Ten scores", ten_scores, 9411137133_u64);
            ten_scores_done = true;
        }

        if let Some(i) = stack.windows(ntarget).rev().take(2).position(|p| p == digits) {
            advtools::verify("Positions", n - ntarget - i, 20317612);
            return;
        }
    }
}
