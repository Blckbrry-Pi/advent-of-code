use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Blueprint {
    pub id: u8,
    pub ore_robot_ore: u8,
    pub clay_robot_ore: u8,
    pub obsidian_robot_ore: u8,
    pub obsidian_robot_clay: u8,
    pub geode_robot_ore: u8,
    pub geode_robot_obsidian: u8,
}
impl Blueprint {
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
