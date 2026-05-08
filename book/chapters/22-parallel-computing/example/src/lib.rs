use std::thread;

pub fn sum_assets_single_thread(assets: &[Vec<f64>]) -> Vec<f64> {
    assets
        .iter()
        .map(|values| values.iter().sum::<f64>())
        .collect()
}

pub fn sum_assets_parallel(assets: Vec<Vec<f64>>) -> Vec<f64> {
    assets
        .into_iter()
        .map(|values| thread::spawn(move || values.iter().sum::<f64>()))
        .collect::<Vec<_>>()
        .into_iter()
        .map(|handle| handle.join().expect("worker should not panic"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_matches_single_thread() {
        let assets = vec![vec![1.0, 2.0], vec![10.0, 20.0]];

        assert_eq!(
            sum_assets_parallel(assets.clone()),
            sum_assets_single_thread(&assets)
        );
    }
}
