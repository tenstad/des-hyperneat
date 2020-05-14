pub mod conf;
pub mod develop;
pub mod genome;
pub mod link;
pub mod node;
pub mod state;

use crate::codeshyperneat::{
    conf::CODESHYPERNEAT, develop::CombinedGenome, genome::Genome as BlueprintGenome,
};
use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::developer::Developer;
use envconfig::Envconfig;
use evolution::{
    conf::{PopulationConfig, EVOLUTION},
    environment::{Environment, NoStats},
    evaluate::{Evaluate, MultiEvaluator},
    log::{CreateLog, LogEntry, Logger},
    neat::{conf::NeatConfig, state::InitConfig},
    population::Population,
};
use network::execute::Executor;
use serde::Serialize;

#[derive(new, Serialize)]
struct Config {
    blueprint_population: PopulationConfig,
    blueprint_genome: NeatConfig,
    module_population: PopulationConfig,
    module_genome: NeatConfig,
}

pub fn codeshyperneat<E: Environment<Phenotype = Executor> + Default + 'static>() {
    let environment = &E::default();

    let module_population_config = PopulationConfig::init().unwrap();
    let module_genome_config = NeatConfig::default();
    let mut modules = Population::<CppnGenome, NoStats>::new(
        module_population_config.clone(),
        module_genome_config.clone(),
        &InitConfig::new(4, 2),
    );

    let blueprint_population_config = PopulationConfig::init().unwrap();
    let blueprint_genome_config = NeatConfig::default();
    let mut blueprints = Population::<BlueprintGenome, E::Stats>::new(
        blueprint_population_config.clone(),
        blueprint_genome_config.clone(),
        &InitConfig::new(1, 1),
    );

    let evaluator = MultiEvaluator::<CombinedGenome, E>::new::<Developer>(
        blueprints.population_config.population_size,
        EVOLUTION.thread_count,
    );
    let config = Config::new(
        blueprint_population_config,
        blueprint_genome_config,
        module_population_config,
        module_genome_config,
    );
    let mut logger = Logger::new(&environment.description(), &config);

    for i in 1..EVOLUTION.iterations {
        let mut avg_fitnesses = Vec::<f64>::new();

        for _ in 0..CODESHYPERNEAT.blueprint_developments {
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
                .collect::<Vec<(u64, usize, CombinedGenome)>>();

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
                    .get_or_insert(0.0) += fitness / CODESHYPERNEAT.blueprint_developments as f64;
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

        blueprints.state.custom.species = modules.next_id;
    }
}
