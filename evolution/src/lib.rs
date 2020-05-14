pub mod algorithm;
pub mod conf;
pub mod develop;
pub mod environment;
pub mod evaluate;
pub mod genome;
pub mod log;
pub mod neat;
pub mod organism;
pub mod population;
pub mod species;

#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate neat_macro;

use algorithm::Algorithm;
use conf::{Conf, PopulationConfig, EVOLUTION};
use envconfig::Envconfig;
use environment::Environment;
use evaluate::MultiEvaluator;
use genome::Genome;
use log::Log;
use population::Population;
use serde::Serialize;

#[derive(new, Serialize)]
pub struct Config<T: Serialize> {
    evoltuion: Conf,
    population: PopulationConfig,
    genome: T,
}

pub fn evolve<
    E: Environment + 'static,
    A: Algorithm<E>,
    L: Log<Config<<<A as Algorithm<E>>::Genome as Genome>::Config>, A::Genome, E::Stats>,
>() {
    let environment = &E::default();
    let environment_description = environment.description();

    let population_config = PopulationConfig::init().unwrap();
    let genome_config = A::genome_config(&environment_description);
    let init_config = A::genome_init_config(&environment_description);
    let mut population = Population::<A::Genome, E::Stats>::new(
        population_config.clone(),
        genome_config.clone(),
        &init_config,
    );

    let evaluator = MultiEvaluator::<A::Genome, E>::new::<A::Developer>(
        population.population_config.population_size,
        EVOLUTION.thread_count,
    );
    let config = Config::<<<A as algorithm::Algorithm<E>>::Genome as Genome>::Config>::new(
        EVOLUTION.clone(),
        population_config,
        genome_config,
    );
    let mut logger = L::new(&environment_description, &config);

    for i in 1..EVOLUTION.iterations {
        population.evaluate(&evaluator);
        logger.log(i, &population);

        population.evolve();
    }
}
