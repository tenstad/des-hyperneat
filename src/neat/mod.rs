mod genome;
mod network;

use genome::Genome;
use network::Network;

pub fn neat() {
    let g = Genome::generate(4, 2);
    let n = Network::build(&g);
    let o = n.evaluate(vec![1.0, 2.0, 3.0, 4.0]);
    println!("{:?}", o);
}
