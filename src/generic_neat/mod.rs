pub mod default;
mod dot;
pub mod environment;
pub mod genome;
mod innovation;
pub mod link;
pub mod node;
mod organism;
pub mod phenotype;
mod population;
mod species;

use crate::conf;
use environment::Environment;
use phenotype::Develop;
use population::Population;

pub fn neat<I: node::Custom, H: node::Custom, O: node::Custom, L: link::Custom, P>(
    environment: &dyn Environment<P>,
    developer: &dyn Develop<I, H, O, L, P>,
) {
    let mut population = Population::new(environment.get_dimensions());

    for _ in 0..conf::NEAT.iterations {
        population.evolve();
        population.evaluate(environment, developer);

        let best_organism = population.best().unwrap();
        let acc = environment.accuracy(&mut developer.develop(&best_organism.genome));
        println!("Best fitness: {}\nAcc: {}", best_organism.fitness, acc);

        dot::genome_to_dot(String::from("g.dot"), &best_organism.genome).ok();
    }
}
