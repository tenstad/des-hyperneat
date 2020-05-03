use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::conf::DESHYPERNEAT;
use crate::deshyperneat::genome::State;
use evolution::neat::{
    genome::{Genome, GenomeComponent},
    link::LinkCore,
    state::{InitConfig, StateCore},
};
use rand::Rng;

#[derive(Clone)]
pub struct Link {
    pub core: LinkCore,
    pub cppn: CppnGenome,
    pub depth: usize,
}

impl GenomeComponent<LinkCore, State> for Link {
    fn new(core: LinkCore, state: &mut State) -> Self {
        let init_conf = InitConfig::new(4, 2);

        let cppn = if DESHYPERNEAT.single_cppn_state {
            CppnGenome::new(&init_conf, &mut state.single_cppn_state)
        } else if let Some(cppn_state) = state.unique_cppn_states.get_mut(&(core.from, core.to)) {
            CppnGenome::new(&init_conf, cppn_state)
        } else {
            let mut cppn_state = StateCore::default();
            let cppn = CppnGenome::new(&init_conf, &mut cppn_state);
            state
                .unique_cppn_states
                .insert((core.from, core.to), cppn_state);
            cppn
        };

        Self {
            core,
            cppn,
            depth: 1,
        }
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
