use crate::neat::genome;
use std::collections::HashMap;

pub struct DenseGenome {
    pub inputs: i64,
    pub outputs: i64,
    pub nodes: i64,
    pub links: Vec<Link>,
}

pub struct Network {
    pub inputs: i64,
    pub outputs: i64,
    pub nodes: i64,
    pub links: Vec<Link>,
}

pub struct Link {
    pub from: i64,
    pub to: i64,
    pub weight: f64,
}

pub struct LinkLookup {
    forward_start_index: Vec<i64>,
    forward_count: Vec<i64>,
    forward_link_index: Vec<i64>,
    backward_start_index: Vec<i64>,
    backward_count: Vec<i64>,
    backward_link_index: Vec<i64>,
}

impl Network {
    pub fn build(genome: &genome::Genome) -> Network {
        let genome = condence_genome_node_values(&genome);
        let mut link_lookup = create_link_lookup(&genome);
        prune_network(&genome, &mut link_lookup);
        let order = sort_topological(&genome, &mut link_lookup);

        let mut links: Vec<Link> = Vec::new();
        for node in order.iter() {
            let node = *node as usize;
            let forward_start_index = link_lookup.forward_start_index[node] as usize;
            let forward_count = link_lookup.forward_count[node];

            for j in 0..forward_count {
                let link_index =
                    link_lookup.forward_link_index[forward_start_index + j as usize] as usize;
                let to: i64 = genome.links[link_index].to;
                links.push(Link {
                    from: node as i64,
                    to: to,
                    weight: genome.links[link_index].weight,
                });
            }
        }

        return Network {
            inputs: genome.inputs,
            outputs: genome.outputs,
            nodes: genome.nodes,
            links: links,
        };
    }

    pub fn evaluate(&self, inputs: Vec<f64>) -> Vec<f64> {
        let mut values: Vec<f64> = vec![0.0; self.nodes as usize];
        for i in 0..self.inputs {
            values[i as usize] = inputs[i as usize];
        }

        for link in self.links.iter() {
            values[link.to as usize] += link.weight * values[link.from as usize];
        }

        let mut result: Vec<f64> = vec![0.0; self.outputs as usize];
        for i in 0..self.outputs {
            result[i as usize] = values[(self.inputs + i) as usize];
        }

        return result;
    }
}

/// Convert genome to condenced version, containing stricly increasing node numbers and only enabled linksS
pub fn condence_genome_node_values(genome: &genome::Genome) -> DenseGenome {
    let mut new_id = HashMap::<i64, i64>::new();

    for (i, node) in genome.nodes.iter().enumerate() {
        new_id.insert(*node, (i as i64) + genome.inputs + genome.outputs);
    }

    let mut links: Vec<Link> = Vec::new();

    for link in genome.links.iter() {
        if link.enabled {
            links.push(Link {
                from: *new_id.get(&link.from).unwrap_or(&link.from),
                to: *new_id.get(&link.to).unwrap_or(&link.to),
                weight: link.weight,
            });
        }
    }

    return DenseGenome {
        inputs: genome.inputs,
        outputs: genome.outputs,
        nodes: genome.inputs + genome.outputs + genome.nodes.len() as i64,
        links: links,
    };
}

/// Creat node centered link lookup
pub fn create_link_lookup(genome: &DenseGenome) -> LinkLookup {
    let node_count = genome.nodes as usize;

    let mut forward_count: Vec<i64> = vec![0; node_count];
    let mut backward_count: Vec<i64> = vec![0; node_count];
    for link in genome.links.iter() {
        forward_count[(link.from) as usize] += 1;
        backward_count[(link.to) as usize] += 1;
    }

    let mut forward_start_index: Vec<i64> = vec![0; node_count];
    let mut backward_start_index: Vec<i64> = vec![0; node_count];
    for i in 1..node_count {
        forward_start_index[i] = forward_start_index[i - 1] + forward_count[i - 1];
        backward_start_index[i] = backward_start_index[i - 1] + backward_count[i - 1];
    }

    let mut forward_insert_index = forward_start_index.clone();
    let mut backward_insert_index = backward_start_index.clone();

    let mut forward_link_index: Vec<i64> = vec![0; genome.links.len()];
    let mut backward_link_index: Vec<i64> = vec![0; genome.links.len()];
    for (i, link) in genome.links.iter().enumerate() {
        forward_link_index[forward_insert_index[link.from as usize] as usize] = i as i64;
        backward_link_index[backward_insert_index[link.to as usize] as usize] = i as i64;
        forward_insert_index[link.from as usize] += 1;
        backward_insert_index[link.from as usize] += 1;
    }

    return LinkLookup {
        forward_start_index,
        forward_count,
        forward_link_index,
        backward_start_index,
        backward_count,
        backward_link_index,
    };
}

