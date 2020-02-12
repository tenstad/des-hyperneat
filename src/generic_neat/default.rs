use crate::generic_neat::link;
use crate::generic_neat::node;

#[derive(Copy, Clone)]
pub struct Input {}

#[derive(Copy, Clone)]
pub struct Hidden {}

#[derive(Copy, Clone)]
pub struct Output {}

#[derive(Copy, Clone)]
pub struct Link {}

impl link::Custom for Link {
    fn new() -> Self {
        Self {}
    }

    fn crossover(&self, _: &Self) -> Self {
        Self {}
    }
}

impl node::Custom for Input {
    fn new() -> Self {
        Self {}
    }

    fn crossover(&self, _: &Self) -> Self {
        Self {}
    }
}

impl node::Custom for Hidden {
    fn new() -> Self {
        Self {}
    }

    fn crossover(&self, _: &Self) -> Self {
        Self {}
    }
}

impl node::Custom for Output {
    fn new() -> Self {
        Self {}
    }

    fn crossover(&self, _: &Self) -> Self {
        Self {}
    }
}
