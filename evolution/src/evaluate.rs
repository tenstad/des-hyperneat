use crate::environment::Environment;
use crate::genome::{Develop, Genome};
use crossbeam::queue;
use std::sync::Arc;
use std::thread;
use std::time;

type Input<G> = (usize, usize, G);
type Output = (usize, usize, f64);

pub trait Evaluate<G> {
    fn evaluate(&self, organisms: impl Iterator<Item = Input<G>>) -> Vec<Output>;
}

#[derive(new)]
pub struct Evaluator<'a, G, P> {
    environment: &'a dyn Environment<P>,
    developer: &'a dyn Develop<G, P>,
}

impl<'a, G: Genome, P> Evaluate<G> for Evaluator<'a, G, P> {
    fn evaluate(&self, organisms: impl Iterator<Item = Input<G>>) -> Vec<Output> {
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

pub struct MultiEvaluator<G: Genome> {
    input: Arc<queue::ArrayQueue<Input<G>>>,
    output: Arc<queue::ArrayQueue<Output>>,
}

impl<G: Genome + 'static> MultiEvaluator<G> {
    pub fn new<P, D: Develop<G, P> + Default, E: Environment<P> + Default>(
        task_count: usize,
        thread_count: usize,
    ) -> Self {
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

impl<G: Genome> Evaluate<G> for MultiEvaluator<G> {
    fn evaluate(&self, organisms: impl Iterator<Item = Input<G>>) -> Vec<Output> {
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
