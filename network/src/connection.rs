use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;

#[derive(Clone, Debug, new)]
pub struct Target<N, E> {
    pub node: N,
    pub edge: E,
}

#[derive(Clone, Debug, new)]
pub struct Connection<N, E> {
    pub from: N,
    pub to: N,
    pub edge: E,
}

#[derive(PartialEq)]
pub enum OrderedAction<N, E> {
    Edge(N, N, E),
    Node(N),
}

impl<N: fmt::Display, E: fmt::Display> fmt::Display for OrderedAction<N, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderedAction::Edge(from, to, edge) => write!(f, "{}-({})>{}", from, edge, to),
            OrderedAction::Node(id) => write!(f, "{}", id),
        }
    }
}

/// Fast non-cyclic graph structure
#[derive(Clone)]
pub struct Connections<N: Hash, E> {
    connections: HashMap<N, Vec<Target<N, E>>>,
}

#[allow(dead_code)]
impl<N: Hash + Eq + Copy, E: Copy> Connections<N, E> {
    pub fn new() -> Self {
        Self {
            connections: HashMap::<N, Vec<Target<N, E>>>::new(),
        }
    }

    pub fn add(&mut self, from: N, to: N, edge: E) {
        assert!(
            !self.creates_cycle(from, to),
            "cannot add link that creates cycle"
        );
        assert!(!self.contains(&from, to), "cannot add existing connection");

        if let Some(vec) = self.connections.get_mut(&from) {
            vec.push(Target::<N, E>::new(to, edge));
        } else {
            self.connections
                .insert(from, vec![Target::<N, E>::new(to, edge)]);
        }
    }

    pub fn extend(&mut self, other: &Self) {
        for source in other.get_sources() {
            for target in other.get_edges(source) {
                self.add(*source, target.node, target.edge);
            }
        }
    }

