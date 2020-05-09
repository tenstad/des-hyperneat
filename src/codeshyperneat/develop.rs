use crate::codeshyperneat::{genome::Genome as BlueprintGenome, link::Link, node::Node};
use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::desgenome::DesGenome;
use evolution::neat::{genome_core::GenomeCore, node::NodeRef};
use std::collections::HashMap;

#[derive(new, Clone)]
pub struct CombinedGenome {
    pub blueprint: BlueprintGenome,
    pub modules: HashMap<usize, (usize, CppnGenome)>,
}

impl DesGenome for CombinedGenome {
    type Node = Node;
    type Link = Link;

    fn get_node_cppn(&self, node: &NodeRef) -> &CppnGenome {
        &self
            .modules
            .get(&self.blueprint.core.get_node(node).unwrap().module_species)
            .unwrap()
            .1
    }

    fn get_link_cppn(&self, source: NodeRef, target: NodeRef) -> &CppnGenome {
        &self
            .modules
            .get(
                &&self
                    .blueprint
                    .core
                    .links
                    .get(&(source, target))
                    .unwrap()
                    .module_species,
            )
            .unwrap()
            .1
    }

    fn get_depth(&self, node: &NodeRef) -> usize {
        self.blueprint.core.get_node(node).unwrap().depth
    }

    fn get_core(&self) -> &GenomeCore<Self::Node, Self::Link> {
        &self.blueprint.core
    }
}
