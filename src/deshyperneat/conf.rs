use envconfig::Envconfig;
use evolution::neat::conf::{ConfigProvider, NeatConfig};
use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Envconfig, Serialize)]
pub struct MethodConfig {
    #[envconfig(from = "SINGLE_CPPN_STATE", default = "false")]
    pub single_cppn_state: bool,

    #[envconfig(from = "INPUT_CONFIG", default = "line")]
    pub input_config: String,

    #[envconfig(from = "OUTPUT_CONFIG", default = "line")]
    pub output_config: String,

    #[envconfig(from = "MUTATE_NODE_DEPTH_PROBABILITY", default = "0.1")]
    pub mutate_node_depth_probability: f64,

    #[envconfig(from = "MUTATE_ALL_COMPONENTS", default = "true")]
    pub mutate_all_components: bool,

    #[envconfig(from = "LOG_VISUALIZATIONS", default = "false")]
    pub log_visualizations: bool,

    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "MAX_INPUT_SUBSTRATE_DEPTH", default = "0")]
    pub max_input_substrate_depth: u64,

    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "MAX_OUTPUT_SUBSTRATE_DEPTH", default = "0")]
    pub max_output_substrate_depth: u64,

    #[serde(with = "bson::compat::u2f")]
    #[envconfig(from = "MAX_HIDDEN_SUBSTRATE_DEPTH", default = "5")]
    pub max_hidden_substrate_depth: u64,

    #[envconfig(from = "ENABLE_IDENTITY_MAPPING", default = "true")]
    pub enable_identity_mapping: bool,

    #[envconfig(from = "STATIC_SUBSTRATE_DEPTH", default = "-1")]
    pub static_substrate_depth: i64,
}

#[derive(Envconfig, Clone, Serialize)]
pub struct TopologyConfig {
    #[envconfig(from = "TOPOLOGY_ADD_NODE_PROBABILITY", default = "0.03")]
    pub add_node_probability: f64,

    #[envconfig(from = "TOPOLOGY_ADD_LINK_PROBABILITY", default = "0.2")]
    pub add_link_probability: f64,

    #[envconfig(from = "TOPOLOGY_INITIAL_LINK_WEIGHT_SIZE", default = "0.5")]
    pub initial_link_weight_size: f64,

    #[envconfig(from = "TOPOLOGY_MUTATE_LINK_WEIGHT_PROBABILITY", default = "0.9")]
    pub mutate_link_weight_probability: f64,

    #[envconfig(from = "TOPOLOGY_MUTATE_LINK_WEIGHT_SIZE", default = "0.5")]
    pub mutate_link_weight_size: f64,

    #[envconfig(from = "TOPOLOGY_REMOVE_NODE_PROBABILITY", default = "0.006")]
    pub remove_node_probability: f64,

    #[envconfig(from = "TOPOLOGY_REMOVE_LINK_PROBABILITY", default = "0.08")]
    pub remove_link_probability: f64,

    #[envconfig(from = "TOPOLOGY_ONLY_HIDDEN_NODE_DISTANCE", default = "true")]
    pub only_hidden_node_distance: bool,

    #[envconfig(from = "TOPOLOGY_LINK_DISTANCE_WEIGHT", default = "0.5")]
    pub link_distance_weight: f64,

    #[envconfig(from = "TOPOLOGY_MUTATE_ONLY_ONE_LINK", default = "true")]
    pub mutate_only_one_link: bool,
}

impl Default for MethodConfig {
    fn default() -> Self {
        MethodConfig::init().unwrap()
    }
}

impl Default for TopologyConfig {
    fn default() -> Self {
        TopologyConfig::init().unwrap()
    }
}

lazy_static! {
    pub static ref DESHYPERNEAT: MethodConfig = MethodConfig::default();
}

#[derive(Clone, Serialize)]
pub struct GenomeConfig {
    pub cppn: NeatConfig,
    pub topology: NeatConfig,
}

impl Default for GenomeConfig {
    fn default() -> GenomeConfig {
        let cppn = NeatConfig::default();
        let mut topology = NeatConfig::default();
        let topology_conf = TopologyConfig::default();

        topology.add_node_probability = topology_conf.add_node_probability;
        topology.add_link_probability = topology_conf.add_link_probability;
        topology.initial_link_weight_size = topology_conf.initial_link_weight_size;
        topology.mutate_link_weight_probability = topology_conf.mutate_link_weight_probability;
        topology.mutate_link_weight_size = topology_conf.mutate_link_weight_size;
        topology.remove_node_probability = topology_conf.remove_node_probability;
        topology.remove_link_probability = topology_conf.remove_link_probability;
        topology.only_hidden_node_distance = topology_conf.only_hidden_node_distance;
        topology.link_distance_weight = topology_conf.link_distance_weight;
        topology.mutate_only_one_link = topology_conf.mutate_only_one_link;

        GenomeConfig { cppn, topology }
    }
}

impl ConfigProvider<NeatConfig, NeatConfig> for GenomeConfig {
    fn neat(&self) -> &NeatConfig {
        &self.topology
    }
    fn neat_node(&self) -> &NeatConfig {
        &self.cppn
    }
    fn neat_link(&self) -> &NeatConfig {
        &self.cppn
    }
}
