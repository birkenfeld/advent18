use advtools::prelude::{HashSet, Itertools};
use advtools::input::input_string;
use std::cell::Cell;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(i32, i32);

impl std::ops::Add for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Square {
    Elf,
    Goblin,
    Wall,
    Empty,
}
use self::Square::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Player {
    loc: Cell<Pos>,
    elf: bool,
    hp: Cell<i32>,
}

impl Player {
    fn new(elf: bool, y: usize, x: usize) -> Self {
        Self { elf, loc: Cell::new(Pos(y as i32, x as i32)), hp: Cell::new(200) }
    }
    fn hates(&self, v: Square) -> bool {
        (self.elf && v == Goblin) || (!self.elf && v == Elf)
    }
}

#[derive(Clone)]
struct Map<T>(Vec<Vec<T>>);

impl<T: Copy> Map<T> {
    fn set(&mut self, Pos(y, x): Pos, t: T) {
        self.0[y as usize][x as usize] = t;
    }
    fn get(&self, Pos(y, x): Pos) -> T {
        self.0[y as usize][x as usize]
    }
}


const DIRECTIONS: [Pos; 4] = [Pos(-1, 0), Pos(0, -1), Pos(0, 1), Pos(1, 0)];

// Find targets for a player using BFS.  We return a list of candidates, sorted
// by reading order, and also the direction for the first step if that target is
// selected.
fn find_targets(player: &Player, map: &Map<Square>) -> (i32, Vec<(Pos, Pos)>) {
    let mut positions = vec![(1, None, player.loc.get())];
    let mut seen = HashSet::with_capacity(1000);
    seen.insert(player.loc.get());
    let mut targets = vec![];
    let mut min = i32::max_value();

    loop {
        let seen_count = seen.len();
        let mut new_positions = vec![];
        for (steps, first, pos) in positions {
            for &delta in &DIRECTIONS {
                let new_pos = pos + delta;
                let first = first.or_else(|| Some(delta));
                if seen.insert(new_pos) {
                    match map.get(new_pos) {
                        Empty => if steps < min {
                            new_positions.push((steps + 1, first, new_pos))
                        }
                        v if player.hates(v) => {
                            targets.push((steps, new_pos, first.unwrap()));
                            min = min.min(steps)
                        }
                        _ => {}
                    }
                }
            }
        }
        positions = new_positions;
        if seen.len() == seen_count {
            return (min, targets.into_iter().filter(|c| c.0 == min)
                                            .map(|c| (c.1, c.2)).sorted().collect());
        }
    }
}

fn main() {
    let goblin_attack = 3;

    let mut starting_elves = 0;
    let mut new_players = vec![];
    let new_map = Map(input_string().lines().enumerate().map(|(y, line)| {
        line.trim().chars().enumerate().map(|(x, c)| match c {
            '#' => Wall,
            '.' => Empty,
            'G' => { new_players.push(Player::new(false, y, x)); Goblin }
            'E' => { new_players.push(Player::new(true,  y, x)); starting_elves += 1; Elf }
            _ => unreachable!()
        }).collect_vec()
    }).collect_vec());

    for elf_attack in 3.. {
        let mut map = new_map.clone();
        let mut players = new_players.clone();

        for mut round in 1.. {
            for player in &players {
                // Player doesn't exist anymore.
                if player.hp.get() <= 0 { continue; }

                // If there are no targets anymore, the round wasn't fully completed.
                if players.iter().filter(|p| p.hp.get() > 0 && p.elf != player.elf).count() == 0 {
                    round -= 1;
                    break;
                }

                let (mut dist, mut targets) = find_targets(&player, &map);
                if targets.is_empty() {
                    continue;
                }
                if dist > 1 {
                    // Closest target is distant. Move towards it.
                    let pos = player.loc.get();
                    let step = targets[0].1;
                    player.loc.set(pos + step);
                    map.set(pos, Empty);
                    map.set(pos + step, if player.elf { Elf } else { Goblin });

                    // If we were one tile away, we might be able to attack
                    // now. Recheck targets.
                    if dist == 2 {
                        let (new_dist, new_tgts) = find_targets(&player, &map);
                        dist = new_dist;
                        targets = new_tgts;
                    }
                }
                if dist == 1 {
                    // Closest target is adjacent. Attack!
                    // But we need to search again, since enemies are now sorted
                    // by HP in addition to coordinate.
                    let (enemy_hp, enemy_pos) = targets.into_iter().map(|(pos, _)| {
                        (&players.iter().find(|p| p.loc.get() == pos).unwrap().hp, pos)
                    }).sorted().next().unwrap();

                    enemy_hp.set(enemy_hp.get() - if player.elf { elf_attack } else { goblin_attack });
                    if enemy_hp.get() <= 0 {
                        map.set(enemy_pos, Empty);
                    }
                }
            }

            players.retain(|p| p.hp.get() > 0);
            players.sort();

            let elves = players.iter().filter(|p| p.elf).count();

            // First elf died, abandon attempt.
            if elves < starting_elves && elf_attack > 3 {
                break;
            }

            if elves == starting_elves && starting_elves == players.len() {
                advtools::print("Outcome where elves win",
                                players.iter().map(|p| p.hp.get()).sum::<i32>() as u32 * round);
                return;
            } else if elves == 0 || elves == players.len() {
                if elf_attack == 3 {
                    advtools::print("Outcome",
                                    players.iter().map(|p| p.hp.get()).sum::<i32>() as u32 * round);
                }
                break;
            }
        }
    }
}
