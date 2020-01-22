use crate::neat::InnovationLog;
use crate::neat::InnovationTime;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Link {
    pub from: i64,
    pub to: i64,
    pub weight: f64,
    pub enabled: bool,
    pub innovation: i64,
}

pub struct Genome {
    pub inputs: i64,
    pub outputs: i64,
    pub nodes: Vec<i64>,
    pub links: Vec<Link>,
}

impl Genome {
    pub fn generate(inputs: i64, outputs: i64) -> Genome {
        return Genome {
            inputs: inputs,
            outputs: outputs,
            nodes: Vec::new(),
            links: Vec::new(),
        };
    }

    pub fn mutate(&mut self, log: &mut InnovationLog, global_innovation: &mut InnovationTime) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < 0.2 {
            self.add_node(log, global_innovation);
        }
        if rng.gen::<f64>() < 0.2 {
            self.add_connection(log, global_innovation);
        }
    }

    pub fn crossover(&self, other: &Genome) -> Genome {
        let mut nodes: Vec<i64> = Vec::new();
        let mut i = 0;
        let mut j = 0;
        let l1 = self.nodes.len();
        let l2 = other.nodes.len();

        // Select the smallest nodes and pick one when the are equal, to avoid duplicates
        while i < l1 && j < l2 {
            if self.nodes[i] == other.nodes[j] {
                nodes.push(self.nodes[i]);
                i += 1;
                j += 1;
            } else if self.nodes[i] < other.nodes[j] {
                nodes.push(self.nodes[i]);
                i += 1;
            } else {
                nodes.push(other.nodes[j]);
                j += 1;
            }
        }

        // Add remaining elements from the other list when the first reached the end
        nodes.extend(self.nodes.iter().skip(i));
        nodes.extend(other.nodes.iter().skip(j));

        let mut links: Vec<Link> = Vec::new();
        let mut i = 0;
        let mut j = 0;
        let l1 = self.links.len();
        let l2 = other.links.len();
        let mut rng = rand::thread_rng();

        // Select the smallest nodes and pick one when the are equal, to avoid duplicates
        while i < l1 && j < l2 {
            if self.links[i].innovation == other.links[j].innovation {
                if rng.gen_range(0, 2) == 0 {
                    links.push(self.links[i]);
                } else {
                    links.push(other.links[j]);
                }
                i += 1;
                j += 1;
            } else if self.links[i].innovation < other.links[j].innovation {
                links.push(self.links[i]);
                i += 1;
            } else {
                links.push(other.links[j]);
                j += 1;
            }
        }

        // Add remaining elements from the other list when the first reached the end
        links.extend(self.links.iter().skip(i));
        links.extend(other.links.iter().skip(j));

        return Genome {
            inputs: self.inputs,
            outputs: self.outputs,
            nodes: nodes,
            links: links,
        };
    }

    pub fn add_node(&mut self, log: &mut InnovationLog, global_innovation: &mut InnovationTime) {
        if self.links.len() == 0 {
            return;
        }

        // Find indexes of all enabled links
        let link_indexes: Vec<usize> = self
            .links
            .iter()
            .enumerate()
            .filter(|(_, link)| link.enabled)
            .map(|(i, _)| i)
            .collect();

        // Select random enabled link to split
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0, link_indexes.len());
        let link_index = link_indexes[r];

        // Disable the link beeing split
        self.links[link_index].enabled = false;

        // Check if this link has been split by another individual
        match log.node_additions.get(&self.links[link_index].innovation) {
            Some(addition) => {
                let new_node = addition.node_number + self.inputs + self.outputs;

                // Add links from/to new node
                self.links.push(Link {
                    from: self.links[link_index].from,
                    to: new_node,
                    weight: 1.0,
                    enabled: true,
                    innovation: addition.innovation_number,
                });
                self.links.push(Link {
                    from: new_node,
                    to: self.links[link_index].to,
                    weight: self.links[link_index].weight,
                    enabled: true,
                    innovation: addition.innovation_number + 1,
                });

                // Insert new node into node list
                match self.nodes.binary_search(&new_node) {
                    Ok(_) => {}
                    Err(pos) => self.nodes.insert(pos, new_node),
                }
            }
            None => {
                // Add this mutation to log
                log.node_additions.insert(
                    self.links[link_index].innovation,
                    InnovationTime {
                        node_number: global_innovation.node_number,
                        innovation_number: global_innovation.innovation_number,
                    },
                );

                let new_node = global_innovation.node_number + self.inputs + self.outputs;

                // Add links from/to new node
                self.links.push(Link {
                    from: self.links[link_index].from,
                    to: new_node,
                    weight: 1.0,
                    enabled: true,
                    innovation: global_innovation.innovation_number,
                });
                self.links.push(Link {
                    from: new_node,
                    to: self.links[link_index].to,
                    weight: self.links[link_index].weight,
                    enabled: true,
                    innovation: global_innovation.innovation_number + 1,
                });

                // Push new node into node list
                self.nodes.push(new_node);

                // Increase global node count and innovation number
                global_innovation.node_number += 1;
                global_innovation.innovation_number += 2;
            }
        }
    }

    pub fn add_connection(
        &mut self,
        log: &mut InnovationLog,
        global_innovation: &mut InnovationTime,
    ) {
        let mut rng = rand::thread_rng();

        let mut from: i64 = 0;
        let mut to: i64 = 0;
        let mut exists = false;

        for _ in 0..50 {
            from = rng.gen_range(0, self.inputs + (self.nodes.len() as i64));
            to = rng.gen_range(0, self.outputs + (self.nodes.len() as i64));
            if from >= self.inputs {
                from = self.nodes[(from - self.inputs) as usize];
            }
            if to >= self.outputs {
                to = self.nodes[(to - self.outputs) as usize];
            } else {
                to += self.inputs;
            }
            exists = false;
            for link in self.links.iter() {
                if link.from == from && link.to == to {
                    exists = true;
                    break;
                }
            }
            if !exists {
                break;
            }
        }

        if !exists {
            match log.edge_additions.get(&(from, to)) {
                Some(innovation_number) => {
                    self.links.push(Link {
                        from: from,
                        to: to,
                        weight: 1.0,
                        enabled: true,
                        innovation: *innovation_number,
                    });
                }
                None => {
                    self.links.push(Link {
                        from: from,
                        to: to,
                        weight: 1.0,
                        enabled: true,
                        innovation: global_innovation.innovation_number,
                    });
                    log.edge_additions
                        .insert((from, to), global_innovation.innovation_number);
                    global_innovation.innovation_number += 1;
                }
            }
        }
    }
}
