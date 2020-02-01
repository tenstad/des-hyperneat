mod dot;
mod genome;
mod nodes;
mod organism;
mod population;
mod species;

use crate::conf;
use population::Population;

pub fn neat() {
    let mut population = Population::new(4, 2);

    for _ in 0..conf::NEAT.iterations {
        population.evolve();
    }

    let individual = population.best().unwrap();
    
    dot::genome_to_dot(String::from("g.dot"), &individual.genome).ok();
}
