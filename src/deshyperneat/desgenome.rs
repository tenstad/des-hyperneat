use crate::cppn::genome::Genome as CppnGenome;
use evolution::neat::{
    genome::{Link, Node},
    genome_core::GenomeCore,
    node::NodeRef,
    state::NeatStateProvider,
};

pub trait DesGenome {
    type Node: Node<Self::State>;
    type Link: Link<Self::State>;
    type State: NeatStateProvider;

    fn get_node_cppn(&self, node: NodeRef) -> &CppnGenome;
    fn get_link_cppn(&self, source: NodeRef, target: NodeRef) -> &CppnGenome;
    fn get_depth(&self, node: NodeRef) -> usize;
    fn get_core(&self) -> &GenomeCore<Self::Node, Self::Link, Self::State>;
}
