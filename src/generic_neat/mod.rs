mod dot;
pub mod evaluate;
pub mod genome;
mod innovation;
pub mod link;
pub mod log;
pub mod node;
pub mod organism;
pub mod population;
mod species;

use crate::conf;

pub fn neat<
    P,
    E: evaluate::Environment<P> + Default,
    D: evaluate::Develop<P> + Default,
    L: log::Log + Default,
>() {
    let environment = &E::default();
    let developer = &D::default();
    let evaluator = &evaluate::MultiEvaluator::new::<P, E, D>(
        conf::NEAT.population_size,
        conf::GENERAL.thread_count,
    );
    let mut logger = L::default();

    let mut population = population::Population::new(
        conf::NEAT.population_size,
        environment.get_dimensions().inputs,
        environment.get_dimensions().outputs,
    );

    for i in 0..conf::NEAT.iterations {
        population.evolve();
        population.evaluate(evaluator);

        let best_organism = population.best().unwrap();
        let acc = environment.accuracy(&mut developer.develop(&best_organism.genome));

        logger.log(i+1, &population, best_organism.fitness, acc);
    }
}
