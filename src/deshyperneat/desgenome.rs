use crate::cppn::genome::Genome as CppnGenome;
use evolution::neat::{
    genome::{Link, Node},
    genome_core::GenomeCore,
    node::NodeRef,
};

pub trait DesGenome {
    type Node: Node;
    type Link: Link;

    fn get_node_cppn(&self, node: NodeRef) -> &CppnGenome;
    fn get_link_cppn(&self, source: NodeRef, target: NodeRef) -> &CppnGenome;
    fn get_depth(&self, node: NodeRef) -> usize;
    fn get_core(&self) -> &GenomeCore<Self::Node, Self::Link>;
}
