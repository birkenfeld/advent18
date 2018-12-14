use advtools::prelude::{Itertools, HashSet};
use advtools::input::input_string;

enum Dir { U, D, L, R }
use self::Dir::*;

impl Dir {
    fn left(&self)  -> Self { match self { U => L, L => D, D => R, R => U } }
    fn right(&self) -> Self { match self { U => R, R => D, D => L, L => U } }
    fn ul_dr(&self) -> Self { match self { U => L, R => D, D => R, L => U } }
    fn ur_dl(&self) -> Self { match self { U => R, R => U, D => L, L => D } }
}

struct Cart {
    id: usize,
    x: usize,
    y: usize,
    dir: Dir,
    turn: u8,
}

fn main() {
    // Collect list of carts, and a map of tracks and current occupation.
    let mut carts = vec![];
    let mut map = input_string().lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            let id = carts.len() + 1;
            match c {
                '^' => { carts.push(Cart { id, x, y, dir: U, turn: 0 }); ('|', id) }
                'v' => { carts.push(Cart { id, x, y, dir: D, turn: 0 }); ('|', id) }
                '<' => { carts.push(Cart { id, x, y, dir: L, turn: 0 }); ('-', id) }
                '>' => { carts.push(Cart { id, x, y, dir: R, turn: 0 }); ('-', id) }
                ch  => { (ch, 0) }
            }
        }).collect_vec()
    }).collect_vec();

    let mut first_collided = false;
    loop {
        let mut prune = HashSet::new();

        for cart in carts.iter_mut() {
            // If cart is already "gone", don't process it.
            if prune.contains(&cart.id) { continue; }

            // Remove cart from occupation map, and move it along one square.
            map[cart.y][cart.x].1 = 0;
            match cart.dir {
                U => cart.y -= 1,
                D => cart.y += 1,
                L => cart.x -= 1,
                R => cart.x += 1
            }
            // If we have a cart already on the new square, a collision has
            // occurred, and we need to remove both carts.
            let (track, cur_cart) = &mut map[cart.y][cart.x];
            if *cur_cart != 0 {
                if !first_collided {
                    advtools::print("First collision", format!("{},{}", cart.x, cart.y));
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
            return advtools::print("Remaining cart", format!("{},{}", cart.x, cart.y));
        }

        // Make sure the carts are always processed in the right
        // order (top -> down, left -> right).
        carts.sort_by_key(|c| (c.y, c.x));
    }
}
