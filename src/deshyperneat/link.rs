use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::state::CustomState;
use crate::eshyperneat::genome::identity_genome;
use evolution::neat::{
    genome::{Genome, Link as NeatLink},
    link::LinkCore,
    state::{InitConfig, StateCore},
};
use rand::Rng;

#[derive(Clone, new)]
pub struct Link {
    pub core: LinkCore,
    pub cppn: CppnGenome,
    pub depth: usize,
}

impl NeatLink for Link {
    type State = CustomState;

    fn new(core: LinkCore, state: &mut Self::State) -> Self {
        let init_conf = InitConfig::new(4, 2);
        let mut cppn_state = StateCore::default();
        let cppn = CppnGenome::new(&init_conf, &mut cppn_state);

        if !state.unique_cppn_states.contains_key(&(core.from, core.to)) {
            state
                .unique_cppn_states
                .insert((core.from, core.to), cppn_state);
        }

        Self::new(core, cppn, 1)
    }

    fn identity(core: LinkCore, state: &mut Self::State) -> Self {
        let (cppn, cppn_state) = identity_genome();

        if !state.unique_cppn_states.contains_key(&(core.from, core.to)) {
            state
                .unique_cppn_states
                .insert((core.from, core.to), cppn_state);
        }

        Self::new(core, cppn, 1)
    }

    fn clone_with(&self, core: LinkCore, state: &mut Self::State) -> Self {
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

    fn get_core(&self) -> &LinkCore {
        &self.core
    }

    fn get_core_mut(&mut self) -> &mut LinkCore {
        &mut self.core
    }

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        Self {
            core: self.core.crossover(&other.core, fitness, other_fitness),
            cppn: self.cppn.crossover(&other.cppn, fitness, other_fitness),
            depth: if rand::thread_rng().gen::<bool>() {
                self.depth
            } else {
                other.depth
            },
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        let mut distance = 0.5 * self.core.distance(&other.core);
        distance += 0.4 * self.cppn.distance(&other.cppn);
        distance += 0.1 * ((self.depth - other.depth) as f64).abs().tanh();
        distance
    }
}
