use advtools::prelude::Itertools;
use advtools::input::{input_string, to_i32};

const SIZE: usize = 300;

fn power(x: usize, y: usize, serial: i32) -> i32 {
    let rid = (x as i32) + 10;
    let pow = (rid*(y as i32) + serial) * rid;
    ((pow / 100) % 10) - 5
}

fn main() {
    let serial = to_i32(input_string().trim());

    // Create a table with 2-D cumulative sums of powers, i.e.
    // [a b]     [a   a+b    ]
    // [c d]  => [a+c a+b+c+d]
    // We add a row and a column with zeroes at the top/left to aid
    // in calculation and because the problem indices are 1-based.
    let mut table = vec![vec![0; SIZE+1]];
    for y in 1..SIZE+1 {
        table.push((0..SIZE+1).scan(0, |cumulative, x| {
            *cumulative += if x == 0 { 0 } else {
                power(x, y, serial) + table[y-1][x] - table[y-1][x-1]
            };
            Some(*cumulative)
        }).collect());
    }

    // Now calculating a sum of entries in a rectangle is reduced to a
    // simple addition/subtraction of four entries of the table.
    let maxima = (2..=SIZE).flat_map(|sz| {
        (1..=SIZE-sz+1).cartesian_product(1..=SIZE-sz+1).map(|(x, y)| {
            (table[y-1][x-1] + table[y+sz-1][x+sz-1]
             - table[y-1][x+sz-1] - table[y+sz-1][x-1],
             x, y, sz)
        }).max()
    }).collect_vec();

    advtools::verify("Highest 3x3 power at", format!("{},{}", maxima[1].1, maxima[1].2), "20,77");

    let max = maxima.into_iter().max().unwrap();
    advtools::verify("Highest power at", format!("{},{},{}", max.1, max.2, max.3), "143,57,10");
}
