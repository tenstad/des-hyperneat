use crate::neat::{
    conf::NEAT,
    genome::{Genome, GenomeComponent},
    link::LinkCore,
    node::{NodeCore, NodeRef},
    state::{InitConfig, Innovation, NeatStateProvider},
};
use network::connection;
use rand::{seq::SliceRandom, Rng};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct GenomeCore<N, L, S>
where
    N: GenomeComponent<NodeCore, S>,
    L: GenomeComponent<LinkCore, S>,
{
    pub inputs: HashMap<NodeRef, N>,
    pub hidden_nodes: HashMap<NodeRef, N>,
    pub outputs: HashMap<NodeRef, N>,
    pub links: HashMap<(NodeRef, NodeRef), L>, // Links between nodes

    pub connections: connection::Connections<NodeRef, ()>, // Fast connection lookup
    phantom: PhantomData<S>,
}

impl<N, L, S: NeatStateProvider> Genome for GenomeCore<N, L, S>
where
    N: GenomeComponent<NodeCore, S>,
    L: GenomeComponent<LinkCore, S>,
    S: NeatStateProvider,
{
    type Node = N;
    type Link = L;
    type Init = InitConfig;
    type State = S;

    fn get_neat(&self) -> &Self {
        self
    }

    fn get_neat_mut(&mut self) -> &mut Self {
        self
    }

    fn mutate(&mut self, population_state: &mut Self::State) {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < NEAT.add_node_probability {
            self.mutation_add_node(population_state);
        }

        if rng.gen::<f64>() < NEAT.add_connection_probability {
            self.mutation_add_connection(population_state);
        }

        if rng.gen::<f64>() < NEAT.disable_connection_probability {
            self.mutation_disable_connection();
        }

        if rng.gen::<f64>() < NEAT.mutate_link_weight_probability {
            self.mutate_link_weight();
        }
    }

    // Genetic distance between two genomes
    fn distance(&self, other: &Self) -> f64 {
        let mut link_differences: f64 = 0.0; // Number of links present in only one of the genomes
        let mut link_distance: f64 = 0.0; // Total distance between links present in both genomes
        let mut link_count = self.links.len() as f64; // Number of unique links between the two genomes

        for link_ref in other.links.keys() {
            if !self.links.contains_key(link_ref) {
                link_differences += 1.0;
            }
        }
        link_count += link_differences; // Count is number of links in A + links in B that are not in A

        for (link_ref, link) in self.links.iter() {
            if let Some(link2) = other.links.get(link_ref) {
                link_distance += link.distance(link2); // Distance normalized between 0 and 1
            } else {
                link_differences += 1.0;
            }
        }

        let link_dist = if link_count == 0.0 {
            0.0
        } else {
            (link_differences + link_distance) / link_count
        };

        // Same process for nodes
        let mut node_differences = 0.0;
        let mut node_distance = 0.0;
        let mut node_count = self.hidden_nodes.len() as f64;

        if !NEAT.only_hidden_node_distance {
            node_count += (self.inputs.len() + self.outputs.len()) as f64;
        }
        for node_ref in other.hidden_nodes.keys() {
            if !self.hidden_nodes.contains_key(node_ref) {
                node_differences += 1.0;
            }
        }
        if !NEAT.only_hidden_node_distance {
            for node_ref in other.inputs.keys() {
                if !self.inputs.contains_key(node_ref) {
                    node_differences += 1.0;
                }
            }
            for node_ref in other.outputs.keys() {
                if !self.outputs.contains_key(node_ref) {
                    node_differences += 1.0;
                }
            }
        }
        node_count += node_differences;

        for (node_ref, node) in self.hidden_nodes.iter() {
            if let Some(node2) = other.hidden_nodes.get(node_ref) {
                node_distance += node.distance(node2);
            } else {
                node_differences += 1.0;
            }
        }
        if !NEAT.only_hidden_node_distance {
            for (node_ref, node) in self.inputs.iter() {
                if let Some(node2) = other.inputs.get(node_ref) {
                    node_distance += node.distance(node2);
                } else {
                    node_differences += 1.0;
                }
            }
            for (node_ref, node) in self.outputs.iter() {
                if let Some(node2) = other.outputs.get(node_ref) {
                    node_distance += node.distance(node2);
                } else {
                    node_differences += 1.0;
                }
            }
        }

        let node_dist = if node_count == 0.0 {
            0.0
        } else {
            (node_differences + node_distance) / node_count
        };

        NEAT.link_distance_weight * link_dist + (1.0 - NEAT.link_distance_weight) * node_dist
    }

    /// Generate genome with default activation and no connections
    fn new(init_config: &InitConfig, population_state: &mut Self::State) -> Self {
        let inputs: HashMap<NodeRef, N> = (0..init_config.inputs)
            .map(|i| {
                (
                    NodeRef::Input(i),
                    N::new(NodeCore::new(NodeRef::Input(i)), population_state),
                )
            })
            .collect();

        let outputs: HashMap<NodeRef, N> = (0..init_config.outputs)
            .map(|i| {
                (
                    NodeRef::Output(i),
                    N::new(NodeCore::new(NodeRef::Output(i)), population_state),
                )
            })
            .collect();

        Self {
            inputs,
            outputs,
            hidden_nodes: HashMap::new(),
            links: HashMap::new(),
            connections: connection::Connections::<NodeRef, ()>::new(),
            phantom: PhantomData,
        }
    }

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        // Let parent1 be the fitter parent
        let (parent1, parent2) = if fitness > other_fitness {
            (self, other)
        } else {
            (other, self)
        };

        let mut genome = Self::empty();

        // Copy links only in fitter parent, perform crossover if in both parents
        for (link_ref, link) in parent1.links.iter() {
            if !genome
                .connections
                .creates_cycle(link.get_neat().from, link.get_neat().to)
            {
                if let Some(link2) = parent2.links.get(link_ref) {
                    genome.insert_link(link.crossover(link2, fitness, other_fitness));
                } else {
                    genome.insert_link(link.clone());
                }
            }
        }

        // Copy nodes only in fitter parent, perform crossover if in both parents
        for (node_ref, node) in parent1.inputs.iter() {
            if let Some(node2) = parent2.inputs.get(node_ref) {
                genome
                    .inputs
                    .insert(*node_ref, node.crossover(node2, fitness, other_fitness));
            } else {
                genome.inputs.insert(*node_ref, node.clone());
            }
        }

        for (node_ref, node) in parent1.hidden_nodes.iter() {
            if let Some(node2) = parent2.hidden_nodes.get(node_ref) {
                genome
                    .hidden_nodes
                    .insert(*node_ref, node.crossover(node2, fitness, other_fitness));
            } else {
                genome.hidden_nodes.insert(*node_ref, node.clone());
            }
        }

        for (node_ref, node) in parent1.outputs.iter() {
            if let Some(node2) = parent2.outputs.get(node_ref) {
                genome
                    .outputs
                    .insert(*node_ref, node.crossover(node2, fitness, other_fitness));
            } else {
                genome.outputs.insert(*node_ref, node.clone());
            }
        }

        return genome;
    }
}

