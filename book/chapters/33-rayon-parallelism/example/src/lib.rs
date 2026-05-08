use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParallelPlanError {
    EmptyInput,
    InvalidPartitions,
}

pub fn partition_ranges(
    len: usize,
    partitions: usize,
) -> Result<Vec<(usize, usize)>, ParallelPlanError> {
    if len == 0 {
        return Err(ParallelPlanError::EmptyInput);
    }

    if partitions == 0 {
        return Err(ParallelPlanError::InvalidPartitions);
    }

    let actual = partitions.min(len);
    let base = len / actual;
    let remainder = len % actual;
    let mut start = 0;
    let mut ranges = Vec::with_capacity(actual);

    for index in 0..actual {
        let width = base + usize::from(index < remainder);
        let end = start + width;
        ranges.push((start, end));
        start = end;
    }

    Ok(ranges)
}

pub fn sum_single_thread(values: &[f64]) -> Result<f64, ParallelPlanError> {
    if values.is_empty() {
        return Err(ParallelPlanError::EmptyInput);
    }

    Ok(values.iter().sum())
}

pub fn sum_partitioned(values: &[f64], partitions: usize) -> Result<f64, ParallelPlanError> {
    let ranges = partition_ranges(values.len(), partitions)?;

    Ok(ranges
        .iter()
        .map(|&(start, end)| values[start..end].iter().sum::<f64>())
        .sum())
}

pub fn sum_threaded(values: &[f64], partitions: usize) -> Result<f64, ParallelPlanError> {
    let ranges = partition_ranges(values.len(), partitions)?;

    thread::scope(|scope| {
        let mut handles = Vec::with_capacity(ranges.len());

        for (start, end) in ranges {
            handles.push(scope.spawn(move || values[start..end].iter().sum::<f64>()));
        }

        Ok(handles
            .into_iter()
            .map(|handle| handle.join().expect("worker should not panic"))
            .sum())
    })
}

pub fn restore_order<T: Clone>(indexed: &[(usize, T)]) -> Vec<T> {
    let mut sorted = indexed.to_vec();
    sorted.sort_by_key(|(index, _)| *index);
    sorted.into_iter().map(|(_, value)| value).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partitions_cover_input_without_overlap() {
        assert_eq!(partition_ranges(10, 3), Ok(vec![(0, 4), (4, 7), (7, 10)]));
    }

    #[test]
    fn partitioned_sum_matches_single_thread() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0];

        assert_eq!(
            sum_partitioned(&values, 2).expect("valid input"),
            sum_single_thread(&values).expect("valid input")
        );
    }

    #[test]
    fn threaded_sum_matches_single_thread() {
        let values = (1..=10_000).map(|value| value as f64).collect::<Vec<_>>();

        assert_eq!(
            sum_threaded(&values, 8).expect("valid input"),
            sum_single_thread(&values).expect("valid input")
        );
    }

    #[test]
    fn indexed_results_can_restore_deterministic_order() {
        let unordered = vec![(2, "c"), (0, "a"), (1, "b")];

        assert_eq!(restore_order(&unordered), vec!["a", "b", "c"]);
    }
}
