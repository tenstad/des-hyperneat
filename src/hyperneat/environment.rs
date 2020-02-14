use crate::data::dataset::Dimensions;
use crate::generic_neat::environment::Environment;
use crate::network::evaluate;

pub struct HyperneatEnvironment<'a> {
    environment: &'a dyn Environment<evaluate::Evaluator>,
}

impl<'a> HyperneatEnvironment<'a> {
    pub fn from_environment(
        environment: &'a dyn Environment<evaluate::Evaluator>,
    ) -> HyperneatEnvironment {
        HyperneatEnvironment { environment }
    }
}

impl<'a> Environment<evaluate::Evaluator> for HyperneatEnvironment<'a> {
    fn get_name(&self) -> &String {
        self.environment.get_name()
    }

    fn get_dimensions(&self) -> &Dimensions {
        return &Dimensions {
            inputs: 4,
            outputs: 1,
        };
    }

    fn fitness(&self, network: &mut evaluate::Evaluator) -> f64 {
        self.environment.fitness(network)
    }

    fn accuracy(&self, network: &mut evaluate::Evaluator) -> f64 {
        self.environment.accuracy(network)
    }
}
