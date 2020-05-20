use crate::genome::{GenericGenome, Genome as EvolvableGenome};
use crate::neat::{
    conf::ConfigProvider,
    conf::NeatConfig,
    link::{LinkExtension, NeatLink},
    node::{NeatNode, NodeExtension, NodeRef},
    state::{InitConfig, NeatState, StateProvider},
};
use crate::stats::Stats;
use network::connection;
use rand::{seq::SliceRandom, Rng};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone)]
pub struct NeatGenome<N, L> {
    pub inputs: HashMap<NodeRef, N>,
    pub hidden_nodes: HashMap<NodeRef, N>,
    pub outputs: HashMap<NodeRef, N>,
    pub links: HashMap<(NodeRef, NodeRef), L>, // Links between nodes

    pub connections: connection::Connections<NodeRef, ()>, // Fast connection lookup
}

#[derive(Serialize)]
pub struct NeatGenomeStats {
    #[serde(with = "bson::compat::u2f")]
    pub hidden_nodes: u64,
    #[serde(with = "bson::compat::u2f")]
    pub links: u64,
}
impl Stats for NeatGenomeStats {}

pub trait GetNeat<T> {
    fn neat(&self) -> &T;
    fn neat_mut(&mut self) -> &mut T;
}

pub type DefaultNeatGenome = NeatGenome<NeatNode, NeatLink>;
impl EvolvableGenome for DefaultNeatGenome {
    type InitConfig = InitConfig;
    type State = NeatState;
    type Config = NeatConfig;
    type Stats = NeatGenomeStats;
}

impl<N, L, C, S> GenericGenome<C, S, InitConfig, NeatGenomeStats> for NeatGenome<N, L>
where
    N: NodeExtension,
    L: LinkExtension,
    C: ConfigProvider<N::Config, L::Config>,
    S: StateProvider<N::State, L::State>,
{
    fn mutate(&mut self, config: &C, state: &mut S) {
        let neat_config = config.neat();
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < neat_config.add_node_probability {
            self.mutation_add_node(config, state);
        }

        if rng.gen::<f64>() < neat_config.add_link_probability {
            self.mutation_add_link(config, state);
        }

        if rng.gen::<f64>() < neat_config.remove_link_probability {
            self.mutation_remove_link();
        }

        if rng.gen::<f64>() < neat_config.remove_node_probability {
            self.mutation_remove_node();
        }

        if rng.gen::<f64>() < neat_config.mutate_link_weight_probability {
            self.mutate_link_weight(config);
        }
    }

    /// Generate genome with default activation and no connections
    fn new(config: &C, init_config: &InitConfig, state: &mut S) -> Self {
        let node_config = config.neat_node();

        let inputs: HashMap<NodeRef, N> = (0..init_config.inputs)
            .map(|i| {
                (
                    NodeRef::Input(i),
                    N::new(
                        node_config,
                        NeatNode::new(NodeRef::Input(i)),
                        state.node_mut(),
                    ),
                )
            })
            .collect();

        let outputs: HashMap<NodeRef, N> = (0..init_config.outputs)
            .map(|i| {
                (
                    NodeRef::Output(i),
                    N::new(
                        node_config,
                        NeatNode::new(NodeRef::Output(i)),
                        state.node_mut(),
                    ),
                )
            })
            .collect();

        Self {
            inputs,
            outputs,
            hidden_nodes: HashMap::new(),
            links: HashMap::new(),
            connections: connection::Connections::<NodeRef, ()>::new(),
        }
    }

    fn distance(&self, config: &C, other: &Self) -> f64 {
        Self::distance(self, config, other)
    }

    fn crossover(&self, config: &C, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        Self::crossover(self, config, other, fitness, other_fitness)
    }

    fn get_stats(&self) -> NeatGenomeStats {
        Self::get_stats(self)
    }
}

