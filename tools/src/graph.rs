use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::Hash;
use std::ops::Add;

#[derive(Debug)]
pub struct Graph<Id: Hash + Eq + Copy, N, E> {
    nodes: HashMap<Id, (N, HashMap<Id, usize>)>,
    edge_data: HashMap<usize, E>,
    next_edge_data_id: usize,
}

impl<Id: Hash + Eq + Copy, N, E> Graph<Id, N, E> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edge_data: HashMap::new(),
            next_edge_data_id: 0,
        }
    }
    pub fn insert_or_update_node(&mut self, id: Id, data: N) {
        match self.nodes.get_mut(&id) {
            Some((data_ref, _)) => *data_ref = data,
            None => { self.nodes.insert(id, (data, HashMap::new())); }
        }
    }
    pub fn insert_node(&mut self, id: Id, data: N) -> Option<(N, Vec<((Id, Id), E)>)> {
        let (old_data, old_edges) = self.nodes.insert(id, (data, HashMap::new()))?;
        let removed_edges = self.remove_node_bits(id, old_edges);
        Some((old_data, removed_edges))
    }
    pub fn update_node_data(&mut self, id: &Id, data: N) -> N {
        let Some((data_ptr, _)) = self.nodes.get_mut(id) else {
            panic!("Can't update the data of a nonexistent node");
        };
        std::mem::replace(data_ptr, data)
    }
    pub fn remove_node(&mut self, id: &Id) -> Option<(N, Vec<((Id, Id), E)>)> {
        let (data, edges) = self.nodes.remove(id)?;
        let removed_edges = self.remove_node_bits(*id, edges);
        Some((data, removed_edges))
    }
    fn remove_node_bits(&mut self, id: Id, mut old_edges: HashMap<Id, usize>) -> Vec<((Id, Id), E)> {
        let extra_removed_edge = if let Some(edge_data_id) = old_edges.remove(&id) {
            let data = self.edge_data
                .remove(&edge_data_id)
                .expect("Invariant violated: edge data ID missing associated data");
            Some(((id, id), data))
        } else { None };
        let mut removed_edges = self.remove_edges(old_edges.into_keys().map(|other_id| (id, other_id)));
        removed_edges.extend(extra_removed_edge);
        removed_edges
    }
    /// Removes the edge from the list on the second node, and removes the edge data
    /// 
    /// Cannot remove self-edges, will cause function to panic
    fn remove_edges(&mut self, edges: impl IntoIterator<Item = (Id, Id)>) -> Vec<((Id, Id), E)> {
        let edges = edges.into_iter();
        let mut output = Vec::with_capacity(edges.size_hint().0);
        for (node_a, node_b) in edges {
            if node_a == node_b { panic!("Precondition failed: Cannot remove self-edges") }
            let edge_data_id = self.nodes
                .get_mut(&node_b)
                .expect("Prevondition violated: cannot remove edge containing a node that does not exist")
                .1
                .remove(&node_a)
                .expect("Precondition violated: cannot remove edge between nodes that have no record of that edge");
            let edge_data = self.edge_data
                .remove(&edge_data_id)
                .expect("Invariant violated: edge data ID missing associated data");
            output.push(((node_a, node_b), edge_data));
        }
        output
    }
    pub fn nodes(&self) -> impl Iterator<Item = (&Id, &N)> + '_ {
        self.nodes.iter().map(|(id, (node_data, _))| (id, node_data))
    }
    pub fn get_node(&self, id: &Id) -> Option<&N> {
        self.nodes
            .get(&id)
            .map(|(data, _)| data)
    }
    pub fn get_node_mut(&mut self, id: &Id) -> Option<&mut N> {
        self.nodes
            .get_mut(&id)
            .map(|(data, _)| data)
    }
    pub fn edges_for<'a>(&'a self, id: &Id) -> impl Iterator<Item = (Id, &'a E)> + 'a {
        self.nodes
            .get(id)
            .expect("Cannot get the edges for a nonexistent node")
            .1
            .iter()
            .map(|(&other_id, edge_data_id)| {
                let edge_data = self.edge_data
                    .get(edge_data_id)
                    .expect("Invariant violated: edge data ID missing associated data");
                (other_id, edge_data)
            })
    }
    pub fn insert_edge(&mut self, a: &Id, b: &Id, edge_data: E) -> Option<E> {
        if a == b {
            let edge_data_id = *self.nodes
                .get_mut(a)
                .expect("Precondition violated: cannot add self-edge on node that does not exist")
                .1
                .entry(*b)
                .or_insert_with(|| {
                    let out = self.next_edge_data_id;
                    self.next_edge_data_id += 1;
                    out
                });
            self.edge_data.insert(edge_data_id, edge_data)
        } else {
            let [Some((_, edge_map_a)), Some((_, edge_map_b))] = self.nodes.get_disjoint_mut([a, b]) else {
                panic!("Precondition violated: cannot add an edge between 2 nodes where at least 1 is nonexistent")
            };
            match (edge_map_a.get_mut(b), edge_map_b.get_mut(a)) {
                (Some(edge_data_id_a), Some(edge_data_id_b)) => {
                    if edge_data_id_a != edge_data_id_b {
                        panic!("Invariant violated: nodes disagree on the edge data ID of the edge between them")
                    }
                    let prev_data = self.edge_data
                        .insert(*edge_data_id_a, edge_data)
                        .expect("Invariant violated: edge data ID missing associated data");
                    Some(prev_data)
                },
                (None, None) => {
                    let new_edge_data_id = self.next_edge_data_id;
                    self.next_edge_data_id += 1;
                    if self.edge_data.insert(new_edge_data_id, edge_data).is_some() {
                        panic!("Invariant violated: there was an edge entry with the value of self.next_edge_data_id");
                    }
                    edge_map_a.insert(*b, new_edge_data_id);
                    edge_map_b.insert(*a, new_edge_data_id);
                    None
                },
                (_, _) => panic!("Invariant violated: nodes disagreed on the existence of an edge between them"),
            }
        }
    }
    pub fn remove_edge(&mut self, a: &Id, b: &Id) -> Option<E> {
        let edge_data_id = self.nodes
            .get_mut(a)
            .expect("Precondition violated: cannot remove edge on a node that does not exist")
            .1
            .remove(b)?;
        if a != b {
            let other_edge_data_id = self.nodes
                .get_mut(b)
                .expect("Precondition violated: cannot remove edge on a node that does not exist")
                .1
                .remove(a)
                .expect("Invariant violated: nodes disagreed on the existence of an edge between them");
            if edge_data_id != other_edge_data_id {
                panic!("Invariant violated: nodes disagree on the edge data ID of the edge between them")
            }
        }
        let edge_data = self.edge_data
            .remove(&edge_data_id)
            .expect("Invariant violated: edge data ID missing associated data");
        Some(edge_data)
    }
    pub fn get_edge(&mut self, a: &Id, b: &Id) -> Option<&E> {
        let edge_data_id = *self.nodes.get(a)?.1.get(b)?;
        let edge_data = self.edge_data
            .get(&edge_data_id)
            .expect("Invariant violated: edge data ID missing associated data");
        Some(edge_data)
    }
    pub fn get_edge_mut(&mut self, a: &Id, b: &Id) -> Option<&mut E> {
        let edge_data_id = *self.nodes.get(a)?.1.get(b)?;
        let edge_data = self.edge_data
            .get_mut(&edge_data_id)
            .expect("Invariant violated: edge data ID missing associated data");
        Some(edge_data)
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Distance<E> {
    Min,
    Val(E),
    Infinity,
}
impl<E: Clone> Distance<E> where for<'a> &'a E: Add<Output = E> {
    pub fn add(&self, e: &E) -> Self {
        match self {
            Self::Infinity => Self::Infinity,
            Self::Min => Self::Val(e.clone()),
            Self::Val(curr) => Self::Val(curr + e), 
        }
    }
}

impl<Id: Hash + Eq + Copy + std::fmt::Debug, N, E: Ord + Clone + std::fmt::Debug> Graph<Id, N, E>
where for<'a> &'a E: Add<Output = E> {
    pub fn reduce_chains(&mut self, mut keep: impl FnMut(Id) -> bool) -> Vec<N> {
        let mut to_remove = vec![];
        for (id, _) in self.nodes() {
            if keep(*id) { continue }
            let num_edges = self.edges_for(id).count();
            if num_edges == 2 { to_remove.push(*id); }
        }

        let mut removed = Vec::with_capacity(to_remove.len());
        for id in to_remove {
            let (data, edges) = self.remove_node(&id).expect("How???");
            removed.push(data);
            let a = edges[0].0.1;
            let b = edges[1].0.1;
            let new_edge_data = &edges[0].1 + &edges[1].1;
            self.insert_edge(&a, &b, new_edge_data);
        }

        removed
    }
    /// May be incorrect on negative edges
    pub fn dijkstra(&self, start: &Id, target: &Id) -> Option<(E, Vec<Id>)> {
        // Set up visited and unvisited sets/lists
        let mut visited = HashMap::<Id, Distance<E>>::new();

        let mut unvisited: HashMap<Id, Distance<E>> = self.nodes().map(|(id, _)| (
            *id,
            if id == start { Distance::Min } else { Distance::Infinity },
        )).collect();
        let mut unvisited_rev: BTreeMap<Distance<E>, HashSet<Id>> = BTreeMap::new();
        for (&id, distance) in unvisited.iter() {
            unvisited_rev
                .entry(distance.clone())
                .or_insert_with(|| HashSet::with_capacity(1))
                .insert(id);
        }

        'outer: while let Some((distance, hash)) = unvisited_rev.pop_first() {
            if distance == Distance::Infinity { return None; }
            for &id_to_process in &hash {
                visited.insert(id_to_process, distance.clone());
                unvisited.remove(&id_to_process);
            }
            for id_to_process in hash {
                if &id_to_process == target { break 'outer }

                for (neighbor_id, edge_data) in self.neighbors(&id_to_process) {
                    let Some(curr_neighbor_dist) = unvisited.get_mut(&neighbor_id) else { continue };
                    let new_dist = distance.add(edge_data);
                    if &new_dist < &*curr_neighbor_dist {
                        unvisited_rev
                            .get_mut(&curr_neighbor_dist)
                            .expect("Invariant violated: unvisited_rev map is out of sync with unvisited map")
                            .remove(&neighbor_id);
                        unvisited_rev
                            .entry(new_dist.clone())
                            .or_insert_with(|| HashSet::with_capacity(1))
                            .insert(neighbor_id);
                        *curr_neighbor_dist = new_dist;
                    }
                }
            }
        }
        let Some(output) = visited.remove(target) else { return None; };
        let mut path = vec![];
        let mut curr = *target;
        let mut curr_dist = output.clone();
        while &curr != start {
            path.push(curr);
            let (prev, prev_dist) = self.edges_for(&curr)
                .flat_map(|(id, _)| Some((id, visited.remove(&id)?)))
                .find(|(_, new_dist)| new_dist < &curr_dist)
                .expect("Something went wrong!");
            curr = prev;
            curr_dist = prev_dist;
        }
        path.push(*start);
        path.reverse();

        let Distance::Val(output) = output else { todo!() };
        Some((output, path))
    }
}

impl<Id: Hash + Eq + Copy, N, E: Ord> Graph<Id, N, E> {
    pub fn neighbors(&self, of_node: &Id) -> impl Iterator<Item = (Id, &E)> + '_ {
        self.edges_for(of_node)
    }
}

#[test]
fn test_dijkstra() {
    let mut graph = Graph::<u8, (), usize>::new();
    graph.insert_node(b'A', ());
    graph.insert_node(b'B', ());
    graph.insert_node(b'C', ());
    graph.insert_node(b'D', ());
    graph.insert_node(b'E', ());

    graph.insert_edge(&b'A', &b'B', 7);
    graph.insert_edge(&b'A', &b'E', 1);
    graph.insert_edge(&b'B', &b'C', 3);
    graph.insert_edge(&b'B', &b'E', 8);
    graph.insert_edge(&b'C', &b'D', 6);
    graph.insert_edge(&b'C', &b'E', 2);
    graph.insert_edge(&b'D', &b'E', 7);

    println!("{graph:?}");

    assert_eq!(graph.dijkstra(&b'A', &b'C'), Some((3, vec![b'A', b'E', b'C'])));
}