    pub fn set_edge<'a>(&mut self, from: &'a N, to: N, edge: E) {
        let error = "cannot set non-existent edge";
        let edges = self.connections.get_mut(from).expect(error);
        let index = edges.iter().position(|t| t.node == to).expect(error);
        edges[index].edge = edge;
    }

    pub fn get_edge<'a>(&'a self, from: &'a N, to: N) -> &'a E {
        let error = "cannot get non-existent edge";
        let edges = self.connections.get(from).expect(error);
        let index = edges.iter().position(|t| t.node == to).expect(error);
        &edges[index].edge
    }

    pub fn get_all_connections(&self) -> Vec<Connection<N, E>> {
        self.connections
            .iter()
            .flat_map(|(source, targets)| {
                targets
                    .iter()
                    .cloned()
                    .map(move |target| Connection::new(*source, target.node, target.edge))
            })
            .collect()
    }

    pub fn get_all_nodes(&self) -> Vec<N> {
        self.get_all_connections()
            .iter()
            .flat_map(|connection| {
                std::iter::once(connection.from).chain(std::iter::once(connection.to))
            })
            .collect::<HashSet<N>>()
            .iter()
            .cloned()
            .collect()
    }

    pub fn get_edges<'a>(&'a self, from: &'a N) -> impl Iterator<Item = &'a Target<N, E>> {
        self.connections.get(from).into_iter().flatten()
    }

    pub fn get_targets<'a>(&'a self, from: &'a N) -> impl Iterator<Item = &'a N> {
        self.get_edges(from).map(|t| &t.node)
    }

    pub fn get_sources<'a>(&'a self) -> impl Iterator<Item = &'a N> {
        self.connections.keys()
    }

    pub fn contains(&self, from: &N, to: N) -> bool {
        !self.get_edges(from).position(|t| t.node == to).is_none()
    }

    pub fn remove(&mut self, from: &N, to: N) -> E {
        let error = "cannot remove non-existent connection";
        let vec = self.connections.get_mut(from).expect(error);
        let index = vec.iter().position(|t| t.node == to).expect(error);
        vec.swap_remove(index).edge
    }

    /// DFS search to check for cycles.
    ///
    /// If 'from' is reachable from 'to', then addition will cause cycle
    pub fn creates_cycle(&self, from: N, to: N) -> bool {
        let mut visited: HashSet<N> = [to].iter().cloned().collect();
        let mut stack: Vec<N> = vec![to];

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

        return false; // Enable to reach from when starting at to, addition will not cause cycle
    }

    /// Determine order of nodes and links to actiave in forward pass
    pub fn sort_topologically(&self) -> Vec<OrderedAction<N, E>> {
        // Store number of incoming connections for all nodes
        let mut backward_count: HashMap<N, u64> = HashMap::new();
        for (_, targets) in self.connections.iter() {
            for target in targets.iter() {
                backward_count.insert(
                    target.node,
                    *backward_count.get(&target.node).unwrap_or(&0) + 1,
                );
            }
        }

        // Start search from all nodes without incoming connections
        let mut stack: Vec<N> = self
            .connections
            .keys()
            .filter(|node| *backward_count.get(node).unwrap_or(&0) == 0)
            .cloned()
            .collect();

        let mut actions = Vec::<OrderedAction<N, E>>::new();

        // Create topological order
        while let Some(node) = stack.pop() {
            actions.push(OrderedAction::Node(node));

            // Process all outgoing connections from the current node
            for target in self.get_edges(&node) {
                actions.push(OrderedAction::Edge(node, target.node, target.edge));

                // Reduce backward count by 1
                backward_count.insert(target.node, *backward_count.get(&target.node).unwrap() - 1);

                // Add nodes with no incoming connections to the stack
                if *backward_count.get(&target.node).unwrap() == 0 {
                    stack.push(target.node);
                }
            }
        }

        actions
    }

    pub fn prune(&mut self, inputs: &Vec<N>, outputs: &Vec<N>) {
        self.prune_dangling_inputs(inputs);
        self.prune_dangling_outputs(outputs);
    }

    pub fn prune_dangling_inputs(&mut self, inputs: &Vec<N>) {
        let mut backward_count: HashMap<N, u64> = HashMap::new();
        for (_, targets) in self.connections.iter() {
            for target in targets.iter() {
                backward_count.insert(
                    target.node,
                    *backward_count.get(&target.node).unwrap_or(&0) + 1,
                );
            }
        }

        loop {
            let dangling_inputs = self
                .connections
                .keys()
                .filter(|n| !inputs.contains(n) && *backward_count.get(n).unwrap_or(&0) == 0)
                .cloned()
                .collect::<Vec<N>>();
            if dangling_inputs.len() == 0 {
                break;
            }
            for node in dangling_inputs.iter() {
                backward_count.remove(node);
                for target in self.connections.get(node).unwrap().iter() {
                    backward_count
                        .insert(target.node, *backward_count.get(&target.node).unwrap() - 1);
                }
                self.connections.remove(node);
            }
        }
    }

    pub fn prune_dangling_outputs(&mut self, outputs: &Vec<N>) {
        loop {
            let mut deleted_node = false;
            for source in self.get_sources().cloned().collect::<Vec<N>>().iter() {
                let targets = self.connections.get(source).unwrap();
                let delete_indexes = (0..targets.len())
                    .rev()
                    .filter(|i| {
                        !outputs.contains(&targets[*i].node)
                            && !self.connections.contains_key(&targets[*i].node)
                    })
                    .collect::<Vec<usize>>();
                if delete_indexes.len() > 0 {
                    deleted_node = true;
                    if delete_indexes.len() == targets.len() {
                        self.connections.remove(source);
                    } else {
                        let targets = self.connections.get_mut(source).unwrap();
                        for delete_index in delete_indexes.iter() {
                            targets.swap_remove(*delete_index);
                        }
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
            .map(|t| (t.node, t.edge))
            .collect::<Vec<(u8, u8)>>();
        targets.sort();

        assert_eq!(targets, vec![(1, 5), (2, 6), (3, 7)]);
        assert_eq!(connections.get_edges(&5).count(), 0);
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
                .position(|y| *y == OrderedAction::Node(x))
                .unwrap()
        };

        assert!(pos(0) < pos(1));
        assert!(pos(1) < pos(3));
        assert!(pos(0) < pos(2));
        assert!(pos(2) < pos(3));
        assert!(pos(2) < pos(1));
    }

    #[test]
    fn test_dangeling_inputs() {
        let mut connections = Connections::<u8, ()>::new();

        connections.add(5, 10, ());
        connections.add(6, 10, ());
        connections.add(7, 10, ());
        connections.add(0, 5, ());
        connections.add(1, 5, ());
        connections.add(2, 6, ());
        connections.add(3, 10, ());
        connections.add(4, 0, ());
        connections.add(4, 1, ());
        connections.add(10, 11, ());
        connections.add(10, 12, ());
        connections.add(10, 13, ());
        connections.add(11, 12, ());
        connections.add(14, 12, ());

        connections.prune_dangling_inputs(&vec![3]);
        assert!(!connections.contains(&5, 10));
        assert!(!connections.contains(&6, 10));
        assert!(!connections.contains(&7, 10));
        assert!(!connections.contains(&0, 5));
        assert!(!connections.contains(&1, 5));
        assert!(!connections.contains(&2, 6));
        assert!(connections.contains(&3, 10));
        assert!(!connections.contains(&4, 0));
        assert!(!connections.contains(&4, 1));
        assert!(connections.contains(&10, 11));
        assert!(connections.contains(&10, 12));
        assert!(connections.contains(&10, 13));
        assert!(connections.contains(&11, 12));
        assert!(!connections.contains(&14, 12));
    }

    #[test]
    fn test_dangeling_outputs() {
        let mut connections = Connections::<u8, ()>::new();

        connections.add(10, 5, ());
        connections.add(10, 6, ());
        connections.add(10, 7, ());
        connections.add(5, 0, ());
        connections.add(5, 1, ());
        connections.add(6, 2, ());
        connections.add(10, 3, ());
        connections.add(0, 4, ());
        connections.add(1, 4, ());
        connections.add(11, 10, ());
        connections.add(12, 10, ());
        connections.add(13, 10, ());
        connections.add(12, 11, ());
        connections.add(12, 14, ());

        connections.prune_dangling_outputs(&vec![3]);
        assert!(!connections.contains(&10, 5));
        assert!(!connections.contains(&10, 6));
        assert!(!connections.contains(&10, 7));
        assert!(!connections.contains(&5, 0));
        assert!(!connections.contains(&5, 1));
        assert!(!connections.contains(&6, 2));
        assert!(connections.contains(&10, 3));
        assert!(!connections.contains(&0, 4));
        assert!(!connections.contains(&1, 4));
        assert!(connections.contains(&11, 10));
        assert!(connections.contains(&12, 10));
        assert!(connections.contains(&13, 10));
        assert!(connections.contains(&12, 11));
        assert!(!connections.contains(&12, 14));
    }
}
