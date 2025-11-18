use std::{rc::Rc, str::FromStr};
aoc_tools::aoc_sol!(day24 2018: part1, part2);

type Scalar = u64;

#[derive(Clone, PartialEq, Eq)]
struct Group {
    id: u8,
    count: Scalar,
    hp: Scalar,
    weaknesses: Rc<HashSet<String>>,
    immunities: Rc<HashSet<String>>,
    damage: Scalar,
    attack_type: Rc<str>,
    initiative: u32,
}
impl Group {
    pub fn effective_power(&self) -> Scalar {
        self.count * self.damage
    }
    pub fn damage_amount(&self, o: &Self) -> Scalar {
        if o.immunities.contains(self.attack_type.as_ref()) {
            0
        } else if o.weaknesses.contains(self.attack_type.as_ref()) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }
    pub fn receive_damage(&mut self, damage: Scalar) -> Scalar {
        let troops_lost = damage / self.hp;
        let new_count = self.count.saturating_sub(troops_lost);
        let actual_lost = self.count - new_count;
        self.count = new_count;
        actual_lost
    }
    pub fn order_for_selection(groups: &mut [Self]) {
        groups.sort_by(|a, b| {
            let effective_power_cmp = a.effective_power().cmp(&b.effective_power());
            let initiative_cmp = a.initiative.cmp(&b.initiative);
            effective_power_cmp.reverse().then(initiative_cmp.reverse())
        });
    }
    pub fn choose_target(&self, other: &[Group], taken: &HashSet<u8>) -> Option<u8> {
        let mut best_seen = None;
        let mut best_damage = 0;
        let mut best_effective_power = 0;
        let mut best_initiative = 0;
        for target in other {
            if taken.contains(&target.id) { continue }
            let damage_amount = self.damage_amount(target);
            if damage_amount == 0 { continue }
            if damage_amount < best_damage {
                continue
            } else if damage_amount == best_damage {
                if target.effective_power() < best_effective_power {
                    continue
                } else if target.effective_power() == best_effective_power {
                    if target.initiative <= best_initiative {
                        continue
                    }
                }
            }
            best_seen = Some(target.id);
            best_damage = damage_amount;
            best_effective_power = target.effective_power();
            best_initiative = target.initiative;
        }
        best_seen
    }
    pub fn choose_targets(attackers: &mut [Group], defenders: &[Group]) -> HashMap<u8, Option<u8>> {
        Self::order_for_selection(attackers);
        let mut taken = HashSet::with_capacity(attackers.len().min(defenders.len()));
        let mut targets = HashMap::with_capacity(attackers.len());
        for attacker in &*attackers {
            let target = attacker.choose_target(defenders, &taken);
            taken.extend(target);
            targets.insert(attacker.id, target);
        }
        targets
    }
}
impl FromStr for Group {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((count, s)) = s.split_once(" units each with ") else { return Err("missing unit count".to_string()); };
        let Some((hp, s)) = s.split_once(" hit points") else { return Err("missing unit hp".to_string()); };
        let Some((weak_immune, s)) = s.split_once("with an attack that does ") else { return Err("missing weaknesses and immunities".to_string()); };
        let Some((damage, s)) = s.split_once(' ') else { return Err("missing attack damage".to_string()); };
        let Some((attack_type, initiative)) = s.split_once(" damage at initiative ") else { return Err("missing attack type".to_string()); };
        let count      = count     .parse::<Scalar>().map_err(|e| format!("Invalid unit count {count:?}: {e:?}"))?;
        let hp         = hp        .parse::<Scalar>().map_err(|e| format!("Invalid unit hp {hp:?}: {e:?}"))?;
        let damage     = damage    .parse::<Scalar>().map_err(|e| format!("Invalid attack damage {damage:?}: {e:?}"))?;
        let initiative = initiative.parse::<u32>().map_err(|e| format!("Invalid initiative {initiative:?}: {e:?}"))?;


        let weak_immune = weak_immune.trim_start_matches(" (").trim_end_matches(") ");
        let (a, b) = weak_immune.split_once(';').unwrap_or((weak_immune, ""));
        let (weaknesses, immunities) = if a.starts_with('w') {
            (a.trim(), b.trim())
        } else {
            (b.trim(), a.trim())
        };
        let weaknesses = weaknesses.strip_prefix("weak to ").unwrap_or(weaknesses);
        let immunities = immunities.strip_prefix("immune to ").unwrap_or(immunities);
        let weaknesses = weaknesses.split(", ").filter(|w| !w.is_empty()).map(|w| w.to_string()).collect();
        let immunities = immunities.split(", ").filter(|w| !w.is_empty()).map(|i| i.to_string()).collect();

