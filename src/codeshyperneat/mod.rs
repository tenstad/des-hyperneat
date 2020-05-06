use crate::codeshyperneat::{developer::CombinedGenome, genome::Genome as BlueprintGenome};
use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::developer::Developer;
use evolution::{
    conf::EVOLUTION,
    environment::{Environment, NoStats},
    evaluate::{Evaluate, MultiEvaluator},
    log::{Log, Logger},
    neat::state::InitConfig,
    population::Population,
};
use network::execute::Executor;

pub mod conf;
pub mod developer;
pub mod genome;
pub mod link;
pub mod node;

pub fn codeshyperneat<E: Environment<Phenotype = Executor> + Default + 'static>() {
    let environment = &E::default();

    let mut modules =
        Population::<CppnGenome, NoStats>::new(EVOLUTION.population_size, &InitConfig::new(4, 2));
    let mut blueprints = Population::<BlueprintGenome, E::Stats>::new(
        EVOLUTION.population_size,
        &InitConfig::new(1, 1),
    );

    let evaluator = MultiEvaluator::<CombinedGenome, E>::new::<Developer>(
        EVOLUTION.population_size,
        EVOLUTION.thread_count,
    );
    let mut logger = Logger::from(environment.description());

    for i in 1..EVOLUTION.iterations {
        let mut avg_fitnesses = Vec::<f64>::new();

        let num_repeats = 5;
        for _ in 0..num_repeats {
            let mut combined_genomes = blueprints
                .enumerate()
                .map(|(species_index, organism_index, organism)| {
                    (
                        species_index,
                        organism_index,
                        CombinedGenome::new(
                            organism.genome.clone(),
                            organism.genome.select_modules(&modules),
                        ),
                    )
                })
                .collect::<Vec<(usize, usize, CombinedGenome)>>();

            let mut fitnesses = evaluator.evaluate(combined_genomes.iter().cloned());

            avg_fitnesses
                .push(fitnesses.iter().map(|(_, _, f, _)| f).sum::<f64>() / fitnesses.len() as f64);

            for ((species_index, organism_index, combined_genome), (_, _, fitness, _stats)) in
                combined_genomes.drain(..).zip(fitnesses.drain(..))
            {
                // Assign fitness to the blueprint
                *blueprints
                    .species
                    .get_mut(&species_index)
                    .unwrap()
                    .organisms[organism_index]
                    .fitness
                    .get_or_insert(0.0) += fitness / num_repeats as f64;
                // Assign fitness to the modules
                for (module_species, (module_index, _)) in combined_genome.modules.iter() {
                    if modules.extinct_species.contains_key(module_species) {
                        continue;
                    }
                    let organism = modules
                        .species
                        .get_mut(module_species)
                        .expect("unable to find module species")
                        .organisms
                        .get_mut(*module_index)
                        .expect("unable to find organism");
                    *organism.fitness.get_or_insert(0.0) += fitness;
                    // NOTE: adjusted_fitness is used as counter and will be overwritten in evolution
                    *organism.adjusted_fitness.get_or_insert(0.0) += 1.0;
                }
            }
        }

        let avg_fitness = avg_fitnesses.iter().sum::<f64>() / avg_fitnesses.len() as f64;
        for organism in modules.iter_mut() {
            if let Some(adjusted_fitness) = organism.adjusted_fitness {
                // Calculate average for modules selected by at least one blueprint
                *organism.fitness.get_or_insert(0.0) /= adjusted_fitness;
            } else {
                // Assume average fitness in all modules not chosen by any blueprint
                organism.fitness = Some(avg_fitness);
            }
        }

        logger.log(i, &blueprints);
        logger.log(i, &modules);

        blueprints.evolve();
        modules.evolve();

        blueprints.state.species = modules.next_id;
    }
}
