use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::{
    desgenome::DesGenome, genome::Genome as DesGenomeStruct, link::Link, node::Node, state::State,
};
use crate::sideshyperneat::genome::Genome;
use evolution::{
    genome::GenericGenome,
    neat::{genome_core::GenomeCore, node::NodeRef, state::InitConfig},
};
use std::collections::HashMap;

impl DesGenome for Genome {
    type Node = Node;
    type Link = Link;

    fn init_desgenome(&mut self) {
        let mut des_core =
            GenomeCore::<Node, Link>::new(&InitConfig::new(0, 0), &mut State::default());

        des_core.connections = self.topology.connections.clone();
        des_core.inputs = self
            .topology
            .inputs
            .iter()
            .map(|(node_ref, node)| {
                let mut cppn = self.cppn.clone();
                cppn.core
                    .connections
                    .prune_dangling_outputs(&vec![NodeRef::Output(node.cppn_output_id)]);
                cppn.core
                    .outputs
                    .retain(|node_ref, _| node_ref.id() == node.cppn_output_id);
                (*node_ref, Node::new(node.core.clone(), cppn, node.depth))
            })
            .collect::<HashMap<NodeRef, Node>>();
        des_core.hidden_nodes = self
            .topology
            .hidden_nodes
            .iter()
            .map(|(node_ref, node)| {
                let mut cppn = self.cppn.clone();
                cppn.core
                    .connections
                    .prune_dangling_outputs(&vec![NodeRef::Output(node.cppn_output_id)]);
                cppn.core
                    .outputs
                    .retain(|node_ref, _| node_ref.id() == node.cppn_output_id);
                (*node_ref, Node::new(node.core.clone(), cppn, node.depth))
            })
            .collect::<HashMap<NodeRef, Node>>();
        des_core.outputs = self
            .topology
            .outputs
            .iter()
            .map(|(node_ref, node)| {
                let mut cppn = self.cppn.clone();
                cppn.core
                    .connections
                    .prune_dangling_outputs(&vec![NodeRef::Output(node.cppn_output_id)]);
                cppn.core
                    .outputs
                    .retain(|node_ref, _| node_ref.id() == node.cppn_output_id);
                (*node_ref, Node::new(node.core.clone(), cppn, node.depth))
            })
            .collect::<HashMap<NodeRef, Node>>();
        des_core.links = self
            .topology
            .links
            .iter()
            .map(|(key, link)| {
                let mut cppn = self.cppn.clone();
                cppn.core
                    .connections
                    .prune_dangling_outputs(&vec![NodeRef::Output(link.cppn_output_id)]);
                cppn.core
                    .outputs
                    .retain(|node_ref, _| node_ref.id() == link.cppn_output_id);
                (*key, Link::new(link.core.clone(), cppn, link.depth))
            })
            .collect::<HashMap<(NodeRef, NodeRef), Link>>();

        self.des_genome = Some(DesGenomeStruct { core: des_core });
    }

    fn get_node_cppn(&self, node: &NodeRef) -> &CppnGenome {
        &self
            .des_genome
            .as_ref()
            .unwrap()
            .core
            .get_node(node)
            .unwrap()
            .cppn
    }

    fn get_link_cppn(&self, source: NodeRef, target: NodeRef) -> &CppnGenome {
        &self
            .des_genome
            .as_ref()
            .unwrap()
            .core
            .links
            .get(&(source, target))
            .unwrap()
            .cppn
    }

    fn get_depth(&self, node: &NodeRef) -> usize {
        self.des_genome
            .as_ref()
            .unwrap()
            .core
            .get_node(node)
            .unwrap()
            .depth
    }

    fn get_core(&self) -> &GenomeCore<Self::Node, Self::Link> {
        &self.des_genome.as_ref().unwrap().core
    }
}
