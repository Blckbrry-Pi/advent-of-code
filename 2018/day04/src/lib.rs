use std::{fmt::Write, str::FromStr};

aoc_tools::aoc_sol!(day04 2018: part1, part2);

fn sort_events(mut events: Vec<Event>) -> Vec<Event> {
    events.sort();
    events
}
fn shift_iter(events: &[Event]) -> impl Iterator<Item = (u16, Shift)> + '_ {
    Event::shifts(events).map(|event_subslice| Shift::from_events(event_subslice).unwrap())
}
fn guards(shift_iter: impl Iterator<Item = (u16, Shift)>) -> HashMap<u16, Vec<Shift>> {
    let mut guards: HashMap<u16, Vec<Shift>> = HashMap::new();
    shift_iter.for_each(|(id, shift)| guards.entry(id).or_default().push(shift));
    guards
}
fn method_1(guards: &HashMap<u16, Vec<Shift>>) -> u32 {
    let (guard, (_, min_sums)) = guards.into_iter()
        .map(|(id, shifts)| {
            let min_sums = Shift::sum(shifts.iter());
            let sum_awake = min_sums.iter().sum::<u16>();
            let sum_asleep = shifts.len() as u16 * 60 - sum_awake;
            (*id, (sum_asleep, min_sums))
        })
        .max_by_key(|(_, (sum, _))| *sum)
        .unwrap();

    let (min, _count) = min_sums
        .into_iter()
        .enumerate()
        .min_by_key(|(_, count)| *count)
        .unwrap();

    guard as u32 * min as u32
}
fn method_2(guards: &HashMap<u16, Vec<Shift>>) -> u32 {
    let (guard, (_, min_sums)) = guards.into_iter()
        .map(|(id, shifts)| {
            let min_sums = Shift::sum(shifts.iter());
            let min_frequency = min_sums.into_iter().min().unwrap();
            let freq_asleep = shifts.len() as u16 - min_frequency;
            (*id, (freq_asleep, min_sums))
        })
        .max_by_key(|(_, (freq_asleep, _))| *freq_asleep)
        .unwrap();

    let (min, _count) = min_sums
        .into_iter()
        .enumerate()
        .min_by_key(|(_, count)| *count)
        .unwrap();

    guard as u32 * min as u32
}

pub fn part1(input: &str) -> u32 {
    let events = parse_input(input);
    let guards = guards(shift_iter(&sort_events(events)));
    method_1(&guards)
}

pub fn part2(input: &str) -> u32 {
    let events = parse_input(input);
    let guards = guards(shift_iter(&sort_events(events)));
    method_2(&guards)
}


