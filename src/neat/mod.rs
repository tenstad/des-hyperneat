mod dot;
mod genome;
mod nodes;

use dot::genome_to_dot;
use genome::Genome;
use std::collections::HashMap;

pub struct InnovationLog {
    pub node_additions: HashMap<u64, InnovationTime>,
    pub edge_additions: HashMap<(nodes::NodeRef, nodes::NodeRef), u64>,
}

impl InnovationLog {
    pub fn new() -> InnovationLog {
        InnovationLog {
            node_additions: HashMap::<u64, InnovationTime>::new(),
            edge_additions: HashMap::<(nodes::NodeRef, nodes::NodeRef), u64>::new(),
        }
    }
}

pub struct InnovationTime {
    node_number: u64,
    innovation_number: u64,
}

impl InnovationTime {
    pub fn new() -> InnovationTime {
        InnovationTime {
            node_number: 0,
            innovation_number: 0,
        }
    }
}

pub fn neat() {
    let mut innovation_log = InnovationLog::new();
    let mut global_innovation = InnovationTime::new();
    let mut g = Genome::generate(4, 2);
    let mut g2 = Genome::generate(4, 2);

    for _ in 0..30 {
        g.mutate(&mut innovation_log, &mut global_innovation);
        g2.mutate(&mut innovation_log, &mut global_innovation);
    }

    let g3 = g.crossover(&g2, true);

    genome_to_dot(String::from("g.dot"), &g);
    genome_to_dot(String::from("g2.dot"), &g2);
    genome_to_dot(String::from("g3.dot"), &g3);

    let o = g3.evaluate(vec![1.0, 2.0, 3.0, 4.0]);
    println!("{:?}", o);
}
