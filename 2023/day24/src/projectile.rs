use std::{num::ParseIntError, ops::{Add, Div, Mul, Sub}, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3 { x: i128, y: i128, z: i128 }

impl Vec3 {
    pub fn new(x: i128, y: i128, z: i128) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> i128 { self.x }
    pub fn y(&self) -> i128 { self.y }
    pub fn z(&self) -> i128 { self.z }

    pub fn dot(&self, other: Vec3) -> i128 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Projectile {
    start: Vec3,
    velocity: Vec3,
}

impl Projectile {
    pub fn new(start: Vec3, velocity: Vec3) -> Self {
        Projectile { start, velocity }
    }

    pub fn pos(&self) -> Vec3 { self.start }
    pub fn vel(&self) -> Vec3 { self.velocity }

    pub fn at_time(&self, time: i128) -> Vec3 {
        Vec3::new(
            self.start.x() + self.velocity.x() * time,
            self.start.y() + self.velocity.y() * time,
            self.start.z() + self.velocity.z() * time,
        )
    }
}

impl FromStr for Projectile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s.split_once('@')
            .ok_or("Invalid format")
            .map_err(str::to_string)?;

        let pos = pos.split(',')
            .map(|p| p.trim().parse::<i128>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e: ParseIntError| e.to_string())?;

        let vel = vel.split(',')
            .map(|v| v.trim().parse::<i128>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e: ParseIntError| e.to_string())?;

        let pos = Vec3::new(pos[0], pos[1], pos[2]);
        let vel = Vec3::new(vel[0], vel[1], vel[2]);

        Ok(Self::new(pos, vel))
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

impl Mul<i128> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: i128) -> Vec3 {
        Vec3::new(
            self.x * other,
            self.y * other,
            self.z * other,
        )
    }
}

impl Div<i128> for Vec3 {
    type Output = Vec3;

    fn div(self, other: i128) -> Vec3 {
        Vec3::new(
            self.x / other,
            self.y / other,
            self.z / other,
        )
    }
}
