use advtools::input::input_string;

fn reacts(a: char, b: char) -> bool {
    a != b && a.eq_ignore_ascii_case(&b)
}

fn reduce(polymer: &str, skip: Option<char>) -> usize {
    // One pass over the input is enough, if we always keep track if the last
    // pushed and the new unit react.
    polymer.chars().fold(vec!(), |mut stack, ch| match stack.last() {
        _ if skip == Some(ch.to_ascii_lowercase()) => stack,
        Some(&pch) if reacts(pch, ch) => { stack.pop(); stack }
        _                             => { stack.push(ch); stack }
    }).len()
}

fn main() {
    let input = input_string();
    let polymer = input.trim();
    // Part 1: reduce input as is.
    println!("Remaining length: {}", reduce(polymer, None));

    // Part 2: reduce after removal of a specific pair, find the minimum.
    let min_len = (b'a'..=b'z').map(|c| reduce(polymer, Some(c as char))).min();
    println!("Remaining length after removal: {}", min_len.unwrap());
}
