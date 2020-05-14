use crate::codeshyperneat::{genome::Genome as BlueprintGenome, link::Link, node::Node};
use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::desgenome::DesGenome;
use evolution::neat::{genome::NeatGenome, node::NodeRef};
use std::collections::HashMap;

#[derive(new, Clone)]
pub struct CombinedGenome {
    pub blueprint: BlueprintGenome,
    pub modules: HashMap<u64, (usize, CppnGenome)>,
}

impl DesGenome for CombinedGenome {
    type Node = Node;
    type Link = Link;

    fn get_node_cppn(&self, node: &NodeRef) -> &CppnGenome {
        &self
            .modules
            .get(&self.blueprint.neat.get_node(node).unwrap().module_species)
            .unwrap()
            .1
    }

    fn get_link_cppn(&self, source: NodeRef, target: NodeRef) -> &CppnGenome {
        &self
            .modules
            .get(
                &self
                    .blueprint
                    .neat
                    .links
                    .get(&(source, target))
                    .unwrap()
                    .module_species,
            )
            .unwrap()
            .1
    }

    fn get_depth(&self, node: &NodeRef) -> u64 {
        self.blueprint.neat.get_node(node).unwrap().depth
    }

    fn get_neat(&self) -> &NeatGenome<Self::Node, Self::Link> {
        &self.blueprint.neat
    }
}
