use crate::conf;
use crate::neat::actions::Action;
use crate::neat::actions::Actions;
use crate::neat::connections::Connections;
use crate::neat::evaluator;
use crate::neat::nodes::*;
use crate::neat::population::InnovationLog;
use crate::neat::population::InnovationTime;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Genome {
    pub inputs: HashMap<NodeRef, InputNode>,
    pub outputs: HashMap<NodeRef, OutputNode>,
    pub hidden_nodes: HashMap<NodeRef, HiddenNode>,
    pub links: HashMap<(NodeRef, NodeRef), Link>, // Links between nodes

    actions: Actions<NodeRef>,         // Actions to perform when evaluating
    connections: Connections<NodeRef>, // Fast connection lookup
}

/// Link between two nodes
#[derive(Copy, Clone, Debug)]
pub struct Link {
    pub from: NodeRef,
    pub to: NodeRef,
    pub weight: f64,
    pub enabled: bool,
    pub split: bool,     // Link has been split
    pub innovation: u64, // Global innovation number
}

impl Link {
    fn crossover(&self, other: &Link) -> Link {
        assert_eq!(self.from, other.from);
        assert_eq!(self.to, other.to);
        assert_eq!(self.innovation, other.innovation);

        Link {
            from: self.from,
            to: self.to,
            weight: (self.weight + other.weight) / 2.0,
            enabled: self.enabled || other.enabled,
            split: self.split && other.split,
            innovation: self.innovation,
        }
    }

    fn distance(&self, other: &Link) -> f64 {
        0.5 * (self.weight - other.weight).tanh().abs()
            + 0.5 * ((self.enabled == other.enabled) as u64) as f64
    }
}

impl Genome {
    pub fn empty() -> Genome {
        Genome {
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            hidden_nodes: HashMap::new(),
            links: HashMap::new(),
            actions: Actions::<NodeRef>::empty(),
            connections: Connections::<NodeRef>::new(),
        }
    }

    /// Generate genome with default activation and no connections
    pub fn new(inputs: u64, outputs: u64) -> Genome {
        let inputs: HashMap<NodeRef, InputNode> = (0..inputs)
            .map(|i| (NodeRef::Input(i), InputNode::new(i)))
            .collect();

        let outputs: HashMap<NodeRef, OutputNode> = (0..outputs)
            .map(|i| (NodeRef::Output(i), OutputNode::new(i)))
            .collect();

        let actions = Actions::<NodeRef>::from_nodes(
            inputs.keys().cloned().collect(),
            vec![],
            outputs.keys().cloned().collect(),
        );

        Genome {
            inputs,
            outputs,
            hidden_nodes: HashMap::new(),
            links: HashMap::new(),
            actions,
            connections: Connections::<NodeRef>::new(),
        }
    }

    fn split_link(&mut self, link: Link, new_node_id: u64, innovation_number: u64) {
        {
            // Disable link
            let link = self
                .links
                .get_mut(&(link.from, link.to))
                .expect("Unable to split nonexistent link");

            assert!(link.enabled);
            link.enabled = false;
            link.split = true;
        }

        let new_node_ref = NodeRef::Hidden(new_node_id);

        // Might have inherited that the connection is not split, but also the nodes splitting it
        if self.hidden_nodes.contains_key(&new_node_ref) {
            return;
        }

        // Disable connection
        self.connections.disable(link.from, link.to);

        // Add and remvoe actions
        self.actions.split_link(link.from, link.to, new_node_ref);

        self.hidden_nodes
            .insert(new_node_ref, HiddenNode::new(new_node_id));

        let link1 = Link {
            from: link.from,
            to: new_node_ref,
            weight: 1.0,
            enabled: true,
            split: false,
            innovation: innovation_number,
        };

        let link2 = Link {
            from: new_node_ref,
            to: link.to,
            weight: link.weight,
            enabled: true,
            split: false,
            innovation: innovation_number + 1,
        };

        assert!(!self.links.contains_key(&(link1.from, link1.to)));
        self.insert_link(link1, false);

        assert!(!self.links.contains_key(&(link2.from, link2.to)));
        self.insert_link(link2, false);
    }

    fn insert_link(&mut self, link: Link, add_action: bool) {
        // Add link
        self.links.insert((link.from, link.to), link);

        // Add connections
        self.connections.add(link.from, link.to, link.enabled);

        // Add action
        if link.enabled && add_action {
            // When adding many links at the same time, it is faster to sort
            // topologically at the end than adding every connection independently
            // When 'add_action' is false, 'sort_topologically' must be called on
            // self.actions when all links are inserted.
            // Except when the link is added by split, then self.action should
            // perform the split internally.
            self.actions.add_link(link.from, link.to, &self.connections);
        }
    }

