mod genome;
mod network;

use genome::Genome;
use network::Network;

use std::collections::HashMap;

pub struct InnovationLog {
    pub node_additions: HashMap<i64, InnovationTime>,
    pub edge_additions: HashMap<(i64, i64), i64>,
}

impl InnovationLog {
    pub fn new() -> InnovationLog {
        InnovationLog {
            node_additions: HashMap::<i64, InnovationTime>::new(),
            edge_additions: HashMap::<(i64, i64), i64>::new(),
        }
    }
}

pub struct InnovationTime {
    node_number: i64,
    innovation_number: i64,
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

    for link in g.links.iter() {
        println!("{} {}", link.from, link.to);
    }
    println!("{:?}", g.nodes);

    let n = Network::build(&g);
    let o = n.evaluate(vec![1.0, 2.0, 3.0, 4.0]);
    println!("{:?}", o);
}
