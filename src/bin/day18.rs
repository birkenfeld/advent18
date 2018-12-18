use advtools::prelude::{Itertools, HashMap};
use advtools::input::iter_input;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum State { Open, Wood, Yard }
use self::State::*;

fn neighbor_count(grid: &[Vec<State>], (py, px): (usize, usize), st: State) -> usize {
    [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
        .iter()
        .filter_map(move |(dy, dx)| {
            grid.get(((py as isize) + dy) as usize).and_then(
                |line| line.get(((px as isize) + dx) as usize))
        })
        .filter(|&&c| c == st)
        .count()
}

fn evolve(grid: &[Vec<State>]) -> Vec<Vec<State>> {
    grid.iter().enumerate().map(|(y, line)| {
        line.iter().enumerate().map(|(x, &cell)| match cell {
            Open => if neighbor_count(grid, (y, x), Wood) >= 3 { Wood } else { Open },
            Wood => if neighbor_count(grid, (y, x), Yard) >= 3 { Yard } else { Wood },
            Yard => if neighbor_count(grid, (y, x), Yard) >= 1 &&
                       neighbor_count(grid, (y, x), Wood) >= 1 { Yard } else { Open },
        }).collect()
    }).collect()
}

fn evaluate(grid: &[Vec<State>]) -> usize {
    let woods = grid.iter().map(|line| line.iter().filter(|&&c| c == Wood).count()).sum::<usize>();
    let yards = grid.iter().map(|line| line.iter().filter(|&&c| c == Yard).count()).sum::<usize>();
    woods * yards
}

fn main() {
    let mut grid = iter_input::<String>().map(|line| {
        line.chars().map(|ch| match ch {
            '.' => State::Open,
            '|' => State::Wood,
            '#' => State::Yard,
            _   => panic!("invalid state")
        }).collect_vec()
    }).collect_vec();

    for _ in 0..10 {
        grid = evolve(&grid);
    }
    advtools::print("Value after 10min", evaluate(&grid));

    let mut seen = HashMap::new();
    for min in 10.. {
        if seen.contains_key(&grid) {
            let rest = (1_000_000_000 - min) % (min - seen[&grid]);
            for _ in 0..rest {
                grid = evolve(&grid);
            }
            break;
        }
        seen.insert(grid.clone(), min);
        grid = evolve(&grid);
    }
    advtools::print("Value after 1bn min", evaluate(&grid));
}
