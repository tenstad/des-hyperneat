use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::{
    conf::GenomeConfig as DesConfig, desgenome::DesGenome, genome::Genome as DesGenomeStruct,
    link::Link, node::Node, state::State,
};
use crate::sideshyperneat::genome::Genome;
use evolution::{
    genome::GenericGenome,
    neat::{genome::NeatGenome, node::NodeRef, state::InitConfig},
};
use std::collections::HashMap;

impl DesGenome for Genome {
    type Node = Node;
    type Link = Link;

    fn init_desgenome(&mut self) {
        let config = DesConfig::default();
        let mut des_neat =
            NeatGenome::<Node, Link>::new(&config, &InitConfig::new(0, 0), &mut State::default());

        des_neat.connections = self.topology.connections.clone();
        des_neat.inputs = self
            .topology
            .inputs
            .iter()
            .map(|(node_ref, node)| {
                let mut cppn = self.cppn.clone();
                cppn.neat
                    .connections
                    .prune_dangling_outputs(&vec![NodeRef::Output(node.cppn_output_id)]);
                cppn.neat
                    .outputs
                    .retain(|node_ref, _| node_ref.id() == node.cppn_output_id);
                (*node_ref, Node::new(node.neat.clone(), cppn, node.depth))
            })
            .collect::<HashMap<NodeRef, Node>>();
        des_neat.hidden_nodes = self
            .topology
            .hidden_nodes
            .iter()
            .map(|(node_ref, node)| {
                let mut cppn = self.cppn.clone();
                cppn.neat
                    .connections
                    .prune_dangling_outputs(&vec![NodeRef::Output(node.cppn_output_id)]);
                cppn.neat
                    .outputs
                    .retain(|node_ref, _| node_ref.id() == node.cppn_output_id);
                (*node_ref, Node::new(node.neat.clone(), cppn, node.depth))
            })
            .collect::<HashMap<NodeRef, Node>>();
        des_neat.outputs = self
            .topology
            .outputs
            .iter()
            .map(|(node_ref, node)| {
                let mut cppn = self.cppn.clone();
                cppn.neat
                    .connections
                    .prune_dangling_outputs(&vec![NodeRef::Output(node.cppn_output_id)]);
                cppn.neat
                    .outputs
                    .retain(|node_ref, _| node_ref.id() == node.cppn_output_id);
                (*node_ref, Node::new(node.neat.clone(), cppn, node.depth))
            })
            .collect::<HashMap<NodeRef, Node>>();
        des_neat.links = self
            .topology
            .links
            .iter()
            .map(|(key, link)| {
                let mut cppn = self.cppn.clone();
                cppn.neat
                    .connections
                    .prune_dangling_outputs(&vec![NodeRef::Output(link.cppn_output_id)]);
                cppn.neat
                    .outputs
                    .retain(|node_ref, _| node_ref.id() == link.cppn_output_id);
                (*key, Link::new(link.neat.clone(), cppn, link.depth))
            })
            .collect::<HashMap<(NodeRef, NodeRef), Link>>();

        self.des_genome = Some(DesGenomeStruct { neat: des_neat });
    }

    fn get_node_cppn(&self, node: &NodeRef) -> &CppnGenome {
        &self
            .des_genome
            .as_ref()
            .unwrap()
            .neat
            .get_node(node)
            .unwrap()
            .cppn
    }

    fn get_link_cppn(&self, source: NodeRef, target: NodeRef) -> &CppnGenome {
        &self
            .des_genome
            .as_ref()
            .unwrap()
            .neat
            .links
            .get(&(source, target))
            .unwrap()
            .cppn
    }

    fn get_depth(&self, node: &NodeRef) -> u64 {
        self.des_genome
            .as_ref()
            .unwrap()
            .neat
            .get_node(node)
            .unwrap()
            .depth
    }

    fn get_neat(&self) -> &NeatGenome<Self::Node, Self::Link> {
        &self.des_genome.as_ref().unwrap().neat
    }
}
