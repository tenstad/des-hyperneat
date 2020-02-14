use crate::network::connection::Connections;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

/// Topologically sorted list of actions to perform when evalutating network
#[derive(Clone)]
pub struct Order<T> {
    inputs: Vec<T>,
    outputs: Vec<T>,
    pub hiddens: Vec<T>,
    actions: Vec<Action<T>>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Action<T> {
    Link(T, T),
    Activation(T),
}

#[allow(dead_code)]
impl<T: Hash + Eq + Copy> Order<T> {
    pub fn new() -> Order<T> {
        Order {
            inputs: Vec::new(),
            outputs: Vec::new(),
            hiddens: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn from_nodes(inputs: Vec<T>, hiddens: Vec<T>, outputs: Vec<T>) -> Order<T> {
        let actions = inputs
            .iter()
            .map(|x| Action::Activation(*x))
            .chain(hiddens.iter().map(|x| Action::Activation(*x)))
            .chain(outputs.iter().map(|x| Action::Activation(*x)))
            .collect();

        Order {
            inputs,
            outputs,
            hiddens,
            actions,
        }
    }

    pub fn add_input(&mut self, node: T) {
        self.inputs.push(node);
        // New inputs are inserted at the start
        self.actions.insert(0, Action::Activation(node));
    }

    pub fn add_output(&mut self, node: T) {
        self.outputs.push(node);
        // New outputs are appended at the end
        self.actions.push(Action::Activation(node));
    }

    pub fn add_hidden(&mut self, node: T) {
        // A new node is not connected, so it is valid in all locations
        // It should however be placed where a link to/from it is likely to be valid without reordering
        self.hiddens.push(node);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Action<T>> {
        self.actions.iter()
    }

    pub fn contains(&self, action: &Action<T>) -> bool {
        self.actions.contains(action)
    }

    pub fn add_link(&mut self, from: T, to: T, connections: &Connections<T>) {
        for (i, action) in self.actions.iter().enumerate() {
            if let Action::Activation(node_ref) = action {
                if from == *node_ref {
                    self.actions.insert(i + 1, Action::Link(from, to));
                    break;
                } else if to == *node_ref {
                    // If iteration hits target before source, the topological order needs to be altered
                    // Might be a fast way to do it, instead of redoing the entire order
                    self.sort_topologically(connections);
                    break;
                }
            }
        }
    }

    pub fn split_link(&mut self, from: T, to: T, new: T) {
        let mut skip = 0;

        // Insert link between 'from' and 'new' after 'from'-activation
        for (i, action) in self.actions.iter().enumerate() {
            if let Action::Activation(node_ref) = action {
                if from == *node_ref {
                    self.actions.insert(i + 1, Action::Link(from, new));
                    skip = i + 2;
                    break;
                }
            }
        }

        // Insert new node activation before next activation,
        // followed by link between new node and 'to'-node
        for (i, action) in self.actions.iter().skip(skip).enumerate() {
            if let Action::Activation(_) = action {
                self.actions.insert(i + skip, Action::Activation(new));
                self.actions.insert(i + skip + 1, Action::Link(new, to));
                break;
            }
        }

        self.remove_link(from, to);
    }

    pub fn remove_link(&mut self, from: T, to: T) {
        let index = self
            .actions
            .iter()
            .position(|x| *x == Action::Link(from, to))
            .expect("Link action does not exist");
        self.actions.remove(index);
    }

    /// Determine order of nodes and links to actiave in forward pass
    ///
    /// To allow for insertion of new links without finding new topological sorting,
    /// all nodes must be included, even though they are not currently connected.
    pub fn sort_topologically(&mut self, connections: &Connections<T>) {
        self.actions.clear();

        // Store number of incoming connections for all nodes
        // Ignore disabled connections
        let mut backward_count: HashMap<T, u64> = HashMap::new();
        for source in connections.enabled_sources() {
            for target in connections.get_enabled(source) {
                backward_count.insert(*target, *backward_count.get(target).unwrap_or(&0) + 1);
            }
        }

        // Add all input and hidden nodes without incoming connections to stack
        // Output nodes without incoming connections will be appended at the end
        let mut stack: Vec<T> = self
            .hiddens
            .iter()
            .filter(|node| *backward_count.get(node).unwrap_or(&0) == 0)
            .map(|x| *x)
            .collect();
        stack.extend(self.inputs.iter());

        // Outputs added at the end
        let append_outputs: Vec<T> = self
            .outputs
            .iter()
            .filter(|node| *backward_count.get(node).unwrap_or(&0) == 0)
            .map(|x| *x)
            .collect();

        // Create topological order
        while let Some(node) = stack.pop() {
            self.actions.push(Action::Activation(node));

            // Process all outgoing connections from the current node
            for to in connections.get_enabled(&node) {
                self.actions.push(Action::Link(node, *to));

                // Reduce backward count by 1
                backward_count.insert(*to, *backward_count.get(to).unwrap_or(&0) - 1);

                // Add nodes with no incoming connections to the stack
                if *backward_count.get(to).unwrap_or(&0) == 0 {
                    stack.push(*to);
                }
            }
        }

        // Add non-connected output nodes
        // These are added at the end instead of the stack to increase
        // the probability of new links beeing in valid topological order
        self.actions
            .extend(append_outputs.iter().map(|x| Action::Activation(*x)));
    }
}

impl<T: Hash + Eq + Copy + fmt::Display> fmt::Display for Order<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Actions[")?;
        for action in self.iter() {
            write!(f, " {}", action)?;
        }
        write!(f, " ]")
    }
}

impl<T: fmt::Display> fmt::Display for Action<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Link(from, to) => write!(f, "{}->{}", from, to),
            Action::Activation(id) => write!(f, "{}", id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_equal<T: Hash + Eq + Copy + fmt::Debug>(order: Order<T>, target: Vec<Action<T>>) {
        let mut order = order.iter();
        let mut target = target.iter();
        while let (Some(a), Some(t)) = (order.next(), target.next()) {
            assert_eq!(a, t);
        }
    }

    #[test]
    fn test_from_node() {
        let order = Order::<u8>::from_nodes(vec![0, 1], vec![3, 4], vec![6, 7]);

        assert_equal(
            order,
            vec![
                Action::Activation(0),
                Action::Activation(1),
                Action::Activation(3),
                Action::Activation(4),
                Action::Activation(6),
                Action::Activation(7),
            ],
        );
    }

    #[test]
    fn test_add_remove_link() {
        let connections = Connections::<u8>::new();

        let mut order = Order::<u8>::from_nodes(vec![0], vec![1], vec![2]);
        order.add_link(0, 1, &connections);
        order.add_link(0, 2, &connections);

        assert_equal(
            order.clone(),
            vec![
                Action::Activation(0),
                Action::Link(0, 2),
                Action::Link(0, 1),
                Action::Activation(1),
                Action::Activation(2),
            ],
        );

        order.remove_link(0, 2);
        assert_equal(
            order,
            vec![
                Action::Activation(0),
                Action::Link(0, 1),
                Action::Activation(1),
                Action::Activation(2),
            ],
        );
    }

    #[test]
    fn test_split_link() {
        let connections = Connections::<u8>::new();

        let mut order = Order::<u8>::from_nodes(vec![0], vec![1], vec![2]);
        order.add_link(0, 1, &connections);
        order.split_link(0, 1, 3);

        assert_equal(
            order,
            vec![
                Action::Activation(0),
                Action::Link(0, 3),
                Action::Activation(3),
                Action::Link(3, 1),
                Action::Activation(1),
                Action::Activation(2),
            ],
        );
    }

    #[test]
    fn test_unreachable() {
        let mut connections = Connections::<u8>::new();
        connections.add_enabled(1, 2);

        let mut order = Order::<u8>::from_nodes(vec![0], vec![1, 2], vec![3]);
        order.add_link(1, 2, &connections);

        order.sort_topologically(&connections);

        assert_equal(
            order,
            vec![
                Action::Activation(0),
                Action::Activation(1),
                Action::Link(1, 2),
                Action::Activation(2),
                Action::Activation(3),
            ],
        );
    }

    #[test]
    fn test_add_link_causing_sort() {
        let mut connections = Connections::<u8>::new();
        connections.add_enabled(0, 1);
        connections.add_enabled(1, 3);
        connections.add_enabled(0, 2);
        connections.add_enabled(2, 3);
        connections.add_enabled(2, 1);

        let mut order = Order::<u8>::from_nodes(vec![0], vec![1, 2], vec![3]);
        order.add_link(0, 1, &connections);
        order.add_link(1, 3, &connections);
        order.add_link(0, 2, &connections);
        order.add_link(2, 3, &connections);

        order.add_link(2, 1, &connections);

        let pos = |x: u8| {
            order
                .iter()
                .position(|y| *y == Action::Activation(x))
                .unwrap()
        };

        assert!(pos(0) < pos(1));
        assert!(pos(0) < pos(2));
        assert!(pos(2) < pos(1)); // Order of 1 and 2 has changed
        assert!(pos(1) < pos(3));
        assert!(pos(2) < pos(3));
    }
}
