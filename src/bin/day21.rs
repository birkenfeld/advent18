use advtools::prelude::HashSet;

// Run the algorithm, translated from the assembly.
fn run() -> (u32, u32) {
    let (mut first, mut last) = (0, 0);
    let mut seen = HashSet::new();

    // The "sanity check" in instructions 0-5 is omitted here.
    let mut r3 = 0;
    let mut r1;
    'outer: loop {
        // Instructions 6-7.
        r1 = r3 | 0x10000;
        r3 = 9450265;
        loop {
            // Instructions 8-12.
            r3 = (r3 + (r1 & 0xff)) & 0xffffff;
            r3 = (r3 * 65899) & 0xffffff;

            // Instructions 13-16, 28-30.
            if r1 < 256 {
                // Here, the comparison with r0 is made.
                // If `seen` is empty, it's the first time we can try.
                if seen.is_empty() {
                    first = r3;
                }
                // If we have seen this value already, the previous one
                // is the final one since after that, values will repeat.
                if !seen.insert(r3) {
                    return (first, last);
                }
                last = r3;
                continue 'outer;
            }

            // Instructions 17-27.
            r1 >>= 8;
        }
    }
}

fn main() {
    let (first, last) = run();
    advtools::print("r0 for fewest instructions", first);
    advtools::print("r0 for most instructions", last);
}
