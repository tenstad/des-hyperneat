use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

/// Fast structure for looking up all outgoing connections from a node,
/// either enabled, disabled or both. Can also check if addition creates cycle.
#[derive(Clone)]
pub struct Connections<T: Hash> {
    enabled: HashMap<T, Vec<T>>,
    disabled: HashMap<T, Vec<T>>,
}

#[allow(dead_code)]
impl<T: Hash + Eq + Copy> Connections<T> {
    // New
    pub fn new() -> Connections<T> {
        Connections {
            enabled: HashMap::<T, Vec<T>>::new(),
            disabled: HashMap::<T, Vec<T>>::new(),
        }
    }

    // Add
    pub fn add(&mut self, from: T, to: T, enabled: bool) {
        if enabled {
            self.add_enabled(from, to);
        } else {
            self.add_disabled(from, to);
        }
    }

    pub fn add_enabled(&mut self, from: T, to: T) {
        assert!(
            !self.contains_disabled(&from, to),
            "Cannot add existing connection."
        );
        assert!(
            !self.creates_cycle(from, to),
            "Cannot add link that creates cycle"
        );
        add_connection(&mut self.enabled, from, to);
    }

    pub fn add_disabled(&mut self, from: T, to: T) {
        assert!(
            !self.contains_enabled(&from, to),
            "Cannot add existing connection."
        );
        assert!(
            !self.creates_cycle(from, to),
            "Cannot add link that creates cycle"
        );
        add_connection(&mut self.disabled, from, to);
    }

    // Get
    pub fn get<'a>(&'a self, from: &'a T) -> impl Iterator<Item = &'a T> {
        self.get_enabled(from).chain(self.get_disabled(from))
    }

    pub fn get_enabled<'a>(&'a self, from: &'a T) -> impl Iterator<Item = &'a T> {
        get_connections(&self.enabled, from)
    }

    pub fn get_disabled<'a>(&'a self, from: &'a T) -> impl Iterator<Item = &'a T> {
        get_connections(&self.disabled, from)
    }

    pub fn enabled_sources<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.enabled.keys()
    }

    // Contains
    pub fn contains(&self, from: &T, to: T) -> bool {
        self.contains_enabled(from, to) || self.contains_disabled(from, to)
    }

    pub fn contains_enabled(&self, from: &T, to: T) -> bool {
        contains(&self.enabled, from, to)
    }

    pub fn contains_disabled(&self, from: &T, to: T) -> bool {
        contains(&self.disabled, from, to)
    }

    // Toggle
    pub fn disable(&mut self, from: T, to: T) {
        self.remove_enabled(&from, to);
        self.add_disabled(from, to);
    }

    pub fn enable(&mut self, from: T, to: T) {
        self.remove_disabled(&from, to);
        self.add_enabled(from, to);
    }

    // Remove
    pub fn remove(&mut self, from: &T, to: T, enabled: bool) {
        if enabled {
            self.remove_enabled(from, to);
        } else {
            self.remove_disabled(from, to);
        }
    }

    pub fn remove_enabled(&mut self, from: &T, to: T) {
        remove_connection(&mut self.enabled, from, to);
    }

    pub fn remove_disabled(&mut self, from: &T, to: T) {
        remove_connection(&mut self.disabled, from, to);
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
                // Walk both enabled and disabled connections because disabled might become enabled later on
                stack.extend(self.get(&node).filter(|n| !visited.contains(n)));
                visited.extend(stack.iter().skip(l));
            }
        }

        return false; // Unable to reach from when starting at to, addition will not cause cycle
    }
}

fn add_connection<T: Hash + Eq>(connections: &mut HashMap<T, Vec<T>>, from: T, to: T) {
    if let Some(vec) = connections.get_mut(&from) {
        assert!(!vec.contains(&to), "Cannot add existing connection.");
        vec.push(to);
    } else {
        connections.insert(from, vec![to]);
    }
}

fn get_connections<'a, T: Hash + Eq>(
    connections: &'a HashMap<T, Vec<T>>,
    from: &'a T,
) -> impl Iterator<Item = &'a T> {
    connections.get(from).into_iter().flatten()
}