    pub fn crossover(&self, other: &Self, is_fitter: bool) -> Genome {
        // Let parent1 be the fitter parent
        let (parent1, parent2) = if is_fitter {
            (self, other)
        } else {
            (other, self)
        };

        let mut genome = Genome::empty();

        // Copy links only in fitter parent, perform crossover if in both parents
        for (link_ref, link) in parent1.links.iter() {
            if !genome.connections.creates_cycle(link.from, link.to) {
                if let Some(link2) = parent2.links.get(link_ref) {
                    genome.insert_link(link.crossover(link2), false);
                } else {
                    genome.insert_link(*link, false);
                }
            }
        }

        // Copy nodes only in fitter parent, perform crossover if in both parents
        for (node_ref, node) in parent1.inputs.iter() {
            if let Some(node2) = parent2.inputs.get(node_ref) {
                genome.inputs.insert(*node_ref, node.crossover(node2));
            } else {
                genome.inputs.insert(*node_ref, *node);
            }
            genome.actions.add_input(*node_ref);
        }

        for (node_ref, node) in parent1.hidden_nodes.iter() {
            if let Some(node2) = parent2.hidden_nodes.get(node_ref) {
                genome.hidden_nodes.insert(*node_ref, node.crossover(node2));
            } else {
                genome.hidden_nodes.insert(*node_ref, *node);
            }
            genome.actions.add_hidden(*node_ref);
        }

        for (node_ref, node) in parent1.outputs.iter() {
            if let Some(node2) = parent2.outputs.get(node_ref) {
                genome.outputs.insert(*node_ref, node.crossover(node2));
            } else {
                genome.outputs.insert(*node_ref, *node);
            }
            genome.actions.add_output(*node_ref);
        }

        // Topologically sort actions of child, as this is not done when inserting links and nodes
        genome.actions.sort_topologically(&genome.connections);

        return genome;
    }

    pub fn mutate(&mut self, log: &mut InnovationLog, global_innovation: &mut InnovationTime) {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < conf::NEAT.add_node_probability {
            self.mutation_add_node(log, global_innovation);
        }

        if rng.gen::<f64>() < conf::NEAT.add_connection_probability {
            self.mutation_add_connection(log, global_innovation);
        }

        if rng.gen::<f64>() < conf::NEAT.disable_connection_probability {
            self.mutation_disable_connection();
        }

        if rng.gen::<f64>() < conf::NEAT.mutate_link_weight_probability {
            self.mutate_link_weight();
        }

        if rng.gen::<f64>() < conf::NEAT.mutate_hidden_bias_probability {
            self.mutate_hidden_bias();
        }

        if rng.gen::<f64>() < conf::NEAT.mutate_hidden_activation_probability {
            self.mutate_hidden_activation();
        }

        if rng.gen::<f64>() < conf::NEAT.mutate_output_bias_probability {
            self.mutate_output_bias();
        }

        if rng.gen::<f64>() < conf::NEAT.mutate_output_activation_probability {
            self.mutate_output_activation();
        }
    }

    fn mutate_link_weight(&mut self) {
        let mut rng = rand::thread_rng();

        // Mutate single link
        /*if !self.links.is_empty() {
            let link_index = rng.gen_range(0, self.links.len());
            if let Some(link) = self.links.values_mut().skip(link_index).next() {
                link.weight += (rng.gen::<f64>() - 0.5) * 2.0 * conf::NEAT.mutate_link_weight_size;
            }
        }*/

        for link in self.links.values_mut() {
            link.weight += (rng.gen::<f64>() - 0.5) * 2.0 * conf::NEAT.mutate_link_weight_size;
        }
    }

    fn mutate_hidden_bias(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.hidden_nodes.is_empty() {
            let link_index = rng.gen_range(0, self.hidden_nodes.len());
            if let Some(node) = self.hidden_nodes.values_mut().skip(link_index).next() {
                node.bias += (rng.gen::<f64>() - 0.5) * 2.0 * conf::NEAT.mutate_hidden_bias_size;
            }
        }
    }

    fn mutate_hidden_activation(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.hidden_nodes.is_empty() {
            let link_index = rng.gen_range(0, self.hidden_nodes.len());
            if let Some(node) = self.hidden_nodes.values_mut().skip(link_index).next() {
                node.activation = conf::NEAT.hidden_activations.random();
            }
        }
    }