impl<N, L> NeatGenome<N, L>
where
    N: NodeExtension,
    L: LinkExtension,
{
    // Genetic distance between two genomes
    pub fn distance<C: ConfigProvider<N::Config, L::Config>>(
        &self,
        config: &C,
        other: &Self,
    ) -> f64 {
        let neat_config = config.neat();
        let node_config = config.neat_node();
        let link_config = config.neat_link();

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
                link_distance += link.distance(link_config, link2); // Distance normalized between 0 and 1
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

        if !neat_config.only_hidden_node_distance {
            node_count += (self.inputs.len() + self.outputs.len()) as f64;
        }
        for node_ref in other.hidden_nodes.keys() {
            if !self.hidden_nodes.contains_key(node_ref) {
                node_differences += 1.0;
            }
        }
        if !neat_config.only_hidden_node_distance {
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
                node_distance += node.distance(node_config, node2);
            } else {
                node_differences += 1.0;
            }
        }
        if !neat_config.only_hidden_node_distance {
            for (node_ref, node) in self.inputs.iter() {
                if let Some(node2) = other.inputs.get(node_ref) {
                    node_distance += node.distance(node_config, node2);
                } else {
                    node_differences += 1.0;
                }
            }
            for (node_ref, node) in self.outputs.iter() {
                if let Some(node2) = other.outputs.get(node_ref) {
                    node_distance += node.distance(node_config, node2);
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

        neat_config.link_distance_weight * link_dist
            + (1.0 - neat_config.link_distance_weight) * node_dist
    }

    pub fn crossover<C: ConfigProvider<N::Config, L::Config>>(
        &self,
        config: &C,
        other: &Self,
        fitness: &f64,
        other_fitness: &f64,
    ) -> Self {
        let node_config = config.neat_node();
        let link_config = config.neat_link();

        // Let parent1 be the fitter parent
        let (parent1, parent2) = if fitness > other_fitness {
            (self, other)
        } else {
            (other, self)
        };

        let mut genome = Self::empty();

        // Copy links only in fitter parent, perform crossover if in both parents
        for (link_ref, link) in parent1.links.iter() {
            /*if !genome
                .connections
                .creates_cycle(link.neat().from, link.neat().to)
            {*/
            if let Some(link2) = parent2.links.get(link_ref) {
                genome.insert_link(link.crossover(link_config, link2, fitness, other_fitness));
            } else {
                genome.insert_link(link.clone());
            }
            //}
        }

        // Copy nodes only in fitter parent, perform crossover if in both parents
        for (node_ref, node) in parent1.inputs.iter() {
            if let Some(node2) = parent2.inputs.get(node_ref) {
                genome.inputs.insert(
                    *node_ref,
                    node.crossover(node_config, node2, fitness, other_fitness),
                );
            } else {
                genome.inputs.insert(*node_ref, node.clone());
            }
        }

        for (node_ref, node) in parent1.hidden_nodes.iter() {
            if let Some(node2) = parent2.hidden_nodes.get(node_ref) {
                genome.hidden_nodes.insert(
                    *node_ref,
                    node.crossover(node_config, node2, fitness, other_fitness),
                );
            } else {
                genome.hidden_nodes.insert(*node_ref, node.clone());
            }
        }

        for (node_ref, node) in parent1.outputs.iter() {
            if let Some(node2) = parent2.outputs.get(node_ref) {
                genome.outputs.insert(
                    *node_ref,
                    node.crossover(node_config, node2, fitness, other_fitness),
                );
            } else {
                genome.outputs.insert(*node_ref, node.clone());
            }
        }

        return genome;
    }

    pub fn get_stats(&self) -> NeatGenomeStats {
        NeatGenomeStats {
            hidden_nodes: self.hidden_nodes.len() as u64,
            links: self.links.len() as u64,
        }
    }

    pub fn empty() -> Self {
        Self {
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            hidden_nodes: HashMap::new(),
            links: HashMap::new(),
            connections: connection::Connections::<NodeRef, ()>::new(),
        }
    }

    pub fn get_node(&self, node_ref: &NodeRef) -> Option<&N> {
        match node_ref {
            &NodeRef::Input(_) => self.inputs.get(node_ref),
            &NodeRef::Hidden(_) => self.hidden_nodes.get(node_ref),
            &NodeRef::Output(_) => self.outputs.get(node_ref),
        }
    }

    pub fn get_node_mut(&mut self, node_ref: &NodeRef) -> Option<&mut N> {
        match node_ref {
            &NodeRef::Input(_) => self.inputs.get_mut(node_ref),
            &NodeRef::Hidden(_) => self.hidden_nodes.get_mut(node_ref),
            &NodeRef::Output(_) => self.outputs.get_mut(node_ref),
        }
    }

    pub fn split_link<
        C: ConfigProvider<N::Config, L::Config>,
        S: StateProvider<N::State, L::State>,
    >(
        &mut self,
        config: &C,
        from: NodeRef,
        to: NodeRef,
        new_node_id: u64,
        innovation_number: u64,
        state: &mut S,
    ) {
        let link = self
            .links
            .get_mut(&(from, to))
            .expect("unable to split nonexistent link")
            .clone();

        // Remove old link and connection
        self.links.remove(&(from, to));
        self.connections.remove(&from, to);

        let new_node_ref = NodeRef::Hidden(new_node_id);

        self.hidden_nodes.insert(
            new_node_ref,
            N::new(
                config.neat_node(),
                NeatNode::new(NodeRef::Hidden(new_node_id)),
                state.node_mut(),
            ),
        );

        let (link1_details, link2_details) = if let NodeRef::Input(_) = from {
            (
                (new_node_ref, to, innovation_number + 1),
                (from, new_node_ref, innovation_number),
            )
        } else {
            (
                (from, new_node_ref, innovation_number),
                (new_node_ref, to, innovation_number + 1),
            )
        };
        let link1 = L::identity(
            config.neat_link(),
            NeatLink::new(link1_details.0, link1_details.1, 1.0, link1_details.2),
            state.link_mut(),
        );
        let link2 = link.clone_with(
            config.neat_link(),
            NeatLink::new(
                link2_details.0,
                link2_details.1,
                link.neat().weight,
                link2_details.2,
            ),
            state.link_mut(),
        );

        self.insert_link(link1);
        self.insert_link(link2);
    }

    pub fn insert_link(&mut self, link: L) {
        // Add link
        self.links
            .insert((link.neat().from, link.neat().to), link.clone());

        // Add connections
        self.connections.add(link.neat().from, link.neat().to, ());
    }

    fn mutate_link_weight<C: ConfigProvider<N::Config, L::Config>>(&mut self, config: &C) {
        let neat_config = config.neat();
        let mut rng = rand::thread_rng();

        // Mutate single link
        if !self.links.is_empty() {
            let link_index = rng.gen_range(0, self.links.len());
            if let Some(link) = self.links.values_mut().skip(link_index).next() {
                link.neat_mut().weight +=
                    (rng.gen::<f64>() - 0.5) * 2.0 * neat_config.mutate_link_weight_size;
            }
        }

        /*for link in self.links.values_mut() {
            link.weight += (rng.gen::<f64>() - 0.5) * 2.0 * NEAT.mutate_link_weight_size;
        }*/
    }

    fn mutation_add_node<
        C: ConfigProvider<N::Config, L::Config>,
        S: StateProvider<N::State, L::State>,
    >(
        &mut self,
        config: &C,
        state: &mut S,
    ) {
        // Select random enabled link
        for _ in 0..50 {
            if let Some(index) = self
                .links
                .keys()
                .cloned()
                .collect::<Vec<(NodeRef, NodeRef)>>()
                .choose(&mut rand::thread_rng())
            {
                let link = self.links.get(index).unwrap().neat().clone();
                let innovation = state.neat_mut().get_split_innovation(link.innovation);

                if !self
                    .links
                    .contains_key(&(link.from, NodeRef::Hidden(innovation.node_number)))
                    && !self
                        .links
                        .contains_key(&(NodeRef::Hidden(innovation.node_number), link.to))
                {
                    self.split_link(
                        config,
                        link.from,
                        link.to,
                        innovation.node_number,
                        innovation.innovation_number,
                        state,
                    );
                    break;
                }
            }
        }
    }

    fn mutation_add_link<
        C: ConfigProvider<N::Config, L::Config>,
        S: StateProvider<N::State, L::State>,
    >(
        &mut self,
        config: &C,
        state: &mut S,
    ) {
        let mut rng = rand::thread_rng();

        // Select random source and target nodes for new link
        let num_sources = self.inputs.len() + self.hidden_nodes.len();
        let num_targets = self.hidden_nodes.len() + self.outputs.len();

        if num_sources == 0 || num_targets == 0 {
            return;
        }

        let source_nodes = self
            .inputs
            .keys()
            .chain(self.hidden_nodes.keys())
            .cloned()
            .collect::<Vec<NodeRef>>();
        let source_weights = source_nodes
            .iter()
            .map(|node_ref| num_targets - self.connections.edge_count(node_ref))
            .collect::<Vec<usize>>();
        let mut wheel = vec![source_weights[0]];
        for (i, w) in source_weights.iter().skip(1).enumerate() {
            wheel.push(wheel[i] + w);
        }

        // Netowrk is fully saturated with links
        if *wheel.last().unwrap() <= 0 {
            return;
        }

        let val = rng.gen_range(1, wheel.last().unwrap() + 1);
        let source_index = wheel.binary_search(&val).unwrap_or_else(|x| x);
        let source = source_nodes[source_index];

        let target_nodes = self
            .hidden_nodes
            .keys()
            .chain(self.outputs.keys())
            .filter(|node| !self.links.contains_key(&(source, **node)))
            .cloned()
            .collect::<Vec<NodeRef>>();
        let mut order = (0..target_nodes.len()).collect::<Vec<usize>>();
        order.shuffle(&mut rand::thread_rng());

        // Try to create link with potential target nodes in random order
        for index in order {
            let target = target_nodes[index];

            if !self.connections.creates_cycle(source, target) {
                let innovation = state.neat_mut().get_connect_innovation(source, target);

                self.insert_link(L::new(
                    config.neat_link(),
                    NeatLink::new(
                        source,
                        target,
                        (rng.gen::<f64>() - 0.5) * 2.0 * config.neat().initial_link_weight_size,
                        innovation,
                    ),
                    state.link_mut(),
                ));

                break;
            }
        }
    }

    fn mutation_remove_link(&mut self) {
        if let Some(link_ref) = self
            .links
            .keys()
            .cloned()
            .collect::<Vec<(NodeRef, NodeRef)>>()
            .choose(&mut rand::thread_rng())
        {
            self.links.remove(link_ref);
            self.connections.remove(&link_ref.0, link_ref.1);
        }
    }

    fn mutation_remove_node(&mut self) {
        if let Some(node_ref) = self
            .hidden_nodes
            .keys()
            .cloned()
            .collect::<Vec<NodeRef>>()
            .choose(&mut rand::thread_rng())
        {
            self.hidden_nodes.remove(node_ref);

            for connection in self.connections.remove_node(*node_ref).iter() {
                self.links.remove(&(connection.from, connection.to));
            }
        }
    }
}
