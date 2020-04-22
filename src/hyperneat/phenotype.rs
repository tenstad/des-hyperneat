use crate::hyperneat::conf::HYPERNEAT;
use crate::hyperneat::substrate;
use crate::neat::genome::{Genome as NeatGenome, InitConfig};
use crate::neat::phenotype::Developer as NeatDeveloper;
use evolution::environment::EnvironmentDescription;
use evolution::genome::Develop;
use network::activation;
use network::execute::{self, Executor};

pub struct Developer {
    neat_developer: NeatDeveloper,
    init_config: InitConfig,
    network: substrate::Network,
}

impl From<EnvironmentDescription> for Developer {
    fn from(description: EnvironmentDescription) -> Self {
        Developer {
            neat_developer: NeatDeveloper::from(description),
            init_config: InitConfig::new(4, 2),
            network: substrate::Network::layered(vec![description.inputs, 8, description.outputs]),
        }
    }
}

impl Develop<NeatGenome, Executor> for Developer {
    fn init_config(&self) -> &InitConfig {
        &self.init_config
    }

    fn develop(&self, genome: &NeatGenome) -> Executor {
        let mut neat_executor = self.neat_developer.develop(genome);

        execute::Executor::create(
            self.network.length,
            self.network.inputs.iter().cloned().collect(),
            self.network.outputs.iter().cloned().collect(),
            self.network
                .actions
                .iter()
                .filter_map(|action| match action {
                    substrate::Action::Activation(node, x, y) => Some(execute::Action::Activation(
                        *node,
                        neat_executor.execute(&vec![0.0, 0.0, *x, *y])[1],
                        if self.network.inputs.contains(node) {
                            activation::Activation::None
                        } else if self.network.outputs.contains(node) {
                            HYPERNEAT.output_activation
                        } else {
                            HYPERNEAT.hidden_activation
                        },
                    )),
                    substrate::Action::Link(from, to, x0, y0, x1, y1) => {
                        let weight = neat_executor.execute(&vec![*x0, *y0, *x1, *y1])[0];
                        if weight.abs() > HYPERNEAT.weight_threshold {
                            Some(execute::Action::Link(*from, *to, weight))
                        } else {
                            None
                        }
                    }
                })
                .collect(),
        )
    }
}