/// Prune network of dangeling nodes
pub fn prune_network(genome: &DenseGenome, link_lookup: &mut LinkLookup) {
    let mut forward_prune_nodes: Vec<i64> =
        ((genome.inputs + genome.outputs)..genome.nodes).collect();
    let mut backward_prune_nodes = forward_prune_nodes.clone();

    while let Some(node) = forward_prune_nodes.pop() {
        let backward_count = link_lookup.backward_count[node as usize];
        if backward_count == 0 {
            let forward_start_index = link_lookup.forward_start_index[node as usize] as usize;
            let forward_count = link_lookup.forward_count[node as usize];
            for i in 0..forward_count {
                let i = i as usize;
                let link_index = link_lookup.forward_link_index[forward_start_index + i] as usize;
                let other: i64 = genome.links[link_index].to;

                let backward_start_index =
                    link_lookup.backward_start_index[other as usize] as usize;
                let backward_count = link_lookup.backward_count[other as usize];

                for j in 0..backward_count {
                    let link_index =
                        link_lookup.backward_link_index[backward_start_index + j as usize] as usize;
                    let other_other: i64 = genome.links[link_index].from;
                    if node == other_other {
                        for k in j..(backward_count - 1) {
                            let k_index = backward_start_index + (j + k) as usize;
                            link_lookup.backward_link_index[k_index] =
                                link_lookup.backward_link_index[k_index + 1];
                        }
                        break;
                    }
                }

                link_lookup.backward_count[other as usize] -= 1;
                if backward_count == 0 {
                    if !forward_prune_nodes.contains(&other) {
                        forward_prune_nodes.push(other);
                    }
                }
            }
        }
    }

    while let Some(node) = backward_prune_nodes.pop() {
        let forward_count = link_lookup.forward_count[node as usize];
        if forward_count == 0 {
            let backward_start_index = link_lookup.backward_start_index[node as usize] as usize;
            let backward_count = link_lookup.backward_count[node as usize];
            for i in 0..backward_count {
                let i = i as usize;
                let link_index = link_lookup.backward_link_index[backward_start_index + i] as usize;
                let other: i64 = genome.links[link_index].from;

                let forward_start_index = link_lookup.forward_start_index[other as usize] as usize;
                let forward_count = link_lookup.forward_count[other as usize];

                for j in 0..forward_count {
                    let link_index =
                        link_lookup.forward_link_index[forward_start_index + j as usize] as usize;
                    let other_other: i64 = genome.links[link_index].to;
                    if node == other_other {
                        for k in j..(forward_count - 1) {
                            let k_index = forward_start_index + (j + k) as usize;
                            link_lookup.forward_link_index[k_index] =
                                link_lookup.forward_link_index[k_index + 1];
                        }
                        break;
                    }
                }

                link_lookup.forward_count[other as usize] -= 1;
                if forward_count == 0 {
                    if !backward_prune_nodes.contains(&other) {
                        backward_prune_nodes.push(other);
                    }
                }
            }
        }
    }
}

fn sort_topological(genome: &DenseGenome, link_lookup: &mut LinkLookup) -> Vec<i64> {
    let mut order = Vec::<i64>::new();
    let mut stack: Vec<i64> = (0..genome.inputs).collect();

    let mut backward_count = link_lookup.backward_count.clone();

    while let Some(node) = stack.pop() {
        order.push(node);
        let forward_start_index = link_lookup.forward_start_index[node as usize] as usize;
        let forward_count = link_lookup.forward_count[node as usize];
        for i in 0..forward_count {
            let i = i as usize;
            let link_index = link_lookup.forward_link_index[forward_start_index + i] as usize;
            let other: i64 = genome.links[link_index].to;

            backward_count[other as usize] -= 1;
            if backward_count[other as usize] == 0 {
                stack.push(other);
            }
        }
    }

    return order;
}
