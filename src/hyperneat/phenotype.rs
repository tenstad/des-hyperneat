use crate::conf;
use crate::generic_neat::evaluate;
use crate::generic_neat::genome;
use crate::hyperneat::substrate;
use crate::neat::phenotype::Developer as NeatDeveloper;
use crate::network::execute;
use crate::network::activation;
use crate::network::execute::Executor as P;

pub struct Developer {
    neat_developer: NeatDeveloper,
    network: substrate::Network,
}

impl Developer {
    pub fn create(network: substrate::Network) -> Self {
        Developer {
            neat_developer: NeatDeveloper::default(),
            network,
        }
    }
}

impl Default for Developer {
    fn default() -> Self {
        let network = substrate::Network::layered(vec![13, 16, 8, 3]);

        Developer::create(network)
    }
}

impl evaluate::Develop<P> for Developer {
    fn develop(&self, genome: &genome::Genome) -> P {
        let mut neat_executor = self.neat_developer.develop(genome);

        execute::Executor::create(
            self.network.length,
            self.network.inputs.iter().cloned().collect(),
            self.network.outputs.iter().cloned().collect(),
            self.network
                .actions
                .iter()
                .map(|action| match action {
                    substrate::Action::Activation(node, x, y) => execute::Action::Activation(
                        *node,
                        neat_executor.execute(&vec![0.0, 0.0, *x, *y])[1],
                        if self.network.inputs.contains(node) {
                            activation::Activation::None
                        } else if self.network.outputs.contains(node) {
                            conf::HYPERNEAT.output_activation
                        } else {
                            conf::HYPERNEAT.hidden_activation
                        },
                    ),
                    substrate::Action::Link(from, to, x0, y0, x1, y1) => execute::Action::Link(
                        *from,
                        *to,
                        neat_executor.execute(&vec![*x0, *y0, *x1, *y1])[0],
                    ),
                })
                .filter(|action| match action {
                    execute::Action::Activation(_, _, _) => true,
                    execute::Action::Link(_, _, weight) => weight.abs() > conf::HYPERNEAT.weight_threshold,
                })
                .collect(),
        )
    }
}
