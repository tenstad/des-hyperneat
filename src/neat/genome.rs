use crate::neat::nodes::*;
use crate::neat::population::InnovationLog;
use crate::neat::population::InnovationTime;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

pub struct Genome {
    pub inputs: HashMap<NodeRef, InputNode>,
    pub outputs: HashMap<NodeRef, OutputNode>,
    hidden_nodes: HashMap<NodeRef, HiddenNode>,
    pub links: HashMap<(NodeRef, NodeRef), Link>, // Links between nodes

    actions: Vec<Action>, // Actions to perform when evaluating
    connections: HashMap<NodeRef, Vec<NodeRef>>, // List of connections to other nodes (for faster lookup)
}

/// Link between two nodes
#[derive(Copy, Clone)]
pub struct Link {
    pub from: NodeRef,
    pub to: NodeRef,
    pub weight: f64,
    pub enabled: bool,
    pub innovation: u64, // Global innovation number
}

impl Link {
    fn crossover(&self, other: &Link) -> Link {
        assert_eq!(self.innovation, other.innovation);

        Link {
            from: self.from,
            to: self.to,
            weight: (self.weight + other.weight) / 2.0,
            enabled: self.enabled && other.enabled,
            innovation: self.innovation,
        }
    }

    fn distance(&self, other: &Link) -> f64 {
        (self.weight - other.weight).tanh().abs()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Action {
    Link(NodeRef, NodeRef),
    Activation(NodeRef),
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Link(from, to) => write!(f, "Link({}, {})", from, to),
            Action::Activation(id) => write!(f, "Activation({})", id),
        }
    }
}

impl Genome {
    /// Generate genome with default activation and no connections
    pub fn new(inputs: u64, outputs: u64) -> Genome {
        let mut actions: Vec<Action> = (0..inputs)
            .map(|i| Action::Activation(NodeRef::Input(i)))
            .collect();
        let output_actions: Vec<Action> = (0..outputs)
            .map(|i| Action::Activation(NodeRef::Output(i)))
            .collect();
        actions.extend(output_actions);

        return Genome {
            inputs: (0..inputs)
                .map(|i| (NodeRef::Input(i), InputNode { id: i }))
                .collect(),
            outputs: (0..outputs)
                .map(|i| {
                    (
                        NodeRef::Output(i),
                        OutputNode {
                            id: i,
                            activation: Activation::None,
                        },
                    )
                })
                .collect(),
            hidden_nodes: HashMap::new(), // No hidden nodes
            links: HashMap::new(),        // No links between nodes
            actions: actions,
            connections: HashMap::new(),
        };
    }

    fn split_link(&mut self, link: Link, new_node_id: u64, innovation_number: u64) {
        // Disable link
        if let Some(link) = self.links.get_mut(&(link.from, link.to)) {
            link.enabled = false;
        }

        // Remove connection
        if let Some(vec) = self.connections.get_mut(&link.from) {
            if let Some(index) = vec.iter().position(|x| *x == link.to) {
                vec.swap_remove(index);
            }
        }

        // Remove action
        if let Some(index) = self
            .actions
            .iter()
            .position(|x| *x == Action::Link(link.from, link.to))
        {
            self.actions.remove(index);
        }

        let new_node_ref = NodeRef::Hidden(new_node_id);

        self.hidden_nodes.insert(
            new_node_ref,
            HiddenNode {
                id: new_node_id,
                activation: Activation::None,
            },
        );

        let link1 = Link {
            from: link.from,
            to: new_node_ref,
            weight: 1.0,
            enabled: true,
            innovation: innovation_number,
        };

        let link2 = Link {
            from: new_node_ref,
            to: link.to,
            weight: link.weight,
            enabled: true,
            innovation: innovation_number + 1,
        };

        self.insert_link(link1, false);
        self.insert_link(link2, false);

        let mut skip = 0;

        // Insert link between 'from' and new node after 'from'-activation
        for (i, action) in self.actions.iter().enumerate() {
            if let Action::Activation(node_ref) = action {
                if link.from == *node_ref {
                    self.actions
                        .insert(i + 1, Action::Link(link1.from, link1.to));
                    skip = i + 2;
                    break;
                }
            }
        }

        // Insert new node activation before next activation,
        // followed by link between new node and 'to'-node
        for (i, action) in self.actions.iter().skip(skip).enumerate() {
            if let Action::Activation(_) = action {
                self.actions.insert(i + skip, Action::Activation(link1.to));
                self.actions
                    .insert(i + skip + 1, Action::Link(link2.from, link2.to));
                break;
            }
        }
    }

