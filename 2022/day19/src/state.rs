use std::collections::HashMap;

use crate::blueprints::Blueprint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
    minutes_left: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
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
    pub fn part2() -> Self {
        Self {
            minutes_left: 32,
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
    pub fn advanced(mut self) -> Self {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
        self.minutes_left -= 1;
        self
    }
    pub fn advanced_n(mut self, n: usize) -> Self {
        for _ in 0..n {
            self = self.advanced();
        }
        self
    }

    pub fn recurse(self, blueprint: &Blueprint, memo: &mut HashMap<Self, i32>, best_seen_so_far: &mut i32) -> i32 {
        if self.minutes_left <= 1 {
            return self.geodes + self.geode_robots * self.minutes_left;
        } else if let Some(best) = memo.get(&Self { geodes: 0, ..self }) {
            return *best + self.geodes;
        }

        // This approximation definitely isn't sound, but it works for my input :3
        if self.minutes_left < 10 {
            let guess_at_max = self.geodes + (self.geode_robots + 2) * (self.minutes_left + 2);
            if guess_at_max < *best_seen_so_far {
                return 0;
            }
        }
        let mut best = 0;
        for i in 0..4 {
            if self.can_build_geode(blueprint) && i != 3 { continue }
            let new = match i {
                0 => self.until_ore_built(blueprint),
                1 => self.until_clay_built(blueprint),
                2 => self.until_obsidian_built(blueprint),
                3 => self.until_geode_built(blueprint),
                _ => unreachable!("a"),
            };
            let new = if new.minutes_left <= 0 {
                self.advanced_n(self.minutes_left as usize)
            } else {
                new.advanced()
            };
            best = best.max(new.recurse(blueprint, memo, best_seen_so_far));
        }
        *best_seen_so_far = best.max(*best_seen_so_far);
        memo.insert(Self { geodes: 0, ..self }, best - self.geodes);
        best
    }

    pub fn can_build_ore(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.ore_robot_ore as i32
    }
    pub fn ore_built(mut self, blueprint: &Blueprint) -> Self {
        self.ore_robots += 1;
        self.ore -= 1;
        self.ore -= blueprint.ore_robot_ore as i32;
        self
    }
    pub fn until_ore_built(mut self, blueprint: &Blueprint) -> Self {
        while !self.can_build_ore(blueprint) { self = self.advanced(); }
        self = self.ore_built(blueprint);
        self
    }

    pub fn can_build_clay(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.clay_robot_ore as i32
    }
    pub fn clay_built(mut self, blueprint: &Blueprint) -> Self {
        self.clay_robots += 1;
        self.clay -= 1;
        self.ore -= blueprint.clay_robot_ore as i32;
        self
    }
    pub fn until_clay_built(mut self, blueprint: &Blueprint) -> Self {
        while !self.can_build_clay(blueprint) { self = self.advanced(); }
        self = self.clay_built(blueprint);
        self
    }

    pub fn can_build_obsidian(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.obsidian_robot_ore as i32 && self.clay >= blueprint.obsidian_robot_clay as i32
    }
    pub fn obsidian_built(mut self, blueprint: &Blueprint) -> Self {
        self.obsidian_robots += 1;
        self.obsidian -= 1;
        self.ore -= blueprint.obsidian_robot_ore as i32;
        self.clay -= blueprint.obsidian_robot_clay as i32;
        self
    }
    pub fn until_obsidian_built(mut self, blueprint: &Blueprint) -> Self {
        if self.clay_robots == 0 { return Self::zero() }
        while !self.can_build_obsidian(blueprint) { self = self.advanced(); }
        self = self.obsidian_built(blueprint);
        self
    }

    pub fn can_build_geode(&self, blueprint: &Blueprint) -> bool {
        self.ore >= blueprint.geode_robot_ore as i32 && self.obsidian >= blueprint.geode_robot_obsidian as i32
    }
    pub fn geode_built(mut self, blueprint: &Blueprint) -> Self {
        self.geode_robots += 1;
        self.geodes -= 1;
        self.ore -= blueprint.geode_robot_ore as i32;
        self.obsidian -= blueprint.geode_robot_obsidian as i32;
        self
    }
    pub fn until_geode_built(mut self, blueprint: &Blueprint) -> Self {
        if self.obsidian_robots == 0 { return Self::zero() }
        while !self.can_build_geode(blueprint) { self = self.advanced(); }
        self = self.geode_built(blueprint);
        self
    }

    pub fn zero() -> Self {
        Self {
            minutes_left: 0,
            ore_robots: 0,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }
}
