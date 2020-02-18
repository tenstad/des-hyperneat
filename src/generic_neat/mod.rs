mod dot;
pub mod evaluate;
pub mod genome;
mod innovation;
pub mod link;
pub mod node;
mod organism;
mod population;
mod species;

use crate::conf;

pub fn neat<P, E: evaluate::Environment<P> + Default, D: evaluate::Develop<P> + Default>() {
    run(
        &E::default(),
        &D::default(),
        &evaluate::MultiEvaluator::new::<P, E, D>(
            conf::NEAT.population_size,
            conf::GENERAL.thread_count,
        ),
    );
}

fn run<P>(
    environment: &impl evaluate::Environment<P>,
    developer: &impl evaluate::Develop<P>,
    evaluator: &impl evaluate::Evaluate,
) {
    let mut population = population::Population::new(
        conf::NEAT.population_size,
        environment.get_dimensions().inputs,
        environment.get_dimensions().outputs,
    );

    for i in 0..conf::NEAT.iterations {
        println!("Iteration: {}", i + 1);
        population.evolve();
        population.evaluate(evaluator);

        let best_organism = population.best().unwrap();
        let acc = environment.accuracy(&mut developer.develop(&best_organism.genome));
        println!("Best fitness: {}\nAcc: {}", best_organism.fitness, acc);

        dot::genome_to_dot(String::from("g.dot"), &best_organism.genome).ok();
    }
}
