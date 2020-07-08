use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::{conf::DESHYPERNEAT, genome::Genome, link::Link, node::Node};
use evolution::neat::{
    genome::NeatGenome,
    link::LinkExtension as NeatLink,
    node::{NodeExtension as NeatNode, NodeRef},
};

pub trait DesGenome {
    type Node: NeatNode;
    type Link: NeatLink;

    fn init_desgenome(&mut self) {}
    fn get_node_cppn(&self, node: &NodeRef) -> &CppnGenome;
    fn get_link_cppn(&self, source: NodeRef, target: NodeRef) -> &CppnGenome;
    fn get_depth(&self, node: &NodeRef) -> u64;
    fn get_neat(&self) -> &NeatGenome<Self::Node, Self::Link>;
}

impl DesGenome for Genome {
    type Node = Node;
    type Link = Link;

    fn get_node_cppn(&self, node: &NodeRef) -> &CppnGenome {
        &self.neat.get_node(node).unwrap().cppn
    }

    fn get_link_cppn(&self, source: NodeRef, target: NodeRef) -> &CppnGenome {
        &self.neat.links.get(&(source, target)).unwrap().cppn
    }

    fn get_depth(&self, node: &NodeRef) -> u64 {
        if DESHYPERNEAT.static_substrate_depth >= 0 {
            match node {
                NodeRef::Hidden(_) => DESHYPERNEAT.static_substrate_depth as u64,
                _ => 0,
            }
        } else {
            self.neat.get_node(node).unwrap().depth
        }
    }

    fn get_neat(&self) -> &NeatGenome<Self::Node, Self::Link> {
        &self.neat
    }
}
