use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::{conf::DESHYPERNEAT, state::CustomState};
use evolution::{
    genome::GenericGenome,
    neat::{
        conf::NeatConfig,
        genome::GetNeat,
        node::{NeatNode, NodeExtension},
        state::{InitConfig, NeatState},
    },
};
use rand::Rng;

#[derive(Clone, GetNeat, new)]
pub struct Node {
    #[neat]
    pub neat: NeatNode,
    pub cppn: CppnGenome,
    pub depth: usize,
}

impl NodeExtension for Node {
    type Config = NeatConfig;
    type State = CustomState;

    fn new(config: &Self::Config, neat: NeatNode, state: &mut Self::State) -> Self {
        let init_conf = InitConfig::new(4, 2);

        let cppn = if DESHYPERNEAT.single_cppn_state {
            CppnGenome::new(config, &init_conf, &mut state.single_cppn_state)
        } else if let Some(cppn_state) = state
            .unique_cppn_states
            .get_mut(&(neat.node_ref, neat.node_ref))
        {
            CppnGenome::new(config, &init_conf, cppn_state)
        } else {
            let mut cppn_state = NeatState::default();
            let cppn = CppnGenome::new(config, &init_conf, &mut cppn_state);
            state
                .unique_cppn_states
                .insert((neat.node_ref, neat.node_ref), cppn_state);
            cppn
        };

        Self::new(neat, cppn, 1)
    }

    fn crossover(
        &self,
        config: &Self::Config,
        other: &Self,
        fitness: &f64,
        other_fitness: &f64,
    ) -> Self {
        Self {
            neat: self.neat.crossover(&other.neat, fitness, other_fitness),
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
        let mut distance = self.neat.distance(&other.neat);
        distance += 0.8 * self.cppn.distance(config, &other.cppn);
        distance += 0.2 * (self.depth as f64 - other.depth as f64).abs().tanh();
        distance
    }
}
