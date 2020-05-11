use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::{conf::DESHYPERNEAT, state::CustomState};
use evolution::neat::{
    conf::NeatConfig,
    genome::{Genome, GetCore, Node as NeatNode},
    node::NodeCore,
    state::{InitConfig, StateCore},
};
use rand::Rng;

#[derive(Clone, GetCore, new)]
pub struct Node {
    #[core]
    pub core: NodeCore,
    pub cppn: CppnGenome,
    pub depth: usize,
}

impl NeatNode for Node {
    type Config = NeatConfig;
    type State = CustomState;

    fn new(config: &Self::Config, core: NodeCore, state: &mut Self::State) -> Self {
        let init_conf = InitConfig::new(4, 2);

        let cppn = if DESHYPERNEAT.single_cppn_state {
            CppnGenome::new(config, &init_conf, &mut state.single_cppn_state)
        } else if let Some(cppn_state) = state
            .unique_cppn_states
            .get_mut(&(core.node_ref, core.node_ref))
        {
            CppnGenome::new(config, &init_conf, cppn_state)
        } else {
            let mut cppn_state = StateCore::default();
            let cppn = CppnGenome::new(config, &init_conf, &mut cppn_state);
            state
                .unique_cppn_states
                .insert((core.node_ref, core.node_ref), cppn_state);
            cppn
        };

        Self::new(core, cppn, 1)
    }

    fn crossover(
        &self,
        config: &Self::Config,
        other: &Self,
        fitness: &f64,
        other_fitness: &f64,
    ) -> Self {
        Self {
            core: self.core.crossover(&other.core, fitness, other_fitness),
            cppn: self
                .cppn
                .crossover(config, &other.cppn, fitness, other_fitness),
            depth: if rand::thread_rng().gen::<bool>() {
                self.depth
            } else {
                other.depth
            },
        }
    }

    fn distance(&self, config: &Self::Config, other: &Self) -> f64 {
        let mut distance = self.core.distance(&other.core);
        distance += 0.8 * self.cppn.distance(config, &other.cppn);
        distance += 0.2 * (self.depth as f64 - other.depth as f64).abs().tanh();
        distance
    }
}
