use crate::EngineError;

#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkPlan {
    pub name: String,
    pub baseline_name: String,
    pub candidate_name: String,
    pub input_rows: usize,
    pub repeat: usize,
    pub minimum_speedup: f64,
    pub max_noise_ratio: f64,
}

impl BenchmarkPlan {
    pub fn validate(&self) -> Result<(), EngineError> {
        if self.name.trim().is_empty()
            || self.baseline_name.trim().is_empty()
            || self.candidate_name.trim().is_empty()
        {
            return Err(EngineError::InvalidConfig {
                context: "benchmark names must not be empty",
            });
        }

        if self.input_rows == 0 || self.repeat == 0 {
            return Err(EngineError::InvalidConfig {
                context: "benchmark input_rows and repeat must be greater than zero",
            });
        }

        if !self.minimum_speedup.is_finite() || self.minimum_speedup <= 1.0 {
            return Err(EngineError::InvalidConfig {
                context: "minimum_speedup must be finite and greater than 1",
            });
        }

        if !self.max_noise_ratio.is_finite() || self.max_noise_ratio < 1.0 {
            return Err(EngineError::InvalidConfig {
                context: "max_noise_ratio must be finite and at least 1",
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkObservation {
    pub baseline_nanos: Vec<u128>,
    pub candidate_nanos: Vec<u128>,
    pub outputs_match: bool,
}

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
    CandidateWins,
    KeepBaseline,
    Inconclusive,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkReport {
    pub plan: BenchmarkPlan,
    pub baseline: BenchmarkSummary,
    pub candidate: BenchmarkSummary,
    pub speedup: f64,
    pub decision: BenchmarkDecision,
}

pub fn summarize_samples(samples: &[u128]) -> Option<BenchmarkSummary> {
    if samples.is_empty() {
        return None;
    }

    let mut sorted = samples.to_vec();
    sorted.sort_unstable();
    let middle = sorted.len() / 2;
    let median = if sorted.len().is_multiple_of(2) {
        (sorted[middle - 1] + sorted[middle]) / 2
    } else {
        sorted[middle]
    };
    let total = sorted.iter().sum::<u128>();

    Some(BenchmarkSummary {
        samples: sorted.len(),
        min_nanos: sorted[0],
        median_nanos: median,
        max_nanos: sorted[sorted.len() - 1],
        mean_nanos: total as f64 / sorted.len() as f64,
    })
}

pub fn evaluate_benchmark(
    plan: BenchmarkPlan,
    observation: BenchmarkObservation,
) -> Result<BenchmarkReport, EngineError> {
    plan.validate()?;

    if !observation.outputs_match {
        return Err(EngineError::InvalidConfig {
            context: "benchmark candidate output does not match baseline",
        });
    }

    let baseline =
        summarize_samples(&observation.baseline_nanos).ok_or(EngineError::EmptyInput {
            context: "baseline benchmark samples",
        })?;
    let candidate =
        summarize_samples(&observation.candidate_nanos).ok_or(EngineError::EmptyInput {
            context: "candidate benchmark samples",
        })?;

    let speedup = baseline.mean_nanos / candidate.mean_nanos;
    let baseline_noise = baseline.max_nanos as f64 / baseline.min_nanos.max(1) as f64;
    let candidate_noise = candidate.max_nanos as f64 / candidate.min_nanos.max(1) as f64;

    let evidence_is_insufficient =
        baseline.samples < plan.repeat || candidate.samples < plan.repeat;
    let noise_is_too_high =
        baseline_noise > plan.max_noise_ratio || candidate_noise > plan.max_noise_ratio;

    let decision = if evidence_is_insufficient || noise_is_too_high {
        BenchmarkDecision::Inconclusive
    } else if speedup >= plan.minimum_speedup {
        BenchmarkDecision::CandidateWins
    } else {
        BenchmarkDecision::KeepBaseline
    };

    Ok(BenchmarkReport {
        plan,
        baseline,
        candidate,
        speedup,
        decision,
    })
}
