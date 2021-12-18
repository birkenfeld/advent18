use advtools::prelude::HashSet;
use advtools::input;
use advtools::grid::{Grid, Pos, Dir, Dir::*};

struct Cart {
    id: usize,
    pos: Pos<usize>,
    dir: Dir,
    turn: u8,
}

fn main() {
    // Collect list of carts, and a map of tracks and current occupation.
    let mut carts = vec![];
    let mut map = Grid::new(input::raw_string().lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            let id = carts.len() + 1;
            let pos = Pos(x, y);
            match c {
                '^' => { carts.push(Cart { id, pos, dir: U, turn: 0 }); ('|', id) }
                'v' => { carts.push(Cart { id, pos, dir: D, turn: 0 }); ('|', id) }
                '<' => { carts.push(Cart { id, pos, dir: L, turn: 0 }); ('-', id) }
                '>' => { carts.push(Cart { id, pos, dir: R, turn: 0 }); ('-', id) }
                ch  => { (ch, 0) }
            }
        }).collect()
    }));

    let mut first_collided = false;
    loop {
        let mut prune = HashSet::new();

        for cart in carts.iter_mut() {
            // If cart is already "gone", don't process it.
            if prune.contains(&cart.id) { continue; }

            // Remove cart from occupation map, and move it along one square.
            map[cart.pos].1 = 0;
            cart.pos.step(cart.dir);
            // If we have a cart already on the new square, a collision has
            // occurred, and we need to remove both carts.
            let (track, cur_cart) = &mut map[cart.pos];
            if *cur_cart != 0 {
                if !first_collided {
                    advtools::verify("First collision", cart.pos, "76,108");
                    first_collided = true;
                }
                prune.insert(cart.id);
                prune.insert(std::mem::replace(cur_cart, 0));
                continue;
            }
            *cur_cart = cart.id;
            // Decide if the cart needs to switch direction.
            match *track {
                '/'  => cart.dir = cart.dir.ur_dl(),
                '\\' => cart.dir = cart.dir.ul_dr(),
                '+'  => {
                    match cart.turn {
                        0 => cart.dir = cart.dir.left(),
                        2 => cart.dir = cart.dir.right(),
                        _ => (),
                    }
                    cart.turn = (cart.turn + 1) % 3;
                }
                _ => (),
            }
        }

        // Remove carts that should be removed.
        carts.retain(|c| !prune.contains(&c.id));
        if carts.len() == 1 {
            let cart = carts.into_iter().next().unwrap();
            advtools::verify("Remaining cart", cart.pos, "2,84");
            return;
        }

        // Make sure the carts are always processed in the right
        // order (top -> down, left -> right).
        carts.sort_by_key(|c| (c.pos.y, c.pos.x));
    }
}
