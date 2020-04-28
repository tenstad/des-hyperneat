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

use algorithm::Algorithm;
use conf::EVOLUTION;
use environment::Environment;
use evaluate::MultiEvaluator;
use log::Log;
use population::Population;

pub fn evolve<A: Algorithm, E: Environment<A::Phenotype>>() {
    let environment = &E::default();

    let init_config = A::genome_init_config(&environment.description());
    let mut population = Population::<A::Genome>::new(EVOLUTION.population_size, &init_config);

    let evaluator = MultiEvaluator::<A::Genome>::new::<A::Phenotype, A::Developer, E>(
        EVOLUTION.population_size,
        EVOLUTION.thread_count,
    );
    let mut logger = A::Logger::from(environment.description());

    for i in 1..EVOLUTION.iterations {
        population.evolve();
        population.evaluate(&evaluator);

        logger.log(i, &population);
    }
}
