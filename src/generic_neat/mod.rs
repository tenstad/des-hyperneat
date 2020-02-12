pub mod default;
pub mod dataset_environment;
mod dot;
pub mod environment;
pub mod genome;
mod innovation;
pub mod link;
pub mod node;
mod organism;
mod population;
mod species;

use crate::conf;
use environment::Environment;
use population::Population;

pub fn neat<I: node::Custom, H: node::Custom, O: node::Custom, L: link::Custom>(
    environment: &dyn Environment<I, H, O, L>,
) {
    let mut population = Population::new(environment.get_dimensions());

    for _ in 0..conf::NEAT.iterations {
        population.evolve(environment);

        let individual = population.best().unwrap();
        println!("Best fitness: {}", individual.fitness);

        let acc = environment.accuracy(&individual.genome);

        println!("Acc: {}", acc);

        /*if acc == 1.0 {
            println!("Success!");
            break;
        }*/

        dot::genome_to_dot(String::from("g.dot"), &individual.genome).ok();
    }

    let individual = population.best().unwrap();
    println!("Best fitness: {}", individual.fitness);

    dot::genome_to_dot(String::from("g.dot"), &individual.genome).ok();
}
