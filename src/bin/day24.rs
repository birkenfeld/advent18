use advtools::prelude::{Itertools, Regex, HashSet};
use advtools::input;
use once_cell::sync::Lazy;
use strum_macros::EnumString;
use std::cell::Cell;

static FORMAT: Lazy<Regex> = Lazy::new(|| Regex::new(concat!(
        r"(?P<units>\d+) units each with (?P<hp>\d+) hit points",
        r"(?: \((?P<modifiers>.*?)\))? with an attack that does ",
        r"(?P<dmg>\d+) (?P<dmgtype>\w+) damage at initiative (?P<init>\d+)"
    )).unwrap()
);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Side {
    ImmuneSystem,
    Infection,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, EnumString)]
#[strum(serialize_all="snake_case")]
enum DmgType {
    Fire,
    Cold,
    Slashing,
    Bludgeoning,
    Radiation,
}

#[derive(Clone, Debug)]
struct Group {
    side: Side,
    units: Cell<i32>,
    hp: i32,
    dmg: i32,
    init: i32,
    dmgtype: DmgType,
    weak: HashSet<DmgType>,
    immune: HashSet<DmgType>,
}

impl Group {
    fn parse(line: &str, side: Side) -> Group {
        let caps = FORMAT.captures(line).unwrap();
        let mods = caps.name("modifiers").map_or("", |s| s.as_str());
        let mut weak = HashSet::new();
        let mut immune = HashSet::new();
        for part in mods.split_terminator("; ") {
            if part.starts_with("weak to") {
                weak = part[8..].split(", ").map(|s| s.parse().unwrap()).collect();
            } else {
                immune = part[10..].split(", ").map(|s| s.parse().unwrap()).collect();
            }
        }
        Group {
            side, weak, immune,
            hp: input::to_i32(&caps["hp"]),
            dmg: input::to_i32(&caps["dmg"]),
            init: input::to_i32(&caps["init"]),
            units: Cell::new(input::to_i32(&caps["units"])),
            dmgtype: caps["dmgtype"].parse().unwrap(),
        }
    }
    fn eff_power(&self) -> i32 {
        self.units.get() * self.dmg
    }
    fn dmg_to(&self, other: &Group) -> i32 {
        let mut total_dmg = self.eff_power();
        if other.weak.contains(&self.dmgtype) { total_dmg *= 2; }
        if other.immune.contains(&self.dmgtype) { total_dmg = 0; }
        total_dmg
    }
}

fn fight(mut groups: Vec<Group>) -> (Option<Side>, i32) {
    let mut total_power = 0;
    while groups.iter().map(|g| g.side).collect::<HashSet<_>>().len() == 2 {
        // Target selection
        let mut targets = vec![];
        let mut targeted = HashSet::new();
        groups.sort_by_key(|g| (g.eff_power(), g.init));
        for group in groups.iter().rev() {
            let candidates = groups.iter()
                .filter(|target| target.side != group.side &&
                        !targeted.contains(&target.hp) &&
                        group.dmg_to(target) > 0)
                .sorted_by_key(|target| (group.dmg_to(target), target.eff_power(),
                                         target.init));
            if let Some(target) = candidates.last() {
                targets.push((group, target));
                targeted.insert(target.hp);
            }
        }
        // Deal damage
        for (atk, def) in targets.iter().sorted_by_key(|(a, _)| -a.init) {
            def.units.set((def.units.get() - atk.dmg_to(def) / def.hp).max(0));
        }
        // Throw out dead units
        groups.retain(|g| g.units.get() > 0);
        // Detect endless fights where no units are ever killed
        let power = groups.iter().map(|g| g.eff_power()).sum::<i32>();
        if power == total_power {
            return (None, 0);
        }
        total_power = power;
    }
    (Some(groups[0].side), groups.iter().map(|g| g.units.get()).sum())
}

fn main() {
    let mut input = input::lines();
    let mut groups = input.by_ref().skip(1)
                                   .take_while(|&line| line != "Infection:")
                                   .map(|s| Group::parse(s, Side::ImmuneSystem))
                                   .collect_vec();
    groups.extend(input.map(|s| Group::parse(s, Side::Infection)));

    let winning = fight(groups.clone()).1;
    advtools::verify("Units in winning army", winning, 16678);

    for boost in 1.. {
        let groups = groups.iter().cloned().map(|mut group| {
            if group.side == Side::ImmuneSystem { group.dmg += boost; }
            group
        }).collect();
        if let (Some(Side::ImmuneSystem), winning) = fight(groups) {
            advtools::verify("Units in immune system after boost", winning, 3758);
            return;
        }
    }
}
