use crate::codeshyperneat::{developer::CombinedGenome, genome::Genome as BlueprintGenome};
use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::developer::Developer;
use evolution::{
    conf::EVOLUTION,
    environment::Environment,
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

pub fn codeshyperneat<E: Environment<Executor> + Default>() {
    let environment = &E::default();

    let mut modules =
        Population::<CppnGenome>::new(EVOLUTION.population_size, &InitConfig::new(4, 2));
    let mut blueprints =
        Population::<BlueprintGenome>::new(EVOLUTION.population_size, &InitConfig::new(1, 1));

    let evaluator = MultiEvaluator::<CombinedGenome>::new::<Executor, Developer, E>(
        EVOLUTION.population_size,
        EVOLUTION.thread_count,
    );
    let mut logger = Logger::from(environment.description());

    for i in 1..EVOLUTION.iterations {
        blueprints.evolve();
        modules.evolve();

        blueprints.state.species = modules.next_id;

        // Reset fitness of all organisms
        // Also reset adjusted, as this is used as counter
        for organism in modules.iter_mut() {
            organism.fitness = 0.0;
            organism.adjusted_fitness = 0.0;
        }
        for organism in blueprints.iter_mut() {
            organism.fitness = 0.0;
            organism.adjusted_fitness = 0.0;
        }

        let mut avg_fitnesses = Vec::<f64>::new();

        let num_repeats = 5;
        for _ in 0..num_repeats {
            let combined_genomes = blueprints
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

            let fitnesses = evaluator.evaluate(combined_genomes.iter().cloned());
            avg_fitnesses
                .push(fitnesses.iter().map(|(_, _, f)| f).sum::<f64>() / fitnesses.len() as f64);

            for ((species_index, organism_index, combined_genome), (_, _, fitness)) in
                combined_genomes.iter().zip(fitnesses.iter())
            {
                // Assign fitness to the blueprint
                blueprints.species.get_mut(species_index).unwrap().organisms[*organism_index]
                    .fitness += *fitness;
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
                    organism.fitness += fitness;
                    organism.adjusted_fitness += 1.0;
                }
            }
        }

        let avg_fitness = avg_fitnesses.iter().sum::<f64>() / avg_fitnesses.len() as f64;
        for organism in modules.iter_mut() {
            if organism.adjusted_fitness == 0.0 {
                // Assume average fitness in all modules not chosen by any blueprint
                organism.fitness = avg_fitness;
            } else {
                // Calculate averate for modules selected by at least one blueprint
                organism.fitness /= organism.adjusted_fitness;
            }
        }
        for organism in blueprints.iter_mut() {
            organism.fitness /= num_repeats as f64;
        }

        logger.log(i, &blueprints);
        logger.log(i, &modules);
    }
}
