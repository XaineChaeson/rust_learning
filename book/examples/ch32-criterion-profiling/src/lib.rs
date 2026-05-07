#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkSummary {
    pub samples: usize,
    pub min_nanos: u128,
    pub median_nanos: u128,
    pub max_nanos: u128,
    pub mean_nanos: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchmarkDecision {
    KeepBaseline,
    CandidateWins,
    Inconclusive,
}

pub fn summarize(samples: &[u128]) -> Option<BenchmarkSummary> {
    if samples.is_empty() {
        return None;
    }

    let mut sorted = samples.to_vec();
    sorted.sort_unstable();

    let total = sorted.iter().sum::<u128>();
    let middle = sorted.len() / 2;
    let median = if sorted.len().is_multiple_of(2) {
        (sorted[middle - 1] + sorted[middle]) / 2
    } else {
        sorted[middle]
    };

    Some(BenchmarkSummary {
        samples: sorted.len(),
        min_nanos: sorted[0],
        median_nanos: median,
        max_nanos: sorted[sorted.len() - 1],
        mean_nanos: total as f64 / sorted.len() as f64,
    })
}

pub fn speedup_ratio(baseline: &BenchmarkSummary, candidate: &BenchmarkSummary) -> Option<f64> {
    if candidate.mean_nanos <= 0.0 {
        None
    } else {
        Some(baseline.mean_nanos / candidate.mean_nanos)
    }
}

pub fn decide(
    baseline: &BenchmarkSummary,
    candidate: &BenchmarkSummary,
    minimum_speedup: f64,
    max_noise_ratio: f64,
) -> BenchmarkDecision {
    let baseline_noise = baseline.max_nanos as f64 / baseline.min_nanos.max(1) as f64;
    let candidate_noise = candidate.max_nanos as f64 / candidate.min_nanos.max(1) as f64;

    if baseline_noise > max_noise_ratio || candidate_noise > max_noise_ratio {
        return BenchmarkDecision::Inconclusive;
    }

    match speedup_ratio(baseline, candidate) {
        Some(ratio) if ratio >= minimum_speedup => BenchmarkDecision::CandidateWins,
        Some(_) => BenchmarkDecision::KeepBaseline,
        None => BenchmarkDecision::Inconclusive,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_samples_with_median_and_mean() {
        let summary = summarize(&[110, 90, 100, 120]).expect("non-empty samples");

        assert_eq!(summary.samples, 4);
        assert_eq!(summary.min_nanos, 90);
        assert_eq!(summary.median_nanos, 105);
        assert_eq!(summary.max_nanos, 120);
        assert_eq!(summary.mean_nanos, 105.0);
    }

    #[test]
    fn detects_candidate_speedup_when_noise_is_controlled() {
        let baseline = summarize(&[1_000, 1_050, 980]).expect("samples");
        let candidate = summarize(&[500, 510, 490]).expect("samples");

        assert_eq!(
            decide(&baseline, &candidate, 1.5, 1.25),
            BenchmarkDecision::CandidateWins
        );
    }

    #[test]
    fn refuses_to_conclude_when_noise_is_too_high() {
        let baseline = summarize(&[1_000, 10_000, 900]).expect("samples");
        let candidate = summarize(&[500, 510, 490]).expect("samples");

        assert_eq!(
            decide(&baseline, &candidate, 1.5, 2.0),
            BenchmarkDecision::Inconclusive
        );
    }
}
