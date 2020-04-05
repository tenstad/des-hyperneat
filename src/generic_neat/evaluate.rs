use crate::data::dataset;
use crate::generic_neat::genome;
use crossbeam::queue;
use std::default::Default;
use std::sync::Arc;
use std::thread;
use std::time;

type Input = (usize, usize, genome::Genome);
type Output = (usize, usize, f64);

pub trait Environment<P> {
    fn fitness(&self, phenotype: &mut P) -> f64;
    fn get_dimensions(&self) -> &dataset::Dimensions;
}

pub trait Develop<P> {
    fn develop(&self, genome: &genome::Genome) -> P;
}

pub trait Evaluate {
    fn evaluate(&self, organisms: impl Iterator<Item = Input>) -> Vec<Output>;
}

pub struct Evaluator<'a, P> {
    environment: &'a dyn Environment<P>,
    developer: &'a dyn Develop<P>,
}

impl<'a, P> Evaluator<'a, P> {
    pub fn new(
        environment: &'a dyn Environment<P>,
        developer: &'a dyn Develop<P>,
    ) -> Evaluator<'a, P> {
        Evaluator {
            environment: environment,
            developer: developer,
        }
    }
}

impl<'a, P> Evaluate for Evaluator<'a, P> {
    fn evaluate(&self, organisms: impl Iterator<Item = Input>) -> Vec<Output> {
        organisms
            .map(|(species_index, organism_index, genome)| {
                (
                    species_index,
                    organism_index,
                    self.environment
                        .fitness(&mut self.developer.develop(&genome)),
                )
            })
            .collect()
    }
}

pub struct MultiEvaluator {
    input: Arc<queue::ArrayQueue<Input>>,
    output: Arc<queue::ArrayQueue<Output>>,
}

impl MultiEvaluator {
    pub fn new<P, E: Environment<P> + Default, D: Develop<P> + Default>(
        task_count: usize,
        thread_count: usize,
    ) -> MultiEvaluator {
        let input = Arc::new(queue::ArrayQueue::new(task_count));
        let output = Arc::new(queue::ArrayQueue::new(task_count));

        for _ in 0..thread_count {
            let input = input.clone();
            let output = output.clone();

            thread::spawn(move || {
                let environment = E::default();
                let developer = D::default();

                loop {
                    if let Ok((species_index, organism_index, genome)) = input.pop() {
                        let mut result = (
                            species_index,
                            organism_index,
                            environment.fitness(&mut developer.develop(&genome)),
                        );

                        while let Err(queue::PushError(ret)) = output.push(result) {
                            result = ret;
                            thread::sleep(time::Duration::from_nanos(1000));
                        }
                    } else {
                        thread::sleep(time::Duration::from_nanos(1000));
                    }
                }
            });
        }

        MultiEvaluator { input, output }
    }
}

impl Evaluate for MultiEvaluator {
    fn evaluate(&self, organisms: impl Iterator<Item = Input>) -> Vec<Output> {
        let mut count = 0;
        for mut organism in organisms {
            while let Err(queue::PushError(ret)) = self.input.push(organism) {
                organism = ret;
                thread::sleep(time::Duration::from_nanos(1000));
            }
            count += 1;
        }

        let mut results = Vec::with_capacity(count);
        while results.len() < count {
            if let Ok(result) = self.output.pop() {
                results.push(result);
            } else {
                thread::sleep(time::Duration::from_nanos(1000));
            }
        }

        results
    }
}
