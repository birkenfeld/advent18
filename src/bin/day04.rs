use advtools::prelude::{Itertools, HashMap, ArrayVec};
use advtools::input;

const FORMAT: &str = r"\[1518-(\d+)-(\d+) (\d+):(\d+)\] (?:Guard #)?(falls|wakes|\d+)";

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    minute: u32,
    sleep: bool,
}

struct Day {
    guard: u32,
    events: ArrayVec<Event, 8>,
}

impl Default for Day {
    fn default() -> Self {
        Day { guard: 0, events: std::iter::once(Event::default()).collect() }
    }
}

struct SleepMinutes([u32; 60]);

impl SleepMinutes {
    /// Return (minute, sleep) of maximum sleep.
    fn max_minute(&self) -> (u32, u32) {
        self.0.iter().enumerate().max_by_key(|v| v.1).map(|(i, &n)| (i as u32, n)).unwrap()
    }
    fn total(&self) -> u32 {
        self.0.iter().sum()
    }
}

fn main() {
    let mut days = HashMap::<_, Day>::new();
    let mut asleep = HashMap::new();
    // First, collect the events for each day (needs a separate pass since
    // they are unsorted in the input).
    for line in input::rx_lines(FORMAT) {
        let (m, d, hh, mm, action): (u16, u16, i16, i16, &str) = line;
        // Correct the day and minute for dates before midnight.
        let (day, min) = if hh == 23 { (100*m + d + 1, mm - 60) } else { (100*m + d, mm) };
        let entry = days.entry(day).or_default();
        match action {
            "falls" => entry.events.push(Event { minute: min as u32, sleep: true }),
            "wakes" => entry.events.push(Event { minute: min as u32, sleep: false }),
            num     => entry.guard = num.parse().unwrap(),
        }
    }
    // Now, collect the "asleep minutes" for each guard over all days.
    for (_, Day { guard, mut events }) in days {
        events.sort();
        // With this and the Default impl for Day, we have framed the events
        // of each day between (0, false) and (60, false).
        events.push(Event { minute: 60, sleep: false });
        let entry = asleep.entry(guard).or_insert_with(|| SleepMinutes([0u32; 60]));
        // Now go through each pair of events and increase the "asleep" count
        // for the applicable minutes.
        for (ev1, ev2) in events.iter().tuple_windows() {
            if ev1.sleep {
                for min in ev1.minute..ev2.minute {
                    entry.0[min as usize] += 1;
                }
            }
        }
    }

    // Part 1: find the guard with maximum total sleep.
    // Then multiply with the minute it sleeps the most.
    let (guard, _) = asleep.iter().max_by_key(|(_, sleeps)| sleeps.total()).unwrap();
    advtools::verify("Guard*minute 1", guard * asleep[guard].max_minute().0, 125444);

    // Part 2: find the guard with the maximum sleep in a single minute.
    let ((min, _), guard) = asleep.iter().map(|(&guard, sleeps)| (sleeps.max_minute(), guard))
                                         .max_by_key(|((_, n), _)| *n).unwrap();
    advtools::verify("Guard*minute 2", guard * min, 18325);
}
