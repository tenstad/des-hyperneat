use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;

#[derive(Clone)]
pub struct Target<T, U> {
    pub node: T,
    pub edge: U,
}

impl<T, U> Target<T, U> {
    pub fn new(node: T, edge: U) -> Self {
        Self { node, edge }
    }
}

#[derive(Clone)]
pub struct Connection<T, U> {
    pub from: T,
    pub to: T,
    pub edge: U,
}

impl<T, U> Connection<T, U> {
    pub fn new(from: T, to: T, edge: U) -> Self {
        Self { from, to, edge }
    }
}

#[derive(PartialEq)]
pub enum OrderedAction<T, U> {
    Link(T, T, U),
    Activation(T),
}

impl<T: fmt::Display, U: fmt::Display> fmt::Display for OrderedAction<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderedAction::Link(from, to, edge) => write!(f, "{}-({})>{}", from, edge, to),
            OrderedAction::Activation(id) => write!(f, "{}", id),
        }
    }
}

/// Fast non-cyclic graph structure
#[derive(Clone)]
pub struct Connections<T: Hash, U> {
    connections: HashMap<T, Vec<(T, U)>>,
}

#[allow(dead_code)]
impl<T: Hash + Eq + Copy, U: Copy> Connections<T, U> {
    pub fn new() -> Self {
        Self {
            connections: HashMap::<T, Vec<(T, U)>>::new(),
        }
    }

    pub fn add(&mut self, from: T, to: T, edge: U) {
        assert!(
            !self.creates_cycle(from, to),
            "cannot add link that creates cycle"
        );
        assert!(!self.contains(&from, to), "cannot add existing connection");

        if let Some(vec) = self.connections.get_mut(&from) {
            vec.push((to, edge));
        } else {
            self.connections.insert(from, vec![(to, edge)]);
        }
    }

    pub fn extend(&mut self, other: &Self) {
        for source in other.get_sources() {
            for (target, edge) in other.get_edges(source) {
                self.add(*source, *target, *edge);
            }
        }
    }

