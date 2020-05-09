use crate::develop::Develop;
use crate::environment::Environment;
use crossbeam::queue;
use std::{sync::Arc, thread, time};

type Input<G> = (usize, usize, G);
type Output<S> = (usize, usize, f64, S);

pub trait Evaluate<G, S> {
    fn evaluate(&self, organisms: impl Iterator<Item = Input<G>>) -> Vec<Output<S>>;
}

pub struct MultiEvaluator<G, E: Environment> {
    input: Arc<queue::ArrayQueue<Input<G>>>,
    output: Arc<queue::ArrayQueue<Output<E::Stats>>>,
}

impl<G: Send + 'static, E: Environment + 'static> MultiEvaluator<G, E> {
    pub fn new<D: Develop<G, E::Phenotype>>(task_count: usize, thread_count: usize) -> Self {
        let input = Arc::new(queue::ArrayQueue::new(task_count));
        let output = Arc::new(queue::ArrayQueue::new(task_count));

        for _ in 0..thread_count {
            let input = input.clone();
            let output = output.clone();

            thread::spawn(move || {
                let environment = E::default();
                let developer = D::from(environment.description());

                loop {
                    if let Ok((species_index, organism_index, genome)) = input.pop() {
                        let (fitness, stats) = environment.evaluate(&mut developer.develop(genome));
                        let mut result = (species_index, organism_index, fitness, stats);

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

impl<G, E: Environment> Evaluate<G, E::Stats> for MultiEvaluator<G, E> {
    fn evaluate(&self, organisms: impl Iterator<Item = Input<G>>) -> Vec<Output<E::Stats>> {
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
