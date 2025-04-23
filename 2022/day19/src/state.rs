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

    fn guess_at_max_geodes(&self, blueprint: &Blueprint) -> i32 {
        if self.minutes_left <= 1 { return self.geodes + self.geode_robots * self.minutes_left; }

        // We can only get as many new (relevant) robots as there are minutes
        // left (minus one)
        let max_new_robots = self.minutes_left - 1;

        // Adjust this based on whether or not the obsidian robots can keep up
        // with the rate of production, only if the time left is low
        // This is where the approximation becomes unsound, but it works for
        // most inputs (including mine)
        let max_obsidian_production = self.obsidian_robots + 1;
        let adj_max_obsidian_production = max_obsidian_production + self.obsidian / self.minutes_left;
        let obsidian_rate = blueprint.geode_robot_obsidian as i32 / adj_max_obsidian_production;
        let max_new_robots = if self.minutes_left < 10 {
            max_new_robots / obsidian_rate.max(1)
        } else {
            max_new_robots
        };
        // Something something triangular numbers-ish because of when the robots
        // would come online
        let max_new_output = (self.minutes_left - 1) * max_new_robots / 2;
        // The theoretical maximum with the current & new geode robots
        self.geodes + self.geode_robots * self.minutes_left + max_new_output
    }

    pub fn recurse(self, blueprint: &Blueprint, memo: &mut HashMap<Self, i32>, best_seen_so_far: &mut i32) -> i32 {
        if self.minutes_left <= 1 {
            return self.geodes + self.geode_robots * self.minutes_left;
        } else if let Some(best_number_of_new) = memo.get(&Self { geodes: 0, ..self }) {
            return *best_number_of_new + self.geodes;
        }

        let mut best = 0;
        for i in 0..4 {
            let new = match i {
                0 => self.until_geode_built(blueprint),
                1 => self.until_obsidian_built(blueprint),
                2 => self.until_clay_built(blueprint),
                3 => self.until_ore_built(blueprint),
                _ => unreachable!("a"),
            };
            let new = if new.minutes_left <= 0 {
                self.advanced_n(self.minutes_left as usize)
            } else {
                new.advanced()
            };
            if new.guess_at_max_geodes(blueprint) < *best_seen_so_far { continue }

            best = best.max(new.recurse(blueprint, memo, best_seen_so_far));
        }
        *best_seen_so_far = best.max(*best_seen_so_far);

        let best_number_of_new_geodes = best - self.geodes;
        memo.insert(Self { geodes: 0, ..self }, best_number_of_new_geodes);
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
