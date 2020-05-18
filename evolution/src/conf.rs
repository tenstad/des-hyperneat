use bson;
use envconfig::Envconfig;
use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Serialize, Clone, Default)]
pub struct NoConfig {}

#[derive(Envconfig, Serialize, Clone)]
pub struct EvolutionConfig {
    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "THREADS", default = "0")]
    pub thread_count: u64,

    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "ITERATIONS", default = "10000")]
    pub iterations: u64,

    #[envconfig(from = "DB_LOG", default = "false")]
    pub db_log: bool,
}

#[derive(Envconfig, Serialize, Clone)]
pub struct PopulationConfig {
    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "POPULATION_SIZE", default = "300")]
    pub population_size: u64,

    #[envconfig(from = "SPECIATION_THRESHOLD", default = "0.7")]
    pub speciation_threshold: f64,

    #[envconfig(from = "ASEXUAL_REPRODUCTION_PROBABILITY", default = "0.2")]
    pub asexual_reproduction_probability: f64,

    #[envconfig(from = "INTERSPECIES_REPRODUCTION_PROBABILITY", default = "0.15")]
    pub interspecies_reproduction_probability: f64,

    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "TOURNAMENT_SIZE", default = "2")]
    pub tournament_size: u64,

    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "INTERSPECIES_TOURNAMENT_SIZE", default = "2")]
    pub interspecies_tournament_size: u64,

    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "DROPOFF_AGE", default = "30")]
    pub dropoff_age: u64,

    #[envconfig(from = "YOUNG_SPECIES_FITNESS_MULTIPLIER", default = "1.05")]
    pub young_species_fitness_multiplier: f64,

    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "YOUNG_AGE_LIMIT", default = "20")]
    pub young_age_limit: u64,

    #[envconfig(from = "STAGNENT_SPECIES_FITNESS_MULTIPLIER", default = "0.2")]
    pub stagnent_species_fitness_multiplier: f64,

    #[envconfig(from = "SURVIVAL_RATO", default = "0.4")]
    pub survival_ratio: f64,

    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "GLOBAL_ELITES", default = "1")]
    pub global_elites: u64,

    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "GUARANTEED_ELITES", default = "0")]
    pub guaranteed_elites: u64,

    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "ELITES_FROM_OFFSPRING", default = "1")]
    pub elites_from_offspring: u64,
}

#[derive(new, Serialize)]
pub struct CombinedConfig<G: Serialize, M: Serialize, E: Serialize, C: Serialize> {
    evolution: EvolutionConfig,
    population: PopulationConfig,
    genome: G,
    method: M,
    environment: E,
    main: C,
}

lazy_static! {
    pub static ref EVOLUTION: EvolutionConfig = EvolutionConfig::init().unwrap();
}
