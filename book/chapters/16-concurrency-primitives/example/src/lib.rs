use std::sync::{Arc, Mutex, mpsc};
use std::thread;

pub fn sum_on_threads(chunks: Vec<Vec<f64>>) -> f64 {
    let total = Arc::new(Mutex::new(0.0));
    let mut handles = Vec::new();

    for chunk in chunks {
        let total = Arc::clone(&total);
        handles.push(thread::spawn(move || {
            let partial = chunk.iter().sum::<f64>();
            let mut guard = total.lock().expect("mutex is not poisoned");
            *guard += partial;
        }));
    }

    for handle in handles {
        handle.join().expect("worker should not panic");
    }

    *total.lock().expect("mutex is not poisoned")
}

pub fn worker_results(chunks: Vec<Vec<f64>>) -> Vec<f64> {
    let (sender, receiver) = mpsc::channel();

    for chunk in chunks {
        let sender = sender.clone();
        thread::spawn(move || {
            sender
                .send(chunk.iter().sum::<f64>())
                .expect("receiver should be alive");
        });
    }

    drop(sender);
    receiver.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_mutex_allows_shared_mutation_with_explicit_locking() {
        assert_eq!(sum_on_threads(vec![vec![1.0, 2.0], vec![3.0]]), 6.0);
    }

    #[test]
    fn channels_transfer_ownership_of_results() {
        let mut results = worker_results(vec![vec![1.0], vec![2.0, 3.0]]);
        results.sort_by(f64::total_cmp);

        assert_eq!(results, vec![1.0, 5.0]);
    }
}