        Ok(Group {
            id: 0,
            count,
            hp,
            weaknesses: Rc::new(weaknesses),
            immunities: Rc::new(immunities),
            damage,
            attack_type: Rc::from(attack_type),
            initiative,
        })
    }
}

impl Debug for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "G{:02}: {:5}♡ x {:4}👤 ({:>13} {:4}⚔, {:4}⚡) ", self.id, self.hp, self.count, self.attack_type, self.damage, self.initiative)?;

        write!(f, "[weak to ")?;
        for (i, weakness) in self.weaknesses.iter().enumerate() {
            if i != 0 { write!(f, ", ")?; }
            write!(f, "{weakness}")?;
        }
        if self.weaknesses.is_empty() { write!(f, "nothing")?; }

        write!(f, "; immune to ")?;
        for (i, immunity) in self.immunities.iter().enumerate() {
            if i != 0 { write!(f, ", ")?; }
            write!(f, "{immunity}")?;
        }
        if self.immunities.is_empty() { write!(f, "nothing")?; }
        write!(f, "]")
    }
}


fn do_round(immune: &mut Vec<Group>, infection: &mut Vec<Group>) -> bool {
    let immune_targets = Group::choose_targets(immune, &infection);
    let infection_targets = Group::choose_targets(infection, &immune);

    let mut total_troops_killed = 0;
    let highest_initiative = immune.iter()
        .chain(infection.iter())
        .map(|g| g.initiative)
        .max()
        .unwrap();
    for initiative in (1..=highest_initiative).rev() {
        let Some((is_immune, attacker)) = immune.iter()
            .map(|g| (true, g))
            .chain(infection.iter().map(|g| (false, g)))
            .find(|(_, g)| g.initiative == initiative)
        else { continue; };
        
        if attacker.count == 0 { continue }
        let attacker_id = attacker.id;

        let Some(&Some(target_id)) = (if is_immune {
            immune_targets.get(&attacker_id)
        } else {
            infection_targets.get(&attacker_id)
        }) else { continue; };
        let target = if is_immune { &infection } else { &immune };
        let target = target.iter().find(|g| g.id == target_id).unwrap();
        let damage_dealt = attacker.damage_amount(target);

        let target = if is_immune { &mut *infection } else { &mut *immune };
        let target = target.iter_mut().find(|g| g.id == target_id).unwrap();

        total_troops_killed += target.receive_damage(damage_dealt);
    }

    *immune = immune.drain(..).filter(|g| g.count != 0).collect();
    *infection = infection.drain(..).filter(|g| g.count != 0).collect();

    immune.len() == 0 || infection.len() == 0 || total_troops_killed == 0
}

fn immune_system_wins_by(immune: &[Group], infection: &[Group], boost: Scalar) -> Option<Scalar> {
    let mut immune   : Vec<_> = immune   .iter().map(Clone::clone).map(|mut g| { g.damage += boost; g }).collect();
    let mut infection: Vec<_> = infection.iter().map(Clone::clone).collect();
    while !do_round(&mut immune, &mut infection) {}
    if infection.len() == 0 {
        Some(immune.iter().map(|g| g.count).sum())
    } else {
        None
    }
}

pub fn part1(input: &str) -> Scalar {
    let (mut immune, mut infection) = parse_input(input);
    while !do_round(&mut immune, &mut infection) {}
    immune.into_iter()
        .chain(infection)
        .map(|g| g.count)
        .sum()
}

pub fn part2(input: &str) -> Scalar {
    let (immune, infection) = parse_input(input);

    let mut min = 1;
    let mut max = 2;
    while immune_system_wins_by(&immune, &infection, max).is_none() {
        min = max;
        max *= 2;
    }

    while min != max {
        let boost = (min + max) / 2;
        if immune_system_wins_by(&immune, &infection, boost).is_some() {
            max = boost;
        } else {
            min = boost + 1;
        }
    }
    immune_system_wins_by(&immune, &infection, min).unwrap()
}

fn parse_input(input: &str) -> (Vec<Group>, Vec<Group>) {
    let (immune, infection) = input.split_once("\n\nInfection:\n").unwrap();
    let immune = immune.strip_prefix("Immune System:\n").unwrap();
    (
        immune.trim()
            .lines()
            .map(|g| g.parse::<Group>().unwrap())
            .enumerate()
            .map(|(i, mut group)| { group.id = i as u8 + 1; group })
            .collect(),
        infection.trim()
            .lines()
            .map(|g| g.parse::<Group>().unwrap())
            .enumerate()
            .map(|(i, mut group)| { group.id = i as u8 + 1; group })
            .collect(),
    )
}
