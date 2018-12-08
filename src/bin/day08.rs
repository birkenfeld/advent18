use advtools::prelude::Itertools;
use advtools::input::{input_string, to_usize};

fn sum_meta(it: &mut impl Iterator<Item=usize>) -> usize {
    let (children, meta) = it.next_tuple().unwrap();
    (0..children).map(|_| sum_meta(it)).sum::<usize>() +
        it.take(meta).sum::<usize>()
}

fn get_value(it: &mut impl Iterator<Item=usize>) -> usize {
    let (children, meta) = it.next_tuple().unwrap();
    if children == 0 {
        it.take(meta).sum()
    } else {
        // Meta comes after children, so we need to determine the value
        // for all children first.
        let child_vals = (0..children).map(|_| get_value(it)).collect_vec();
        it.take(meta).filter(|&i| 1 <= i && i <= children)
                     .map(|i| child_vals[i - 1]).sum()
    }
}

fn main() {
    let input = input_string();
    let mut input_iter = input.split_whitespace().map(to_usize);

    // Part 1: sum up metadata for all nodes.
    println!("Sum of metadata: {}", sum_meta(&mut input_iter.clone()));
    // Part 2: get the "value" of the root node.
    println!("Value of root: {}", get_value(&mut input_iter));
}
