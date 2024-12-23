use std::rc::Rc;

aoc_tools::aoc_sol!(day12: part1, part2);
aoc_tools::fast_hash!();

pub fn part1(input: &str) -> u64 {
    let mut garden = parse_input(input);
    garden.calc_area_perimeter();
    garden.calc_total_p1()
}

pub fn part2(input: &str) -> u64 {
    let mut garden = parse_input(input);
    garden.calc_area_perimeter();
    garden.calc_sides();
    garden.calc_total_p2()
}

fn parse_input(input: &str) -> Garden {
    let plots: Vec<Vec<_>> = input.lines()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.chars().collect())
        .collect();

    Garden::from_plots(plots)
}

#[derive(Debug, Clone)]
struct Garden {
    plots: Vec<Vec<char>>,
    groupings: Vec<Vec<Option<usize>>>,
    perimeters: HashMap<usize, u64>,
    areas: HashMap<usize, u64>,
    sides: HashMap<usize, FastSet<Side>>,
}

impl Garden {
    fn from_plots(plots: Vec<Vec<char>>) -> Self {
        let groupings = vec![vec![None; plots[0].len()]; plots.len()];
        let mut output = Self {
            plots,
            groupings,
            perimeters: HashMap::new(),
            areas: HashMap::new(),
            sides: HashMap::new(),
        };
        output.group();
        output
    }
    fn width(&self) -> isize { self.plots[0].len() as isize }
    fn height(&self) -> isize { self.plots.len() as isize }
    fn get(&self, x: isize, y: isize) -> Option<char> {
        if y < 0 || y >= self.height() { return None; }
        if x < 0 || x >= self.width() { return None; }
        Some(self.plots[y as usize][x as usize])
    }
    fn get_group(&self, x: isize, y: isize) -> Option<usize> {
        if y < 0 || y >= self.height() { return None; }
        if x < 0 || x >= self.width() { return None; }
        self.groupings[y as usize][x as usize]
    }