fn remove_connection<T: Hash + Eq>(connections: &mut HashMap<T, Vec<T>>, from: &T, to: T) {
    let error = "Cannot remove non-existent connection.";
    let vec = connections.get_mut(from).expect(error);
    let index = vec.iter().position(|x| *x == to).expect(error);
    vec.swap_remove(index);
}

fn contains<T: Hash + Eq>(connections: &HashMap<T, Vec<T>>, from: &T, to: T) -> bool {
    !get_connections(connections, from)
        .position(|x| *x == to)
        .is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut connections = Connections::<u8>::new();

        connections.add_enabled(0, 1);
        connections.add(0, 2, true);
        connections.add_disabled(1, 2);
        connections.add(1, 3, false);

        assert!(connections.contains(&0, 1));
        assert!(connections.contains(&1, 2));
        assert!(connections.contains(&0, 2));
        assert!(connections.contains(&1, 3));

        assert!(connections.contains_enabled(&0, 1));
        assert!(connections.contains_enabled(&0, 2));
        assert!(!connections.contains_disabled(&0, 1));
        assert!(!connections.contains_disabled(&0, 2));

        assert!(connections.contains_disabled(&1, 2));
        assert!(connections.contains_disabled(&1, 3));
        assert!(!connections.contains_enabled(&1, 2));
        assert!(!connections.contains_enabled(&1, 3));
    }

    #[test]
    fn test_get() {
        let mut connections = Connections::<u8>::new();

        connections.add_enabled(0, 1);
        connections.add_enabled(0, 2);
        connections.add_disabled(0, 3);

        let mut all_targets = connections.get(&0).map(|x| *x).collect::<Vec<u8>>();
        let mut enabled_targets = connections.get_enabled(&0).map(|x| *x).collect::<Vec<u8>>();
        let mut disabled_targets = connections
            .get_disabled(&0)
            .map(|x| *x)
            .collect::<Vec<u8>>();

        all_targets.sort();
        enabled_targets.sort();
        disabled_targets.sort();

        assert_eq!(all_targets, vec![1, 2, 3]);
        assert_eq!(enabled_targets, vec![1, 2]);
        assert_eq!(disabled_targets, vec![3]);
        assert_eq!(connections.get(&5).next(), None);
    }

    #[test]
    fn test_sources() {
        let mut connections = Connections::<u8>::new();

        connections.add_enabled(0, 1);
        connections.add_enabled(1, 3);
        connections.add_enabled(0, 2);
        connections.add_enabled(2, 3);
        connections.add_enabled(2, 1);
        connections.add_disabled(3, 4);

        let mut all_sources = connections.enabled_sources().map(|x| *x).collect::<Vec<u8>>();
        all_sources.sort();

        assert_eq!(all_sources, vec![0, 1, 2]);
    }

    #[test]
    fn test_toggle() {
        let mut connections = Connections::<u8>::new();

        connections.add_enabled(0, 1);
        connections.disable(0, 1);

        assert!(connections.contains_disabled(&0, 1));
        assert!(!connections.contains_enabled(&0, 1));

        connections.enable(0, 1);

        assert!(connections.contains_enabled(&0, 1));
        assert!(!connections.contains_disabled(&0, 1));
    }

    #[test]
    fn test_remove() {
        let mut connections = Connections::<u8>::new();

        connections.add_enabled(0, 1);
        connections.add(0, 2, true);
        connections.remove_enabled(&0, 1);
        connections.remove(&0, 2, true);
        assert!(!connections.contains(&0, 1));
        assert!(!connections.contains(&0, 2));

        connections.add_disabled(0, 1);
        connections.add(0, 2, false);
        connections.remove_disabled(&0, 1);
        connections.remove(&0, 2, false);
        assert!(!connections.contains(&0, 1));
        assert!(!connections.contains(&0, 2));
    }

    #[test]
    fn test_cycle() {
        let mut connections = Connections::<u8>::new();

        connections.add_enabled(0, 1);
        connections.add_disabled(1, 2);

        assert!(connections.creates_cycle(0, 0));
        assert!(connections.creates_cycle(3, 3));
        assert!(connections.creates_cycle(2, 0));
        assert!(!connections.creates_cycle(2, 3));
        assert!(!connections.creates_cycle(0, 2));
    }
}