    fn mutate_output_bias(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.outputs.is_empty() {
            let link_index = rng.gen_range(0, self.outputs.len());
            if let Some(node) = self.outputs.values_mut().skip(link_index).next() {
                node.bias += (rng.gen::<f64>() - 0.5) * 2.0 * conf::NEAT.mutate_output_bias_size;
            }
        }
    }

    fn mutate_output_activation(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.outputs.is_empty() {
            let link_index = rng.gen_range(0, self.outputs.len());
            if let Some(node) = self.outputs.values_mut().skip(link_index).next() {
                node.activation = conf::NEAT.output_activations.random();
            }
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
            .filter(|(_, link)| !link.split && link.enabled)
            .map(|(i, _)| *i)
            .collect::<Vec<(NodeRef, NodeRef)>>()
            .choose(&mut rand::thread_rng())
        {
            assert!(self.actions.contains(&Action::Link(index.0, index.1)));

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
                if !self.links.contains_key(&(from, to))
                    && !self.connections.creates_cycle(from, to)
                {
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
                            weight: rng.gen::<f64>() - 0.5,
                            enabled: true,
                            split: false,
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

    fn mutation_disable_connection(&mut self) {
        if let Some(&connection_ref) = self
            .links
            .iter()
            .filter(|(_, link)| link.enabled)
            .map(|(i, _)| i)
            .collect::<Vec<&(NodeRef, NodeRef)>>()
            .choose(&mut rand::thread_rng())
        {
            let connection_ref = *connection_ref;

            self.connections.disable(connection_ref.0, connection_ref.1);
            self.actions.remove_link(connection_ref.0, connection_ref.1);

            if let Some(link) = self.links.get_mut(&connection_ref) {
                link.enabled = false;
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

        return if link_count == 0 {
            0.0
        } else {
            ((link_differences as f64) + link_distance) / (link_count as f64)
        };
    }

    /// Creates speed-optimized evaluator
    pub fn create_evaluator(&self) -> evaluator::Evaluator {
        let length_inputs = self.inputs.len();
        let length_hiddens = length_inputs + self.hidden_nodes.len();
        let length_outputs = length_hiddens + self.outputs.len();

        let mut input_keys: Vec<NodeRef> = self.inputs.keys().cloned().collect();
        input_keys.sort();
        let mut output_keys: Vec<NodeRef> = self.outputs.keys().cloned().collect();
        output_keys.sort();

        let node_mapper: HashMap<NodeRef, usize> = input_keys
            .iter()
            .enumerate()
            .map(|(i, node_ref)| (*node_ref, i))
            .chain(
                self.hidden_nodes
                    .keys()
                    .enumerate()
                    .map(|(i, node_ref)| (*node_ref, i + length_inputs)),
            )
            .chain(
                output_keys
                    .iter()
                    .enumerate()
                    .map(|(i, node_ref)| (*node_ref, i + length_hiddens)),
            )
            .collect();

        let actions = self
            .actions
            .iter()
            .map(|action| match action {
                Action::Link(from, to) => evaluator::Action::Link(
                    *node_mapper.get(from).unwrap(),
                    *node_mapper.get(to).unwrap(),
                    self.links.get(&(*from, *to)).unwrap().weight,
                ),
                Action::Activation(node) => evaluator::Action::Activation(
                    *node_mapper.get(node).unwrap(),
                    self.get_bias(node).unwrap(),
                    self.get_activation(node).unwrap(),
                ),
            })
            .collect();

        evaluator::Evaluator::create(
            length_outputs,
            input_keys
                .iter()
                .map(|node| node.get_id() as usize)
                .collect(),
            output_keys
                .iter()
                .map(|node| node.get_id() as usize + length_hiddens)
                .collect(),
            actions,
        )
    }

    fn get_activation(&self, node_ref: &NodeRef) -> Option<Activation> {
        match node_ref {
            NodeRef::Hidden(_) => self.hidden_nodes.get(node_ref)?.get_activation(),
            NodeRef::Output(_) => self.outputs.get(node_ref)?.get_activation(),
            _ => Some(Activation::None),
        }
    }

    fn get_bias(&self, node_ref: &NodeRef) -> Option<f64> {
        match node_ref {
            NodeRef::Hidden(_) => self.hidden_nodes.get(node_ref)?.get_bias(),
            NodeRef::Output(_) => self.outputs.get(node_ref)?.get_bias(),
            _ => Some(0.0),
        }
    }
}
