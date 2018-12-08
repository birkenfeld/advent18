## Advent of Code 2018

These are Rust-language solutions for the
[coding-challenge advent calendar](http://adventofcode.com/2018).  You'll
need stable Rust 1.31 and Cargo to run.

I've tried to make the solutions small and somewhat optimized for speed (so far,
no solution takes more than about a second on an up-to-date machine).  Inputs
that are larger than a few lines are included in text file form and parsed.

There are a few external crates commonly used as dependencies, such as regex,
itertools, rayon.  A custom helper library is included, called `advtools`,
mostly for easily parsing the input files, which I don't want to rewrite
each year.
