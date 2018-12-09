## Advent of Code 2018

These are Rust-language solutions for the [coding-challenge advent
calendar](http://adventofcode.com/2018).  You'll need stable Rust 1.31 and Cargo
to run.

I've tried to make the solutions small and somewhat optimized for speed (so far,
no solution takes more than about a second on an up-to-date machine).  Inputs
are included in text file form and parsed.

A custom helper library is used, called `advtools`.  It provides utilities for
easily parsing the input files, which I don't want to rewrite each year, and
access to often used external crates like itertools and rayon.

For tasks that require nontrivial datastructures or algorithms, I try to find
and use a third-party crate to show off the ease of using Rust's crates
infrastructures, e.g. `petgraph`.
