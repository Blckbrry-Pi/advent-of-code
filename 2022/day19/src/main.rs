use std::{collections::HashMap, fmt::Debug, num::ParseIntError, str::FromStr};



fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../../data/2022/day19/test.txt");
const INPUT: &str = include_str!("../../../data/2022/day19/input.txt");

fn part1() {
    let blueprints = parse_input(TEST);
    for blueprint in blueprints {
        println!("{} maxes at {} geodes", blueprint.id, blueprint.most_geodes(State::part1(), &mut HashMap::new()));
    }
    println!("Part 1: ");
}

fn part2() {
    let _ = parse_input(TEST);
    println!("Part 2: ");
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Blueprint {
    id: u8,
    ore_robot_ore: u8,
    clay_robot_ore: u8,
    obsidian_robot_ore: u8,
    obsidian_robot_clay: u8,
    geode_robot_ore: u8,
    geode_robot_obsidian: u8,
}
impl Blueprint {
    pub fn most_geodes(&self, state: State, cache: &mut HashMap<State, u32>) -> u32 {
        if state.minutes_left == 0 { return state.geodes }
        if let Some(&result) = cache.get(&state) {
            return result;
        }
        let mut can_do_everything = true;
        let mut max = 0;
        if state.ore >= self.ore_robot_ore as u32 {
            let mut new_state = state;
            new_state.ore -= self.ore_robot_ore as u32;
            new_state.advance();
            new_state.ore_robots += 1;
            max = max.max(self.most_geodes(new_state, cache));
        } else {
            can_do_everything = false;
        }
        if state.ore >= self.clay_robot_ore as u32 {
            let mut new_state = state;
            new_state.ore -= self.clay_robot_ore as u32;
            new_state.advance();
            new_state.clay_robots += 1;
            max = max.max(self.most_geodes(new_state, cache));
        } else {
            can_do_everything = false;
        }
        if state.ore > self.obsidian_robot_ore as u32 && state.clay > self.obsidian_robot_clay as u32 {
            let mut new_state = state;
            new_state.ore -= self.obsidian_robot_ore as u32;
            new_state.clay -= self.obsidian_robot_clay as u32;
            new_state.advance();
            new_state.obsidian_robots += 1;
            max = max.max(self.most_geodes(new_state, cache));
        } else {
            can_do_everything = false;
        }
        if state.ore > self.geode_robot_ore as u32 && state.obsidian > self.geode_robot_obsidian as u32 {
            let mut new_state = state;
            new_state.ore -= self.geode_robot_ore as u32;
            new_state.obsidian -= self.geode_robot_obsidian as u32;
            new_state.advance();
            new_state.geode_robots += 1;
            max = max.max(self.most_geodes(new_state, cache));
        } else {
            can_do_everything = false;
        }

        if !can_do_everything {
            let mut new_state = state;
            new_state.advance();
            max = max.max(self.most_geodes(new_state, cache));
        }

        max
    }
}
impl Debug for Blueprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Blueprint {}:", self.id)?;
        writeln!(f, "    Each ore robot costs {} ore", self.ore_robot_ore)?;
        writeln!(f, "    Each clay robot costs {} ore", self.clay_robot_ore)?;
        writeln!(f, "    Each obsidian robot costs {} ore and {} clay", self.obsidian_robot_ore, self.obsidian_robot_clay)?;
        write!(f, "    Each geode robot costs {} ore and {} obsidian", self.geode_robot_ore, self.geode_robot_obsidian)
    }
}
impl FromStr for Blueprint {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix("Blueprint ") else {
            return Err("Invalid prefix".to_string());
        };
        let Some((id, s)) = s.split_once(": Each ore robot costs ") else {
            return Err("Colon section error".to_string());
        };
        let Some((ore_robot_ore, s)) = s.split_once(" ore. Each clay robot costs ") else {
            return Err("\"Each Clay Robot Costs\" section error".to_string());
        };
        let Some((clay_robot_ore, s)) = s.split_once(" ore. Each obsidian robot costs ") else {
            return Err("\"Each Obsidian Robot Costs\" section error".to_string());
        };
        let Some((obsidian_robot_ore, s)) = s.split_once(" ore and ") else {
            return Err("Obsidian \"Ore And\" section error".to_string());
        };
        let Some((obsidian_robot_clay, s)) = s.split_once(" clay. Each geode robot costs ") else {
            return Err("\"Each Geode Robot Costs\" section error".to_string());
        };
        let Some((geode_robot_ore, s)) = s.split_once(" ore and ") else {
            return Err("Geode \"Ore And\" section error".to_string());
        };
        let Some(geode_robot_obsidian) = s.strip_suffix(" obsidian.") else {
            return Err("Invalid suffix".to_string());
        };

        let (
            id,
            ore_robot_ore,
            clay_robot_ore,
            obsidian_robot_ore,
            obsidian_robot_clay,
            geode_robot_ore,
            geode_robot_obsidian,
        ) = (
            id.parse().map_err(|e: ParseIntError| e.to_string())?,
            ore_robot_ore.parse().map_err(|e: ParseIntError| e.to_string())?,
            clay_robot_ore.parse().map_err(|e: ParseIntError| e.to_string())?,
            obsidian_robot_ore.parse().map_err(|e: ParseIntError| e.to_string())?,
            obsidian_robot_clay.parse().map_err(|e: ParseIntError| e.to_string())?,
            geode_robot_ore.parse().map_err(|e: ParseIntError| e.to_string())?,
            geode_robot_obsidian.parse().map_err(|e: ParseIntError| e.to_string())?,
        );

        Ok(Self {
            id,
            ore_robot_ore,
            clay_robot_ore,
            obsidian_robot_ore,
            obsidian_robot_clay,
            geode_robot_ore,
            geode_robot_obsidian,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    minutes_left: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
}
impl State {
    pub fn part1() -> Self {
        Self {
            minutes_left: 24,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }
    pub fn advance(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
        self.minutes_left -= 1;
    }
}


fn parse_input(input: &'static str) -> Vec<Blueprint> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}
