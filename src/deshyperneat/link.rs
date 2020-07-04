use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::conf::DESHYPERNEAT;
use crate::deshyperneat::state::CustomState;
use crate::eshyperneat::genome::identity_genome;
use evolution::{
    genome::GenericGenome,
    neat::{
        conf::NeatConfig,
        genome::GetNeat,
        link::{LinkExtension, NeatLink},
        state::{InitConfig, NeatState},
    },
};
use rand::Rng;

#[derive(Clone, GetNeat, new)]
pub struct Link {
    #[neat]
    pub neat: NeatLink,
    pub cppn: CppnGenome,
    pub depth: u64,
}

impl LinkExtension for Link {
    type Config = NeatConfig;
    type State = CustomState;

    fn new(config: &Self::Config, neat: NeatLink, state: &mut Self::State) -> Self {
        let init_conf = InitConfig::new(4, 2);
        let mut cppn_state = NeatState::default();
        let cppn = CppnGenome::new(config, &init_conf, &mut cppn_state);

        if !state.unique_cppn_states.contains_key(&(neat.from, neat.to)) {
            state
                .unique_cppn_states
                .insert((neat.from, neat.to), cppn_state);
        }

        Self::new(neat, cppn, 1)
    }

    fn identity(config: &Self::Config, neat: NeatLink, state: &mut Self::State) -> Self {
        let (cppn, cppn_state) = if DESHYPERNEAT.enable_identity_mapping {
            identity_genome()
        } else {
            let init_conf = InitConfig::new(4, 2);
            let mut cppn_state = NeatState::default();
            let cppn = CppnGenome::new(config, &init_conf, &mut cppn_state);
            (cppn, cppn_state)
        };

        if !state.unique_cppn_states.contains_key(&(neat.from, neat.to)) {
            state
                .unique_cppn_states
                .insert((neat.from, neat.to), cppn_state);
        }

        Self::new(neat, cppn, 1)
    }

    fn clone_with(&self, _: &Self::Config, neat: NeatLink, state: &mut Self::State) -> Self {
        if !state
            .cppn_state_redirects
            .contains_key(&(neat.from, neat.to))
        {
            let key = (self.neat.from, self.neat.to);
            state.cppn_state_redirects.insert(
                (neat.from, neat.to),
                if let Some(redirect) = state.cppn_state_redirects.get(&key) {
                    *redirect
                } else {
                    key
                },
            );
        }

        Self::new(neat, self.cppn.clone(), self.depth)
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
        let mut distance = 0.5 * self.neat.distance(&other.neat);
        distance += 0.4 * self.cppn.distance(config, &other.cppn);
        distance += 0.1 * ((self.depth - other.depth) as f64).abs().tanh();
        distance
    }
}