#[derive(Clone, Copy, PartialEq, Eq)]
struct Time {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    min: u8,
}
impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.year.cmp(&other.year)
            .then(self.month.cmp(&other.month))
            .then(self.day.cmp(&other.day))
            .then(self.hour.cmp(&other.hour))
            .then(self.min.cmp(&other.min))
    }
}
impl FromStr for Time {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || Err("Invalid time format".to_string());
        let Some((year,  rest)) = s.split_once('-') else { return err() };
        let Some((month, rest)) = rest.split_once('-') else { return err() };
        let Some((day,   rest)) = rest.split_once(' ') else { return err() };
        let Some((hour,  min )) = rest.split_once(':') else { return err() };
        let year  = year .parse::<u16>().map_err(|e| e.to_string())?;
        let month = month.parse::<u8>().map_err(|e| e.to_string())?;
        let day   = day  .parse::<u8>().map_err(|e| e.to_string())?;
        let hour  = hour .parse::<u8>().map_err(|e| e.to_string())?;
        let min   = min  .parse::<u8>().map_err(|e| e.to_string())?;
        Ok(Self { year, month, day, hour, min })
    }
}
impl Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.min,
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Event {
    Start(u16, Time),
    Wake(Time),
    Sleep(Time),
}
impl Event {
    pub fn is_start(&self) -> bool {
        matches!(self, Self::Start(_, _))
    }
    pub fn shifts<'a>(events: &'a [Self]) -> impl Iterator<Item = &'a [Self]> {
        (0..events.len())
            .filter(|&i| events[i].is_start())
            .map(|i| &events[i..])
            .map(|list| {
                let mut end = 1;
                while end < list.len() && !list[end].is_start() {
                    end += 1;
                }
                &list[..end]
            })

    }
}
impl Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start(id, time) => write!(f, "[{time:?}] Guard #{id} starts shift"),
            Self::Wake (time) => write!(f, "[{time:?}] wakes up"),
            Self::Sleep(time) => write!(f, "[{time:?}] falls asleep"),
        }
    }
}
impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (
            | Self::Start(_, time_s)
            | Self::Wake (time_s)
            | Self::Sleep(time_s),
            | Self::Start(_, time_o)
            | Self::Wake (time_o)
            | Self::Sleep(time_o),
        ) = (self, other);
        let time_cmp = time_s.cmp(time_o);
        if time_cmp.is_ne() { return time_cmp }
        match (self, other) {
            (Self::Start(id_s, _), Self::Start(id_o, _)) => id_s.cmp(id_o),
            (Self::Start(_, _), _) => std::cmp::Ordering::Less,
            (_, Self::Start(_, _)) => std::cmp::Ordering::Greater,
            (Self::Wake(_), Self::Wake(_)) => std::cmp::Ordering::Equal,
            (Self::Wake(_), _) => std::cmp::Ordering::Less,
            (_, Self::Wake(_)) => std::cmp::Ordering::Greater,
            (Self::Sleep(_), Self::Sleep(_)) => std::cmp::Ordering::Equal,
        }
    }
}

struct Shift([bool; 60]);
impl Shift {
    pub fn from_events(events: &[Event]) -> Result<(u16, Self), String> {
        let Event::Start(id, _) = events[0] else {
            return Err("The first event should be a shift start".to_string());
        };
        let mut curr_min = 0;
        let mut awake = true;
        let mut output = [false; 60];
        for i in 1..events.len() {
            match (awake, events[i]) {
                (_, Event::Start(id2, _)) => return Err(format!("List contained multiple start events (guards {id} & {id2})")),
                (true,  Event::Wake (Time { min, .. })) => return Err(format!("Guard cannot wake  at 00:{min:02} while already awake  at 00:{curr_min:02}")),
                (false, Event::Sleep(Time { min, .. })) => return Err(format!("Guard cannot sleep at 00:{min:02} while already asleep at 00:{curr_min:02}")),
                (true, Event::Sleep(Time { min, .. })) | (false, Event::Wake(Time { min, .. })) => {
                    for m in curr_min..min {
                        output[m as usize] = awake;
                    }
                    curr_min = min;
                    awake = !awake;
                }
            }
        }
        if curr_min < 60 {
            for m in curr_min..60 {
                output[m as usize] = awake;
            }
        }
        Ok((id, Self(output)))
    }
    pub fn sum<'a>(shifts: impl Iterator<Item = &'a Shift>) -> [u16; 60] {
        let mut counts = [0; 60];
        for shift in shifts {
            for i in 0..60 {
                counts[i] += shift.0[i] as u16;
            }
        }
        counts
    }
}
impl Debug for Shift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..60 {
            f.write_char(if self.0[i] { '.' } else { '#' })?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> Vec<Event> {
    input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (time, action) = l.split_once("] ").unwrap();
            let time = time.strip_prefix('[').unwrap();
            let time = time.parse().unwrap();
            match action {
                "falls asleep" => Event::Sleep(time),
                "wakes up" => Event::Wake(time),
                shift_beginning => {
                    let rest = shift_beginning.strip_prefix("Guard #").unwrap();
                    let id = rest.strip_suffix(" begins shift").unwrap();
                    let id = id.parse().unwrap();
                    Event::Start(id, time)
                },
            }
        })
        .collect()
}
