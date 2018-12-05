use advtools::input::input_string;

fn reacts(a: char, b: char) -> bool {
    a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

fn reduce(polymer: &str, without: Option<char>) -> usize {
    let mut output = Vec::new();
    // One pass over the input is enough, if we always keep track if the
    // last pushed and the new unit react.
    for ch in polymer.chars() {
        if Some(ch.to_ascii_lowercase()) == without {
            // ignore this
        } else if reacts(ch, output.last().cloned().unwrap_or_default()) {
            output.pop();
        } else {
            output.push(ch);
        }
    }
    output.len()
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
