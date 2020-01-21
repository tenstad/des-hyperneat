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

    pub fn crossover(&self, other: Genome) -> Genome {
        let mut nodes: Vec<i64> = Vec::new();
        let mut links: Vec<Link> = Vec::new();

        let mut i1 = 0;
        let mut i2 = 0;

        loop {
            if i1 == self.nodes.len() && i2 == other.nodes.len() {
                break;
            } else if i1 == self.nodes.len() {
                if nodes.len() == 0 || nodes[nodes.len() - 1] != other.nodes[i2] {
                    nodes.push(other.nodes[i2]);
                }
                i2 += 1;
            } else if i2 == other.nodes.len() {
                if nodes.len() == 0 || nodes[nodes.len() - 1] != self.nodes[i1] {
                    nodes.push(self.nodes[i1]);
                }
                i1 += 1;
            } else {
                if self.nodes[i1] < other.nodes[i2] {
                    if nodes.len() == 0 || nodes[nodes.len() - 1] != self.nodes[i1] {
                        nodes.push(self.nodes[i1]);
                    }
                    i1 += 1;
                } else {
                    if nodes.len() == 0 || nodes[nodes.len() - 1] != other.nodes[i2] {
                        nodes.push(other.nodes[i2]);
                    }
                    i2 += 1;
                }
            }
        }

        let mut i1 = 0;
        let mut i2 = 0;
        let mut rng = rand::thread_rng();

        loop {
            if i1 == self.links.len() && i2 == other.links.len() {
                break;
            } else if i1 == self.links.len() {
                if links.len() == 0
                    || links[links.len() - 1].innovation != other.links[i2].innovation
                {
                    links.push(other.links[i2]);
                }
                i2 += 1;
            } else if i2 == other.links.len() {
                if links.len() == 0
                    || links[links.len() - 1].innovation != self.links[i1].innovation
                {
                    links.push(self.links[i1]);
                }
                i1 += 1;
            } else {
                if self.links[i1].innovation == other.links[i2].innovation {
                    if rng.gen_range(0, 2) == 0 {
                        links.push(self.links[i1]);
                    } else {
                        links.push(other.links[i2]);
                    }
                    i1 += 1;
                    i2 += 1;
                } else if self.links[i1].innovation < other.links[i2].innovation {
                    if links.len() == 0
                        || links[links.len() - 1].innovation != self.links[i1].innovation
                    {
                        links.push(self.links[i1]);
                    }
                    i1 += 1;
                } else {
                    if links.len() == 0
                        || links[links.len() - 1].innovation != other.links[i2].innovation
                    {
                        links.push(other.links[i2]);
                    }
                    i2 += 1;
                }
            }
        }

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
                self.links.push(Link {
                    from: self.links[link_index].from,
                    to: addition.node_number + self.inputs + self.outputs,
                    weight: 1.0,
                    enabled: true,
                    innovation: addition.innovation_number,
                });
                self.links.push(Link {
                    from: addition.node_number + self.inputs + self.outputs,
                    to: self.links[link_index].to,
                    weight: self.links[link_index].weight,
                    enabled: true,
                    innovation: addition.innovation_number + 1,
                });
                let mut pushed = false;
                for (i, node) in self.nodes.iter().enumerate() {
                    if addition.node_number + self.inputs + self.outputs < *node {
                        self.nodes
                            .insert(i, addition.node_number + self.inputs + self.outputs);
                        pushed = true;
                        break;
                    }
                }
                if !pushed {
                    self.nodes
                        .push(addition.node_number + self.inputs + self.outputs);
                }
            }
            None => {
                log.node_additions.insert(
                    self.links[link_index].innovation,
                    InnovationTime {
                        node_number: global_innovation.node_number,
                        innovation_number: global_innovation.innovation_number,
                    },
                );
                self.links.push(Link {
                    from: self.links[link_index].from,
                    to: global_innovation.node_number + self.inputs + self.outputs,
                    weight: 1.0,
                    enabled: true,
                    innovation: global_innovation.innovation_number,
                });
                self.links.push(Link {
                    from: global_innovation.node_number + self.inputs + self.outputs,
                    to: self.links[link_index].to,
                    weight: self.links[link_index].weight,
                    enabled: true,
                    innovation: global_innovation.innovation_number + 1,
                });
                self.nodes
                    .push(global_innovation.node_number + self.inputs + self.outputs);
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
