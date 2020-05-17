use crate::develop::Develop;
use crate::environment::Environment;
use crate::stats::Stats;
use crossbeam::queue;
use std::{sync::Arc, thread, time};

type Input<G> = (u64, usize, G);
type Output<P, E> = (u64, usize, f64, P, E);

pub trait Evaluate<G> {
    type PhenotypeStats: Stats;
    type EvaluationStats: Stats;

    fn evaluate(
        &self,
        organisms: impl Iterator<Item = Input<G>>,
    ) -> Vec<Output<Self::PhenotypeStats, Self::EvaluationStats>>;
}

pub struct MultiEvaluator<G, D: Develop<G>, E: Environment> {
    input: Arc<queue::ArrayQueue<Input<G>>>,
    output: Arc<queue::ArrayQueue<Output<D::Stats, E::Stats>>>,
}

impl<
        G: Send + 'static,
        D: Develop<G, Phenotype = E::Phenotype> + 'static,
        E: Environment + 'static,
    > MultiEvaluator<G, D, E>
{
    pub fn new(task_count: u64, thread_count: u64) -> Self {
        let input = Arc::new(queue::ArrayQueue::new(task_count as usize));
        let output = Arc::new(queue::ArrayQueue::new(task_count as usize));

        for _ in 0..thread_count {
            let input = input.clone();
            let output = output.clone();

            thread::spawn(move || {
                let environment = E::default();
                let developer = D::from(environment.description());

                loop {
                    if let Ok((species_index, organism_index, genome)) = input.pop() {
                        let (mut phenotype, phenotype_stats) = developer.develop(genome);
                        let (fitness, evaluation_stats) = environment.evaluate(&mut phenotype);
                        let mut result = (
                            species_index,
                            organism_index,
                            fitness,
                            phenotype_stats,
                            evaluation_stats,
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

impl<G, E: Environment, D: Develop<G, Phenotype = E::Phenotype>> Evaluate<G>
    for MultiEvaluator<G, D, E>
{
    type PhenotypeStats = D::Stats;
    type EvaluationStats = E::Stats;

    fn evaluate(
        &self,
        organisms: impl Iterator<Item = Input<G>>,
    ) -> Vec<Output<Self::PhenotypeStats, Self::EvaluationStats>> {
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
