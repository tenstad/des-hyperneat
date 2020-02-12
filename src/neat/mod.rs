mod dot;
pub mod environment;
pub mod experiments;
mod genome;
pub mod nodes;
mod organism;
mod population;
mod species;

use crate::conf;
use environment::Environment;
use population::Population;

pub fn neat(environment: &dyn Environment) {
    let mut population = Population::new(environment.get_dimensions());

    for _ in 0..conf::NEAT.iterations {
        population.evolve(environment);

        let individual = population.best().unwrap();
        println!("Best fitness: {}", individual.fitness);

        let acc = environment.evaluate_accuracy(&individual.genome);

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