    /// ## Does 2 passes:
    /// 
    /// ### Pass 1:
    /// Iterates through each cell, assigning it a group number based on the
    /// upper and right cells, also keeping track of what groups are connected
    /// to it
    /// 
    /// ### Pass 2:
    /// Combine connected groups with the same plot symbol into larger
    /// monolithic groups
    fn group(&mut self) {
        let mut next_group = 0;

        // A HashMap from Group -> All Connected Groups (including itself)
        let mut groupings: HashMap<usize, Rc<HashSet<usize>>> = HashMap::new();

        // Iterate through all cells
        for y in 0..self.height() {
            for x in 0..self.width() {
                let plot_at = self.plots[y as usize][x as usize];
                let bordering = [
                    (
                        self.get(x, y - 1),
                        self.get_group(x, y - 1),
                    ),
                    (
                        self.get(x - 1, y),
                        self.get_group(x - 1, y),
                    ),
                ];

                // Bridge matching adjacent plot groups up and right 
                let mut curr_group: Option<usize> = None;
                for (test_plot, group) in bordering {
                    let Some(test_plot) = test_plot else { continue; };
                    let Some(new_group) = group else { continue; };

                    if test_plot == plot_at {
                        if let Some(curr) = curr_group {
                            // If the newly detected connected plot isn't a
                            // member of the same group, connect all groups the
                            // new plot is connected to with all the groups the
                            // old plot is connected to.
                            if new_group != curr {
                                let curr_connected = &groupings[&curr];
                                let new_connected = &groupings[&new_group];

                                let connected: HashSet<usize> = curr_connected.union(&new_connected).copied().collect();
                                let connected = Rc::new(connected);

                                for group in connected.iter().copied() {
                                    groupings.insert(group, connected.clone());
                                }
                            }
                        } else {
                            // Assign the current group to be whatever the
                            // matching adjacent plot's group is
                            curr_group = Some(new_group);
                        }
                    }
                }

                // If a matching adjacent group was found, that is this plot's
                // group
                // Otherwise, create a new group with just this plot
                let group = if let Some(group) = curr_group {
                    group
                } else {
                    let group = next_group;
                    groupings.insert(group, Rc::new([group].into_iter().collect()));
                    next_group += 1;
                    group
                };
                self.groupings[y as usize][x as usize] = Some(group);
            }
        }

        // Combine connected groups
        let mut seen = HashMap::new();
        let mut next = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let group = self.groupings[y as usize][x as usize].unwrap();
                if let Some(&replacement) = seen.get(&group) {
                    // If this connected group set has already been found, just
                    // use the replacement group index assigned to it
                    self.groupings[y as usize][x as usize] = Some(replacement);
                } else {
                    // Otherwise, create a new replacement group index and
                    // assign it to every connected group
                    let replacement = next;
                    next += 1;
                    for seen_grouping in groupings.get(&group).unwrap().iter().copied() {
                        seen.insert(seen_grouping, replacement);
                    }

                    self.groupings[y as usize][x as usize] = Some(replacement);
                }
            }
        }
    }

    /// ```text
    /// for each cell C:
    ///     C.Group.Area += 1
    ///     for each neighboring cell NC:
    ///         If C.Group != NC.Group:
    ///             C.Group.Perimeter += 1
    /// ``````
    pub fn calc_area_perimeter(&mut self) {
        self.perimeters.clear();
        self.areas.clear();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let group = self.groupings[y as usize][x as usize].unwrap();
                let bordering = [
                    self.get_group(x, y - 1),
                    self.get_group(x + 1, y),
                    self.get_group(x, y + 1),
                    self.get_group(x - 1, y),
                ];
                for test_group in bordering {
                    if Some(group) != test_group {
                        *self.perimeters.entry(group).or_default() += 1;
                    }
                }
                *self.areas.entry(group).or_default() += 1;
            }
        }
    }

    /// ## 3 Passes are done:
    /// 
    /// ### Pass 1:
    /// ```text
    /// for each cell C:
    ///     for each neighboring cell NC:
    ///         If C.Group != NC.Group:
    ///             C.Group.SideSegments += Side between C and NC
    /// ```
    /// 
    /// ### Pass 2:
    /// ```text
    /// for each group G:
    ///     for each side piece SP:
    ///         Add a side to G.Sides that is SP combined
    ///             with as many other side pieces as possible.
    /// ```
    /// 
    /// ### Pass 3:
    /// ```text
    /// for each group G:
    ///     for each side S:
    ///         Add all sides to G.SplitSides that are
    ///             created from S split by every splitting
    ///             side in G
    /// ```
    pub fn calc_sides(&mut self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let group = self.groupings[y as usize][x as usize].unwrap();
                let bordering_groups = [
                    self.get_group(x, y - 1),
                    self.get_group(x, y + 1),
                    self.get_group(x - 1, y),
                    self.get_group(x + 1, y),
                ];
                let bordering_sides = [
                    Side::Hori { y, x_start: x, x_end: x },
                    Side::Hori { y: y + 1, x_start: x, x_end: x },
                    Side::Vert { x, y_start: y, y_end: y },
                    Side::Vert { x: x + 1, y_start: y, y_end: y },
                ];
                for (test_group, new_side) in bordering_groups.into_iter().zip(bordering_sides) {
                    if Some(group) != test_group {
                        self.sides.entry(group)
                            .or_insert_with(|| new_fastset_with_capacity(32))
                            .insert(new_side);
                    }
                }
            }
        }

        // Join adjacent sides together
        for sides in self.sides.values_mut() {
            let mut joinable_queue: FastSet<_> = std::mem::replace(sides, new_fastset_with_capacity(sides.len() / 2));
            while let Some(&side) = joinable_queue.iter().next() {
                // For each side, if it can be joined, join it and put the
                // joined parts back onto the joinable queue
                // Otherwise, it's good to go, so put it into the finished sides
                //
                // Notably, this will create duplicates, but this is alleviated
                // by the fact that all maximally joined sides that would
                // overlap will be the same, which the HashSet will deal with.

                joinable_queue.remove(&side);

                'outer: {
                    for joiner in joinable_queue.iter().copied() {
                        if let Some(joined) = side.join(joiner) {
                            joinable_queue.insert(joined);
                            joinable_queue.remove(&joiner);
                            break 'outer;
                        }
                    }
                    sides.insert(side);
                }
            }
            // *sides = finished_sides;
        }

        // Split intersecting sides apart
        // Deals with situations like 
        // AAAA
        // A.AA
        // AA.A
        // AAAA
        // where the joined sides will intersect
        for sides in self.sides.values_mut() {
            let mut splittable_queue: Vec<_> = sides.iter().copied().collect();
            let mut finished_sides = new_fastset_with_capacity(splittable_queue.len());
            let mut new_splittable_queue = Vec::with_capacity(splittable_queue.len());
            for splitter in sides.iter().copied() {
                while let Some(side) = splittable_queue.pop() {
                    if side.len() <= 1 {
                        finished_sides.insert(side);
                        continue;
                    }
                    // For each side, if it can be split, split it and put the split
                    // parts back onto the splittable queue
                    // Otherwise, it's good to go, so put it into finished sides
                    if let Some((a, b)) = side.split(splitter) {
                        splittable_queue.push(a);
                        splittable_queue.push(b);
                        // break 'outer;
                    } else {
                        new_splittable_queue.push(side);
                    }
                }
                std::mem::swap(&mut splittable_queue, &mut new_splittable_queue);
            }
            finished_sides.extend(splittable_queue.drain(..));
            *sides = finished_sides;
        }
    }

    /// Gets the sum of each groups area * perimeter
    fn calc_total_p1(&self) -> u64 {
        self.areas.iter()
        .map(|(plot, &area)| {
            let peri = *self.perimeters.get(plot).unwrap();
            area * peri
        })
        .sum()
    }

    /// Gets the sum of each groups area * side count
    fn calc_total_p2(&self) -> u64 {
        self.areas.iter()
            .map(|(plot, &area)| {
                let sides = self.sides.get(plot).unwrap().len();
                area * sides as u64
            })
            .sum()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Side {
    Vert { x: isize, y_start: isize, y_end: isize },
    Hori { y: isize, x_start: isize, x_end: isize },
}

impl Side {
    pub fn join(&self, other: Side) -> Option<Self> {
        match (*self, other) {
            (
                Self::Vert { x, y_start, y_end },
                Self::Vert { x: other_x, y_start: other_y_start, y_end: other_y_end },
            ) => {
                if x != other_x { return None; }

                let other_start_in_range = (y_start-1..=y_end+1).contains(&other_y_start);
                let other_end_in_range = (y_start-1..=y_end+1).contains(&other_y_end);
                if other_start_in_range || other_end_in_range {
                    Some(Self::Vert {
                        x,
                        y_start: y_start.min(other_y_start),
                        y_end: y_end.max(other_y_end),
                    })
                } else {
                    None
                }
            },
            (Self::Hori { .. }, Self::Hori { .. }) => Some(self.invert().join(other.invert())?.invert()),
            _ => None,
        }
    }
    fn invert(&self) -> Self {
        match *self {
            Self::Vert { x, y_start, y_end } => Self::Hori { y: x, x_start: y_start, x_end: y_end },
            Self::Hori { y, x_start, x_end } => Self::Vert { x: y, y_start: x_start, y_end: x_end },
        }
    }
    fn split(&self, splitter: Side) -> Option<(Self, Self)> {
        match (*self, splitter) {
            (
                Self::Vert { x, y_start, y_end },
                Self::Hori { y, x_start, x_end },
            ) => {
                let x_contained = (x_start..=x_end+1).contains(&x);
                let y_crosses = y_start < y && y_end >= y;

                if x_contained && y_crosses {
                    let new_y_end = y_end.min(y - 1);
                    let new_y_start = y_start.max(y);

                    Some((
                        Self::Vert { x, y_start, y_end: new_y_end },
                        Self::Vert { x, y_start: new_y_start, y_end },
                    ))
                } else {
                    None
                }
            },
            (Self::Hori { .. }, Self::Vert { .. }) => {
                let (a, b) = self.invert().split(splitter.invert())?;
                Some((a.invert(), b.invert()))
            },
            _ => None,
        }
    }
    fn len(&self) -> isize {
        match *self {
            Self::Hori { x_start, x_end, .. } => (x_end - x_start).abs() + 1,
            Self::Vert { y_start, y_end, .. } => (y_end - y_start).abs() + 1,
        }
    }
}

impl Debug for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vert { x, y_start, y_end } => write!(f, "({x}, {y_start}..{y_end})"),
            Self::Hori { y, x_start, x_end } => write!(f, "({x_start}..{x_end}, {y})"),
        }
    }
}
