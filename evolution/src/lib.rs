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
pub mod stats;

#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate neat_macro;

extern crate num_cpus;

use algorithm::Algorithm;
use conf::{CombinedConfig, PopulationConfig, EVOLUTION};
use envconfig::Envconfig;
use environment::Environment;
use evaluate::MultiEvaluator;
use log::Log;
use population::Population;
use serde::Serialize;
use std::{
    time::{Duration, SystemTime},
    u64,
};

pub fn evolve<
    E: Environment + 'static,
    A: Algorithm<E>,
    L: Log<A::Genome>,
    C: Serialize + Default,
>() {
    let environment = &E::default();
    let environment_description = environment.description();

    let population_config = PopulationConfig::init().unwrap();
    let genome_config = A::genome_config(&environment_description);
    let init_config = A::genome_init_config(&environment_description);
    let mut population = Population::<A::Genome>::new(
        population_config.clone(),
        genome_config.clone(),
        &init_config,
    );

    let evaluator = MultiEvaluator::<A::Genome, A::Developer, E>::new(
        population.population_config.population_size,
        if EVOLUTION.thread_count > 0 {
            EVOLUTION.thread_count
        } else {
            num_cpus::get() as u64
        },
    );
    let config = CombinedConfig::new(
        EVOLUTION.clone(),
        population_config,
        genome_config,
        A::Config::default(),
        E::Config::default(),
        C::default(),
    );
    let mut logger = L::new(&environment_description, &config);

    let iterations = if EVOLUTION.iterations > 0 {
        EVOLUTION.iterations + 1
    } else {
        u64::MAX
    };

    for _ in 0..EVOLUTION.initial_mutations {
        population.mutate();
    }

    let start_time = SystemTime::now();
    for i in 0..iterations {
        let population_stats = population.evaluate(&evaluator);
        logger.log(i, &population, &population_stats);

        if EVOLUTION.seconds_limit > 0
            && SystemTime::elapsed(&start_time).unwrap()
                >= Duration::from_secs(EVOLUTION.seconds_limit + 3)
        {
            break;
        }

        population.evolve();
    }
    logger.close();
}