    fn insert_link(&mut self, link: Link, add_action: bool) {
        // Add link
        self.links.insert((link.from, link.to), link);

        // Link should only be included in network if it's enabled
        if link.enabled {
            // Add connection
            if let Some(vec) = self.connections.get_mut(&link.from) {
                vec.push(link.to);
            } else {
                self.connections.insert(link.from, vec![link.to]);
            }

            // Add action
            if add_action {
                for (i, action) in self.actions.iter().enumerate() {
                    if let Action::Activation(node_ref) = action {
                        if link.to == *node_ref {
                            // If iteration hits target before source, the topological order needs to be altered
                            // Might be a fast way to do it, instead of redoing the entire order
                            self.sort_actions_topologically();
                            break;
                        } else if link.from == *node_ref {
                            self.actions.insert(i + 1, Action::Link(link.from, link.to));
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn crossover(&self, other: &Self, is_fitter: bool) -> Genome {
        // Let parent1 be the fitter parent
        let (parent1, parent2) = if is_fitter {
            (self, other)
        } else {
            (other, self)
        };

        let mut genome = Genome {
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            hidden_nodes: HashMap::new(),
            links: HashMap::new(),
            actions: Vec::new(),
            connections: HashMap::new(),
        };

        // Copy links only in fitter parent, perform crossover if in both parents
        for (link_ref, link) in parent1.links.iter() {
            if let Some(link2) = parent2.links.get(link_ref) {
                genome.insert_link(link.crossover(link2), false);
            } else {
                genome.insert_link(*link, false);
            }
        }

        // Copy nodes only in fitter parent, perform crossover if in both parents
        for (node_ref, node) in parent1.inputs.iter() {
            if let Some(node2) = parent2.inputs.get(node_ref) {
                genome.inputs.insert(*node_ref, node.crossover(node2));
            } else {
                genome.inputs.insert(*node_ref, *node);
            }
        }

        for (node_ref, node) in parent1.hidden_nodes.iter() {
            if let Some(node2) = parent2.hidden_nodes.get(node_ref) {
                genome.hidden_nodes.insert(*node_ref, node.crossover(node2));
            } else {
                genome.hidden_nodes.insert(*node_ref, *node);
            }
        }

        for (node_ref, node) in parent1.outputs.iter() {
            if let Some(node2) = parent2.outputs.get(node_ref) {
                genome.outputs.insert(*node_ref, node.crossover(node2));
            } else {
                genome.outputs.insert(*node_ref, *node);
            }
        }

        // Topologically sort actions of child, as this is not done when inserting links and nodes
        genome.sort_actions_topologically();

        return genome;
    }

    pub fn mutate(&mut self, log: &mut InnovationLog, global_innovation: &mut InnovationTime) {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < 0.2 {
            self.mutation_add_node(log, global_innovation);
        }

        if rng.gen::<f64>() < 0.2 {
            self.mutation_add_connection(log, global_innovation);
        }
    }

    fn mutation_add_node(
        &mut self,
        log: &mut InnovationLog,
        global_innovation: &mut InnovationTime,
    ) {
        // Select random enabled link
        if let Some(index) = self
            .links
            .iter()
            .filter(|(_, link)| link.enabled)
            .map(|(i, _)| *i)
            .collect::<Vec<_>>()
            .choose(&mut rand::thread_rng())
        {
            if let Some(&link) = self.links.get(index) {
                // Check if this link has been split by another individual
                if let Some(addition) = log.node_additions.get(&link.innovation) {
                    // Split the link
                    self.split_link(link, addition.node_number, addition.innovation_number);
                } else {
                    // Split the link
                    self.split_link(
                        link,
                        global_innovation.node_number,
                        global_innovation.innovation_number,
                    );

                    // Add this mutation to log
                    log.node_additions.insert(
                        link.innovation,
                        InnovationTime {
                            node_number: global_innovation.node_number,
                            innovation_number: global_innovation.innovation_number,
                        },
                    );

                    // Increase global node count and innovation number
                    global_innovation.node_number += 1;
                    global_innovation.innovation_number += 2;
                }
            }
        }
    }

    // TODO: avoid retries
    fn mutation_add_connection(
        &mut self,
        log: &mut InnovationLog,
        global_innovation: &mut InnovationTime,
    ) {
        let mut rng = rand::thread_rng();

        // Retry 50 times
        for _ in 0..50 {
            // Select random source and target nodes for new link
            let from_index = rng.gen_range(0, self.inputs.len() + self.hidden_nodes.len());
            let to_index = rng.gen_range(0, self.hidden_nodes.len() + self.outputs.len());

            let from_option = if from_index < self.inputs.len() {
                self.inputs.keys().skip(from_index).next()
            } else {
                self.hidden_nodes
                    .keys()
                    .skip(from_index - self.inputs.len())
                    .next()
            };
            let to_option = if to_index < self.outputs.len() {
                self.outputs.keys().skip(to_index).next()
            } else {
                self.hidden_nodes
                    .keys()
                    .skip(to_index - self.outputs.len())
                    .next()
            };

            if let (Some(&from), Some(&to)) = (from_option, to_option) {
                // If connection does not exist and its addition does not create cycle
                if !self.links.contains_key(&(from, to)) && !self.creates_cycle(from, to) {
                    // Check if this link has been added by another individual
                    let innovation = match log.edge_additions.get(&(from, to)) {
                        Some(innovation_number) => *innovation_number,
                        None => {
                            log.edge_additions
                                .insert((from, to), global_innovation.innovation_number);
                            global_innovation.innovation_number += 1;

                            global_innovation.innovation_number - 1
                        }
                    };

                    self.insert_link(
                        Link {
                            from,
                            to,
                            weight: 1.0,
                            enabled: true,
                            innovation,
                        },
                        true,
                    );
                    break;
                }
            } else {
                break;
            }
        }
    }

    // Genetic distance between two genomes
    pub fn distance(&self, other: &Self) -> f64 {
        let mut link_differences: u64 = 0; // Number of links present in only one of the genomes
        let mut link_distance: f64 = 0.0; // Total distance between links present in both genomes
        let mut link_count = self.links.len() as u64; // Number of unique links between the two genomes

        for link_ref in other.links.keys() {
            if !self.links.contains_key(link_ref) {
                link_differences += 1;
            }
        }
        link_count += link_differences; // Count is number of links in A + links in B that are not in A

        for (link_ref, link) in self.links.iter() {
            if let Some(link2) = other.links.get(link_ref) {
                link_distance += link.distance(link2); // Distance normalized between 0 and 1
            } else {
                link_differences += 1;
            }
        }

        return ((link_differences as f64) + link_distance) / (link_count as f64);
    }

    /// DFS search to check for cycles.
    ///
    /// If 'from' is reachable from 'to', then addition will cause cycle
    fn creates_cycle(&self, from: NodeRef, to: NodeRef) -> bool {
        let mut visited: HashSet<NodeRef> = [to].iter().cloned().collect();
        let mut stack: Vec<NodeRef> = vec![to];

        while let Some(node) = stack.pop() {
            if node == from {
                return true;
            } else if let Some(vec) = self.connections.get(&node) {
                let l = stack.len();
                stack.extend(vec.iter().filter(|n| !visited.contains(n)));
                visited.extend(stack.iter().skip(l));
            }
        }

        return false;
    }

    /// Evaluate network, takes input node values, returns output node values
    pub fn evaluate(&self, inputs: &Vec<f64>) -> HashMap<u64, f64> {
        // Init value storage with input values
        let mut values: HashMap<NodeRef, f64> = inputs
            .iter()
            .enumerate()
            .map(|(i, value)| (NodeRef::Input(i as u64), *value))
            .collect();

        // Do forward pass
        for action in self.actions.iter() {
            match action {
                Action::Link(from, to) => {
                    if let Some(link) = self.links.get(&(*from, *to)) {
                        if link.enabled {
                            values.insert(
                                link.to,
                                values.get(&link.to).unwrap_or(&0.0)
                                    + values.get(&link.from).unwrap_or(&0.0) * link.weight,
                            );
                        }
                    }
                }
                Action::Activation(node_ref) => {
                    if let Some(activation) = self.get_activation(node_ref) {
                        values.insert(
                            *node_ref,
                            activation.activate(*values.get(node_ref).unwrap_or(&0.0)),
                        );
                    }
                }
            }
        }

        // Return values of output nodes
        return self
            .outputs
            .keys()
            .map(|node_ref| (node_ref.get_id(), *values.get(node_ref).unwrap_or(&0.0)))
            .collect();
    }

    fn get_activation(&self, node_ref: &NodeRef) -> Option<Activation> {
        match node_ref {
            NodeRef::Hidden(_) => self.hidden_nodes.get(node_ref)?.get_activation(),
            NodeRef::Output(_) => self.outputs.get(node_ref)?.get_activation(),
            _ => None,
        }
    }

    /// Determine order of nodes and links to actiave in forward pass
    fn sort_actions_topologically(&mut self) {
        let mut actions: Vec<Action> = Vec::new();
        let mut stack: Vec<NodeRef> = self.inputs.keys().cloned().collect();

        let mut hidden_nodes_included: HashMap<NodeRef, bool> =
            self.hidden_nodes.keys().map(|key| (*key, false)).collect();
        let mut output_nodes_included: HashMap<NodeRef, bool> =
            self.outputs.keys().map(|key| (*key, false)).collect();

        // Store number of incoming connections for all nodes
        let mut backward_count: HashMap<NodeRef, u64> = HashMap::new();
        for link in self.links.values() {
            if link.enabled {
                backward_count.insert(link.to, *backward_count.get(&link.to).unwrap_or(&0) + 1);
            }
        }

        while let Some(node) = stack.pop() {
            actions.push(Action::Activation(node));

            // Maintain addition status of hidden and output nodes
            match node {
                NodeRef::Hidden(_) => hidden_nodes_included.insert(node, true),
                NodeRef::Output(_) => output_nodes_included.insert(node, true),
                _ => None,
            };

            // Process all outgoin connections from the current node
            if let Some(vec) = self.connections.get(&node) {
                for to in vec.iter() {
                    actions.push(Action::Link(node, *to));

                    // Reduce backward count by 1
                    backward_count.insert(*to, *backward_count.get(to).unwrap_or(&0) - 1);

                    // Add nodes with no incoming connections to the stack
                    if *backward_count.get(to).unwrap_or(&0) == 0 {
                        stack.push(*to);
                    }
                }
            }
        }

        // To allow for insertion of new links without finding new topological sorting,
        // all nodes must be included, even though they are not currently connected.

        // Add non-connected hidden nodes
        actions.extend(
            hidden_nodes_included
                .iter()
                .filter(|(_, included)| !*included)
                .map(|(node_ref, _)| Action::Activation(*node_ref)),
        );

        // Add non-connected output nodes
        actions.extend(
            output_nodes_included
                .iter()
                .filter(|(_, included)| !*included)
                .map(|(node_ref, _)| Action::Activation(*node_ref)),
        );

        self.actions = actions;
    }
}
