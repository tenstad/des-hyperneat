pub mod conf;
pub mod develop;
pub mod genome;
pub mod link;
pub mod node;
pub mod state;

extern crate num_cpus;

use crate::codeshyperneat::{
    conf::CODESHYPERNEAT, develop::CombinedGenome, genome::Genome as BlueprintGenome,
};
use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::developer::{topology_init_config, Developer};
use conf::MethodConfig;
use envconfig::Envconfig;
use evolution::{
    conf::{EvolutionConfig, PopulationConfig, EVOLUTION},
    develop::Develop,
    environment::Environment,
    evaluate::{Evaluate, MultiEvaluator},
    log::{Log, Logger},
    neat::{conf::NeatConfig, state::InitConfig},
    population::Population,
    stats::{NoStats, OrganismStats, PopulationStats},
};
use network::execute::Executor;
use serde::Serialize;
use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
    u64,
};

#[derive(new, Serialize)]
struct Config<N: Serialize, E: Serialize> {
    evolution: EvolutionConfig,
    population: PopulationConfig,
    genome: NeatConfig,
    module_population: PopulationConfig,
    module_genome: NeatConfig,
    method: MethodConfig,
    environment: E,
    main: N,
}

pub fn codeshyperneat<
    E: Environment<Phenotype = Executor> + Default + 'static,
    N: Serialize + Default,
>() {
    let environment = &E::default();

    let module_population_config = PopulationConfig::init().unwrap();
    let module_genome_config = NeatConfig::default();
    let mut modules = Population::<CppnGenome>::new(
        module_population_config.clone(),
        module_genome_config.clone(),
        &InitConfig::new(4, 2),
    );

    let blueprint_population_config = PopulationConfig::init().unwrap();
    let blueprint_genome_config = NeatConfig::default();
    let blueprint_genome_init = topology_init_config(&environment.description());
    let mut blueprints = Population::<BlueprintGenome>::new(
        blueprint_population_config.clone(),
        blueprint_genome_config.clone(),
        &blueprint_genome_init,
    );

    let evaluator = MultiEvaluator::<CombinedGenome, Developer, E>::new(
        blueprints.population_config.population_size,
        if EVOLUTION.thread_count > 0 {
            EVOLUTION.thread_count
        } else {
            num_cpus::get() as u64
        },
    );
    let config = Config::new(
        EVOLUTION.clone(),
        blueprint_population_config,
        blueprint_genome_config,
        module_population_config,
        module_genome_config,
        CODESHYPERNEAT.clone(),
        E::Config::default(),
        N::default(),
    );
    let mut logger = <Logger as Log<BlueprintGenome>>::new(&environment.description(), &config);

    for _ in 0..EVOLUTION.initial_mutations {
        modules.mutate();
        blueprints.mutate();
    }

    let iterations = if EVOLUTION.iterations > 0 {
        EVOLUTION.iterations + 1
    } else {
        u64::MAX
    };

    let start_time = SystemTime::now();
    for i in 0..iterations {
        let mut avg_fitnesses = Vec::<f64>::new();

        let mut stats = HashMap::<
            (u64, usize),
            OrganismStats<
                Vec<NoStats>,
                Vec<<Developer as Develop<CombinedGenome>>::Stats>,
                Vec<E::Stats>,
            >,
        >::new();

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

            avg_fitnesses.push(
                fitnesses.iter().map(|(_, _, f, _, _)| f).sum::<f64>() / fitnesses.len() as f64,
            );

            for (
                (species_index, organism_index, combined_genome),
                (_, _, fitness, phenotype_stats, evaluation_stats),
            ) in combined_genomes.drain(..).zip(fitnesses.drain(..))
            {
                if let Some(mut organism_stats) = stats.get_mut(&(species_index, organism_index)) {
                    organism_stats.fitness += fitness;
                    organism_stats.genome.push(NoStats {});
                    organism_stats.phenotype.push(phenotype_stats);
                    organism_stats.evaluation.push(evaluation_stats);
                } else {
                    stats.insert(
                        (species_index, organism_index),
                        OrganismStats::new(
                            fitness,
                            vec![NoStats {}],
                            vec![phenotype_stats],
                            vec![evaluation_stats],
                        ),
                    );
                }

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

        for mut organism_stats in stats.values_mut() {
            organism_stats.fitness =
                organism_stats.fitness / CODESHYPERNEAT.blueprint_developments as f64;
        }
        let stats = PopulationStats::new(stats.drain().map(|(_, v)| v).collect::<Vec<_>>());

        logger.log(i, &blueprints, &stats);
        // logger.log(i, &modules);

        if EVOLUTION.seconds_limit > 0
            && SystemTime::elapsed(&start_time).unwrap()
                >= Duration::from_secs(EVOLUTION.seconds_limit + 3)
        {
            break;
        }

        blueprints.evolve();
        modules.evolve();

        blueprints.state.custom.species = modules.next_id;
    }
    <Logger as Log<BlueprintGenome>>::close(&mut logger);
}
