use advtools::prelude::Itertools;
use advtools::input::{input_string, to_usize};

fn reduce<I, F>(it: &mut I, combine: &F) -> usize
where I: Iterator<Item=usize>, F: Fn(&[usize], std::iter::Take<&mut I>) -> usize
{
    let (children, meta) = it.next_tuple().unwrap();
    let child_results = (0..children).map(|_| reduce(it, combine)).collect_vec();
    combine(&child_results, it.take(meta))
}

fn main() {
    let input = input_string();
    // Since we need to iterate twice over the same data, clone the iterator.
    let mut iter1 = input.split_whitespace().map(to_usize);
    let mut iter2 = iter1.clone();

    // Part 1: sum up metadata for all nodes.
    let p1 = reduce(&mut iter1, &|child, meta| child.iter().sum::<usize>() + meta.sum::<usize>());
    advtools::verify("Sum of metadata", p1, 36891);
    // Part 2: get the "value" of the root node.
    let p2 = reduce(&mut iter2, &|child, meta| {
        if child.is_empty() { meta.sum() } else { meta.filter_map(|i| child.get(i - 1)).sum() }
    });
    advtools::verify("Value of root", p2, 20083);
}
