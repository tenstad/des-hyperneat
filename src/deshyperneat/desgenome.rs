use crate::cppn::genome::Genome as CppnGenome;
use evolution::neat::{
    genome::GenomeComponent, genome_core::GenomeCore, link::LinkCore, node::NodeCore,
    node::NodeRef, state::NeatStateProvider,
};

pub trait DesGenome {
    type Node: GenomeComponent<NodeCore, Self::State>;
    type Link: GenomeComponent<LinkCore, Self::State>;
    type State: NeatStateProvider;

    fn get_node_cppn(&self, node: NodeRef) -> &CppnGenome;
    fn get_link_cppn(&self, source: NodeRef, target: NodeRef) -> &CppnGenome;
    fn get_depth(&self, node: NodeRef) -> usize;
    fn get_core(&self) -> &GenomeCore<Self::Node, Self::Link, Self::State>;
}
