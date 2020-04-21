pub mod conf;
pub mod environment;
pub mod evaluate;
pub mod genome;
pub mod log;
pub mod organism;
pub mod population;
pub mod species;

#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

#[macro_use]
extern crate derive_new;

use conf::EVOLUTION;

pub fn evolve<
    G: genome::Genome + 'static,
    P, // phenotype
    D: genome::Develop<G, P> + Default,
    E: environment::Environment<P> + Default,
    L: log::Log<G> + Default,
>() {
    let environment = &E::default();
    let developer = &D::default();
    let evaluator = &evaluate::MultiEvaluator::<G>::new::<P, D, E>(
        EVOLUTION.population_size,
        EVOLUTION.thread_count,
    );
    let mut logger = L::default();

    let mut population = population::Population::<G>::new(
        EVOLUTION.population_size,
        &G::InitConfig::from(environment.description()),
    );

    for i in 0..EVOLUTION.iterations {
        population.evolve();
        population.evaluate(evaluator);

        logger.log(i + 1, &population);
    }
}
