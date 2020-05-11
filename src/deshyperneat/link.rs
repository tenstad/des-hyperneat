use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::state::CustomState;
use crate::eshyperneat::genome::identity_genome;
use evolution::neat::{
    conf::NeatConfig,
    genome::{Genome, GetCore, Link as NeatLink},
    link::LinkCore,
    state::{InitConfig, StateCore},
};
use rand::Rng;

#[derive(Clone, GetCore, new)]
pub struct Link {
    #[core]
    pub core: LinkCore,
    pub cppn: CppnGenome,
    pub depth: usize,
}

impl NeatLink for Link {
    type Config = NeatConfig;
    type State = CustomState;

    fn new(config: &Self::Config, core: LinkCore, state: &mut Self::State) -> Self {
        let init_conf = InitConfig::new(4, 2);
        let mut cppn_state = StateCore::default();
        let cppn = CppnGenome::new(config, &init_conf, &mut cppn_state);

        if !state.unique_cppn_states.contains_key(&(core.from, core.to)) {
            state
                .unique_cppn_states
                .insert((core.from, core.to), cppn_state);
        }

        Self::new(core, cppn, 1)
    }

    fn identity(_: &Self::Config, core: LinkCore, state: &mut Self::State) -> Self {
        let (cppn, cppn_state) = identity_genome();

        if !state.unique_cppn_states.contains_key(&(core.from, core.to)) {
            state
                .unique_cppn_states
                .insert((core.from, core.to), cppn_state);
        }

        Self::new(core, cppn, 1)
    }

    fn clone_with(&self, _: &Self::Config, core: LinkCore, state: &mut Self::State) -> Self {
        if !state
            .cppn_state_redirects
            .contains_key(&(core.from, core.to))
        {
            let key = (self.core.from, self.core.to);
            state.cppn_state_redirects.insert(
                (core.from, core.to),
                if let Some(redirect) = state.cppn_state_redirects.get(&key) {
                    *redirect
                } else {
                    key
                },
            );
        }

        Self::new(core, self.cppn.clone(), self.depth)
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
        let mut distance = 0.5 * self.core.distance(&other.core);
        distance += 0.4 * self.cppn.distance(config, &other.cppn);
        distance += 0.1 * ((self.depth - other.depth) as f64).abs().tanh();
        distance
    }
}
