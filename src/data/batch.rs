pub fn create_batches(thread_count: usize, size: usize) -> Vec<(usize, usize)> {
    let batch_size = ((size as f64) / thread_count as f64).floor() as usize;

    let mut batch_sizes = vec![batch_size; thread_count];
    let excess = size - thread_count * batch_size;
    for i in 0..excess {
        batch_sizes[i] += 1;
    }

    let mut batch_starts = vec![0; thread_count];
    for i in 1..thread_count {
        batch_starts[i] += batch_starts[i - 1] + batch_sizes[i - 1];
    }
    
    batch_starts
        .iter()
        .cloned()
        .zip(batch_sizes.iter().cloned())
        .collect::<Vec<(usize, usize)>>()
}