    pub fn set_edge<'a>(&mut self, from: &'a T, to: T, edge: U) {
        let error = "cannot set non-existent edge";
        let edges = self.connections.get_mut(from).expect(error);
        let index = edges.iter().position(|(n, _)| *n == to).expect(error);
        edges[index].1 = edge;
    }

    pub fn get_edge<'a>(&'a self, from: &'a T, to: T) -> &'a U {
        let error = "cannot get non-existent edge";
        let edges = self.connections.get(from).expect(error);
        let index = edges.iter().position(|(n, _)| *n == to).expect(error);
        &edges[index].1
    }

    pub fn get_all_connections(&self) -> Vec<Connection<T, U>> {
        self.connections
            .iter()
            .flat_map(|(source, targets)| {
                targets
                    .iter()
                    .cloned()
                    .map(move |target| Connection::new(*source, target.0, target.1))
            })
            .collect()
    }

    pub fn get_all_nodes(&self) -> Vec<T> {
        self.get_all_connections()
            .iter()
            .flat_map(|connection| {
                std::iter::once(connection.from).chain(std::iter::once(connection.to))
            })
            .collect::<HashSet<T>>()
            .iter()
            .cloned()
            .collect()
    }

    pub fn get_edges<'a>(&'a self, from: &'a T) -> impl Iterator<Item = &'a (T, U)> {
        self.connections.get(from).into_iter().flatten()
    }

    pub fn get_targets<'a>(&'a self, from: &'a T) -> impl Iterator<Item = &'a T> {
        self.get_edges(from).map(|(n, _)| n)
    }

    pub fn get_sources<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.connections.keys()
    }

    pub fn contains(&self, from: &T, to: T) -> bool {
        !self.get_edges(from).position(|(x, _)| *x == to).is_none()
    }

    pub fn remove(&mut self, from: &T, to: T) -> U {
        let error = "cannot remove non-existent connection";
        let vec = self.connections.get_mut(from).expect(error);
        let index = vec.iter().position(|(x, _)| *x == to).expect(error);
        vec.swap_remove(index).1
    }

    /// DFS search to check for cycles.
    ///
    /// If 'from' is reachable from 'to', then addition will cause cycle
    pub fn creates_cycle(&self, from: T, to: T) -> bool {
        let mut visited: HashSet<T> = [to].iter().cloned().collect();
        let mut stack: Vec<T> = vec![to];

        while let Some(node) = stack.pop() {
            if node == from {
                return true; // Started at to and reached from, addition will cause cycle
            } else {
                // Add all connecting nodes to both stack and visited
                // Avoid extra storage and double filtering by adding to stack and copying from stack into visited
                let l = stack.len();
                stack.extend(self.get_targets(&node).filter(|n| !visited.contains(n)));
                visited.extend(stack.iter().skip(l));
            }
        }

        return false; // Unable to reach from when starting at to, addition will not cause cycle
    }

    /// Determine order of nodes and links to actiave in forward pass
    pub fn sort_topologically(&self) -> Vec<OrderedAction<T, U>> {
        // Store number of incoming connections for all nodes
        let mut backward_count: HashMap<T, u64> = HashMap::new();
        for source in self.get_sources() {
            for target in self.get_targets(source) {
                backward_count.insert(*target, *backward_count.get(target).unwrap_or(&0) + 1);
            }
        }

        // Start search from all nodes without incoming connections
        let mut stack: Vec<T> = self
            .get_sources()
            .filter(|node| *backward_count.get(node).unwrap_or(&0) == 0)
            .cloned()
            .collect();

        let mut actions = Vec::<OrderedAction<T, U>>::new();

        // Create topological order
        while let Some(node) = stack.pop() {
            actions.push(OrderedAction::Activation(node));

            // Process all outgoing connections from the current node
            for (to, edge) in self.get_edges(&node) {
                actions.push(OrderedAction::Link(node, *to, *edge));

                // Reduce backward count by 1
                backward_count.insert(*to, *backward_count.get(to).unwrap() - 1);

                // Add nodes with no incoming connections to the stack
                if *backward_count.get(to).unwrap() == 0 {
                    stack.push(*to);
                }
            }
        }

        actions
    }

    pub fn prune(&mut self, inputs: &Vec<T>, outputs: &Vec<T>) {
        self.prune_dangling_inputs(inputs);
        self.prune_dagnling_outputs(outputs);
    }

    pub fn prune_dangling_inputs(&mut self, inputs: &Vec<T>) {
        let mut backward_count: HashMap<T, u64> = HashMap::new();
        for source in self.get_sources() {
            for target in self.get_targets(source) {
                backward_count.insert(*target, *backward_count.get(target).unwrap_or(&0) + 1);
            }
        }

        loop {
            let dangling_inputs = self
                .get_all_nodes()
                .iter()
                .filter(|n| !inputs.contains(n) && *backward_count.get(n).unwrap_or(&0) == 0)
                .cloned()
                .collect::<Vec<T>>();
            for node in dangling_inputs.iter() {
                backward_count.remove(node);
                for target in self.get_targets(node) {
                    backward_count.insert(*target, *backward_count.get(target).unwrap() - 1);
                }
                self.connections.remove(node);
            }
            if dangling_inputs.len() == 0 {
                break;
            }
        }
    }

    pub fn prune_dagnling_outputs(&mut self, outputs: &Vec<T>) {
        loop {
            let mut deleted_node = false;
            for source in self.get_sources().cloned().collect::<Vec<T>>().iter() {
                let targets = self.connections.get(source).unwrap();
                let delete_indexes = (0..targets.len())
                    .rev()
                    .filter(|i| {
                        !outputs.contains(&targets[*i].0)
                            && !self.connections.contains_key(&targets[*i].0)
                    })
                    .collect::<Vec<usize>>();
                if delete_indexes.len() > 0 {
                    deleted_node = true;
                    let targets = self.connections.get_mut(source).unwrap();
                    for delete_index in delete_indexes {
                        targets.swap_remove(delete_index);
                    }
                }
            }
            if !deleted_node {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut connections = Connections::<u8, ()>::new();

        connections.add(0, 1, ());
        connections.add(0, 2, ());
        connections.add(1, 2, ());
        connections.add(1, 3, ());

        assert!(connections.contains(&0, 1));
        assert!(connections.contains(&1, 2));
        assert!(connections.contains(&0, 2));
        assert!(connections.contains(&1, 3));
    }

    #[test]
    fn test_get() {
        let mut connections = Connections::<u8, u8>::new();

        connections.add(0, 1, 5);
        connections.add(0, 2, 6);
        connections.add(0, 3, 7);

        let mut targets = connections
            .get_edges(&0)
            .cloned()
            .collect::<Vec<(u8, u8)>>();
        targets.sort();

        assert_eq!(targets, vec![(1, 5), (2, 6), (3, 7)]);
        assert_eq!(connections.get_edges(&5).next(), None);
    }

    #[test]
    fn test_set_edge() {
        let mut connections = Connections::<u8, u8>::new();

        connections.add(0, 1, 5);
        connections.add(0, 2, 6);
        connections.set_edge(&0, 1, 7);

        assert_eq!(*connections.get_edge(&0, 1), 7);
    }

    #[test]
    fn test_get_edge() {
        let mut connections = Connections::<u8, u8>::new();

        connections.add(0, 1, 5);
        connections.add(2, 3, 6);
        connections.add(1, 2, 7);

        assert_eq!(*connections.get_edge(&0, 1), 5);
        assert_eq!(*connections.get_edge(&2, 3), 6);
        assert_eq!(*connections.get_edge(&1, 2), 7);
    }

    #[test]
    fn test_sources() {
        let mut connections = Connections::<u8, ()>::new();

        connections.add(0, 1, ());
        connections.add(1, 3, ());
        connections.add(0, 2, ());
        connections.add(2, 3, ());
        connections.add(2, 1, ());
        connections.add(3, 4, ());

        let mut all_sources = connections.get_sources().cloned().collect::<Vec<u8>>();
        all_sources.sort();

        assert_eq!(all_sources, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_remove() {
        let mut connections = Connections::<u8, ()>::new();

        connections.add(0, 1, ());
        connections.add(1, 2, ());
        connections.remove(&0, 1);
        assert!(!connections.contains(&0, 1));
        connections.add(2, 0, ());
        connections.remove(&2, 0);
        assert!(!connections.contains(&2, 0));
    }

    #[test]
    fn test_cycle() {
        let mut connections = Connections::<u8, ()>::new();

        connections.add(0, 1, ());
        connections.add(1, 2, ());

        assert!(connections.creates_cycle(0, 0));
        assert!(connections.creates_cycle(3, 3));
        assert!(connections.creates_cycle(2, 0));
        assert!(!connections.creates_cycle(2, 3));
        assert!(!connections.creates_cycle(0, 2));
    }

    #[test]
    fn test_sort() {
        let mut connections = Connections::<u8, ()>::new();
        connections.add(0, 1, ());
        connections.add(1, 3, ());
        connections.add(0, 2, ());
        connections.add(2, 3, ());
        connections.add(2, 1, ());

        let order = connections.sort_topologically();
        let pos = |x: u8| {
            order
                .iter()
                .position(|y| *y == OrderedAction::Activation(x))
                .unwrap()
        };

        assert!(pos(0) < pos(1));
        assert!(pos(1) < pos(3));
        assert!(pos(0) < pos(2));
        assert!(pos(2) < pos(3));
        assert!(pos(2) < pos(1));
    }
}