impl<N: GenomeComponent<NodeCore, S>, L: GenomeComponent<LinkCore, S>, S: NeatStateProvider>
    GenomeCore<N, L, S>
{
    pub fn empty() -> Self {
        Self {
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            hidden_nodes: HashMap::new(),
            links: HashMap::new(),
            connections: connection::Connections::<NodeRef, ()>::new(),
            phantom: PhantomData,
        }
    }

    pub fn get_node<'a>(&'a self, node_ref: &'a NodeRef) -> Option<&'a N> {
        match node_ref {
            NodeRef::Input(_) => self.inputs.get(node_ref),
            NodeRef::Hidden(_) => self.hidden_nodes.get(node_ref),
            NodeRef::Output(_) => self.outputs.get(node_ref),
        }
    }

    pub fn split_link(
        &mut self,
        from: NodeRef,
        to: NodeRef,
        new_node_id: usize,
        innovation_number: usize,
        population_state: &mut S,
    ) {
        // Disable link
        let link = self
            .links
            .get_mut(&(from, to))
            .expect("unable to split nonexistent link")
            .get_neat_mut();

        assert!(link.enabled);
        link.enabled = false;
        link.split = true;

        let new_node_ref = NodeRef::Hidden(new_node_id);

        // Might have inherited that the connection is not split, but also the nodes splitting it
        if self.hidden_nodes.contains_key(&new_node_ref) {
            return;
        }

        // Remove connection
        self.connections.remove(&from, to);

        self.hidden_nodes.insert(
            new_node_ref,
            N::new(
                NodeCore::new(NodeRef::Hidden(new_node_id)),
                population_state,
            ),
        );

        let link1 = L::new(
            LinkCore::new(from, new_node_ref, 1.0, innovation_number),
            population_state,
        );
        let link2 = L::new(
            LinkCore::new(new_node_ref, to, link.weight, innovation_number + 1),
            population_state,
        );

        assert!(!self
            .links
            .contains_key(&(link1.get_neat().from, link1.get_neat().to)));
        self.insert_link(link1);

        assert!(!self
            .links
            .contains_key(&(link2.get_neat().from, link2.get_neat().to)));
        self.insert_link(link2);
    }

    pub fn insert_link(&mut self, link: L) {
        // Add link
        self.links
            .insert((link.get_neat().from, link.get_neat().to), link.clone());

        // Add connections
        self.connections
            .add(link.get_neat().from, link.get_neat().to, ());
    }

    fn mutate_link_weight(&mut self) {
        let mut rng = rand::thread_rng();

        // Mutate single link
        if !self.links.is_empty() {
            let link_index = rng.gen_range(0, self.links.len());
            if let Some(link) = self.links.values_mut().skip(link_index).next() {
                link.get_neat_mut().weight +=
                    (rng.gen::<f64>() - 0.5) * 2.0 * NEAT.mutate_link_weight_size;
            }
        }

        /*for link in self.links.values_mut() {
            link.weight += (rng.gen::<f64>() - 0.5) * 2.0 * NEAT.mutate_link_weight_size;
        }*/
    }

    fn mutation_add_node(&mut self, population_state: &mut S) {
        // Select random enabled link
        if let Some(index) = self
            .links
            .iter()
            .filter(|(_, link)| !link.get_neat().split && link.get_neat().enabled)
            .map(|(i, _)| *i)
            .collect::<Vec<(NodeRef, NodeRef)>>()
            .choose(&mut rand::thread_rng())
        {
            let link = self.links.get(index).unwrap().clone();
            let from = link.get_neat().from;
            let to = link.get_neat().to;

            // Check if this link has been split by another individual
            if let Some(addition) = population_state
                .get_neat()
                .innovation_log
                .node_additions
                .get(&link.get_neat().innovation)
            {
                // Split the link
                self.split_link(
                    from,
                    to,
                    addition.node_number,
                    addition.innovation_number,
                    population_state,
                );
            } else {
                let (node_number, innovation_number) = {
                    let neat_state = population_state.get_neat();
                    (
                        neat_state.next_innovation.node_number,
                        neat_state.next_innovation.innovation_number,
                    )
                };

                // Split the link
                self.split_link(from, to, node_number, innovation_number, population_state);

                let mut neat_state = population_state.get_neat_mut();

                // Add this mutation to log
                neat_state.innovation_log.node_additions.insert(
                    link.get_neat().innovation,
                    Innovation {
                        node_number: neat_state.next_innovation.node_number,
                        innovation_number: neat_state.next_innovation.innovation_number,
                    },
                );

                // Increase global node count and innovation number
                neat_state.next_innovation.node_number += 1;
                neat_state.next_innovation.innovation_number += 2;
            }
        }
    }

    // TODO: avoid retries
    fn mutation_add_connection(&mut self, population_state: &mut S) {
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
                    let innovation = match population_state
                        .get_neat()
                        .innovation_log
                        .edge_additions
                        .get(&(from, to))
                    {
                        Some(innovation_number) => *innovation_number,
                        None => {
                            let mut neat_state = population_state.get_neat_mut();
                            neat_state
                                .innovation_log
                                .edge_additions
                                .insert((from, to), neat_state.next_innovation.innovation_number);
                            neat_state.next_innovation.innovation_number += 1;

                            neat_state.next_innovation.innovation_number - 1
                        }
                    };

                    self.insert_link(L::new(
                        LinkCore::new(
                            from,
                            to,
                            (rng.gen::<f64>() - 0.5) * 2.0 * NEAT.initial_link_weight_size,
                            innovation,
                        ),
                        population_state,
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
            .filter_map(|(i, link)| {
                if link.get_neat().enabled {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<&(NodeRef, NodeRef)>>()
            .choose(&mut rand::thread_rng())
        {
            let connection_ref = *connection_ref;

            // Do not remove connection as it is needed to check for cycles and might be enabled again
            // self.connections.remove(&connection_ref.0, connection_ref.1);
            self.links
                .get_mut(&connection_ref)
                .unwrap()
                .get_neat_mut()
                .enabled = false;
        }
    }
}
