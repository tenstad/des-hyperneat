use crate::conf;
use crate::generic_neat::innovation::InnovationLog;
use crate::generic_neat::innovation::InnovationTime;
use crate::generic_neat::link::Link;
use crate::generic_neat::node::Node;
use crate::generic_neat::node::NodeRef;
use crate::network::activation;
use crate::network::connection;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Genome {
    pub inputs: HashMap<NodeRef, Node>,
    pub hidden_nodes: HashMap<NodeRef, Node>,
    pub outputs: HashMap<NodeRef, Node>,
    pub links: HashMap<(NodeRef, NodeRef), Link>, // Links between nodes

    pub connections: connection::Connections<NodeRef, ()>, // Fast connection lookup
}

impl Genome {
    pub fn empty() -> Self {
        Self {
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            hidden_nodes: HashMap::new(),
            links: HashMap::new(),
            connections: connection::Connections::<NodeRef, ()>::new(),
        }
    }

    /// Generate genome with default activation and no connections
    pub fn new(inputs: usize, outputs: usize) -> Self {
        let inputs: HashMap<NodeRef, Node> = (0..inputs as u64)
            .map(|i| (NodeRef::Input(i), Node::new(NodeRef::Input(i))))
            .collect();

        let outputs: HashMap<NodeRef, Node> = (0..outputs as u64)
            .map(|i| (NodeRef::Output(i), Node::new(NodeRef::Output(i))))
            .collect();

        Self {
            inputs,
            outputs,
            hidden_nodes: HashMap::new(),
            links: HashMap::new(),
            connections: connection::Connections::<NodeRef, ()>::new(),
        }
    }

    pub fn split_link(&mut self, link: Link, new_node_id: u64, innovation_number: u64) {
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

        // Remove connection
        self.connections.remove(&link.from, link.to);

        self.hidden_nodes
            .insert(new_node_ref, Node::new(NodeRef::Hidden(new_node_id)));

        let link1 = Link::new(link.from, new_node_ref, 1.0, innovation_number);
        let link2 = Link::new(new_node_ref, link.to, link.weight, innovation_number + 1);

        assert!(!self.links.contains_key(&(link1.from, link1.to)));
        self.insert_link(link1);

        assert!(!self.links.contains_key(&(link2.from, link2.to)));
        self.insert_link(link2);
    }

    pub fn insert_link(&mut self, link: Link) {
        // Add link
        self.links.insert((link.from, link.to), link);

        // Add connections
        self.connections.add(link.from, link.to, ());
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
                    genome.insert_link(link.crossover(link2));
                } else {
                    genome.insert_link(*link);
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

                    self.insert_link(Link::new(
                        from,
                        to,
                        (rng.gen::<f64>() - 0.5) * 2.0 * conf::NEAT.initial_link_weight_size,
                        innovation,
                    ));
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
            .filter_map(|(i, link)| if link.enabled { Some(i) } else { None })
            .collect::<Vec<&(NodeRef, NodeRef)>>()
            .choose(&mut rand::thread_rng())
        {
            let connection_ref = *connection_ref;

            // Do not remove connection as it is needed to check for cycles and might be enabled again
            // self.connections.remove(&connection_ref.0, connection_ref.1);
            self.links.get_mut(&connection_ref).unwrap().enabled = false;
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

    pub fn get_activation(&self, node_ref: &NodeRef) -> activation::Activation {
        match node_ref {
            NodeRef::Input(_) => self.inputs.get(node_ref).unwrap().activation,
            NodeRef::Hidden(_) => self.hidden_nodes.get(node_ref).unwrap().activation,
            NodeRef::Output(_) => self.outputs.get(node_ref).unwrap().activation,
        }
    }

    pub fn get_bias(&self, node_ref: &NodeRef) -> f64 {
        match node_ref {
            NodeRef::Input(_) => self.inputs.get(node_ref).unwrap().bias,
            NodeRef::Hidden(_) => self.hidden_nodes.get(node_ref).unwrap().bias,
            NodeRef::Output(_) => self.outputs.get(node_ref).unwrap().bias,
        }
    }
}
