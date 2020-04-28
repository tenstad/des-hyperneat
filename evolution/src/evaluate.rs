use crate::develop::Develop;
use crate::environment::Environment;
use crossbeam::queue;
use std::sync::Arc;
use std::thread;
use std::time;

type Input<G> = (usize, usize, G);
type Output = (usize, usize, f64);

pub trait Evaluate<G> {
    fn evaluate(&self, organisms: impl Iterator<Item = Input<G>>) -> Vec<Output>;
}

/*pub struct Evaluator<G: Genome, P, D: Develop<G, P>, E: Environment<P>> {
    environment: E,
    developer: D,
}

impl<G: Genome, P, D: Develop<G, P>, E: Environment<P>> Default for Evaluator<G, P, D, E> {
    fn default() -> Self {
        let environment = E::default();
        Self {
            environment,
            developer: D::from(environment.description()),
        }
    }
}

impl<G: Genome, P, D: Develop<G, P>, E: Environment<P>> Evaluate<G> for Evaluator<G, P, D, E> {
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
}*/

pub struct MultiEvaluator<G> {
    input: Arc<queue::ArrayQueue<Input<G>>>,
    output: Arc<queue::ArrayQueue<Output>>,
}

impl<G: Send + 'static> MultiEvaluator<G> {
    pub fn new<P, D: Develop<G, P>, E: Environment<P>>(
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
                let developer = D::from(environment.description());

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

impl<G> Evaluate<G> for MultiEvaluator<G> {
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
