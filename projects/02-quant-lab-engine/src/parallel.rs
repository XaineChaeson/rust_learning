use std::thread;

use crate::{EngineError, ExperimentGrid, ExperimentResult, MarketSeries, run_experiment};

pub fn partition_ranges(len: usize, partitions: usize) -> Result<Vec<(usize, usize)>, EngineError> {
    if len == 0 {
        return Err(EngineError::EmptyInput {
            context: "parallel input",
        });
    }

    if partitions == 0 {
        return Err(EngineError::InvalidConfig {
            context: "parallel workers must be greater than zero",
        });
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

pub fn run_grid_parallel(
    series: &MarketSeries,
    grid: &ExperimentGrid,
    workers: usize,
) -> Result<Vec<ExperimentResult>, EngineError> {
    let configs = grid.expand()?;
    let ranges = partition_ranges(configs.len(), workers)?;

    thread::scope(|scope| {
        let mut handles = Vec::with_capacity(ranges.len());

        for (start, end) in ranges {
            let configs = &configs;
            handles.push(scope.spawn(move || {
                configs[start..end]
                    .iter()
                    .enumerate()
                    .map(|(offset, config)| (start + offset, run_experiment(series, config)))
                    .collect::<Vec<_>>()
            }));
        }

        let mut indexed_results = Vec::with_capacity(configs.len());

        for handle in handles {
            indexed_results.extend(handle.join().expect("worker should not panic"));
        }

        indexed_results.sort_by_key(|(index, _)| *index);

        indexed_results
            .into_iter()
            .map(|(_, result)| result)
            .collect()
    })
}
