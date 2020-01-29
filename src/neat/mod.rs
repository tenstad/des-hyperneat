mod dot;
mod genome;
mod individual;
mod nodes;
mod population;
mod species;

use population::Population;

pub fn neat() {
    let mut population = Population::new(50, 0.5, 4, 2);
    
    for _ in 0..100 {
        population.evolve();
    }

    let individual = population.best().unwrap();
    dot::genome_to_dot(String::from("g.dot"), &individual.genome);
}
