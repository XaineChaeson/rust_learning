use std::env;
use std::fs;
use std::hint::black_box;
use std::path::Path;
use std::process;
use std::time::{Duration, Instant};

use factor_core::{rolling_mean, rolling_mean_incremental};

#[derive(Debug, Clone)]
struct BenchConfig {
    len: usize,
    window: usize,
    repeat: usize,
    output: Option<String>,
}

impl Default for BenchConfig {
    fn default() -> Self {
        Self {
            len: 200_000,
            window: 252,
            repeat: 20,
            output: None,
        }
    }
}

fn parse_config() -> Result<BenchConfig, String> {
    let mut config = BenchConfig::default();
    let mut args = env::args().skip(1);

    while let Some(flag) = args.next() {
        if matches!(flag.as_str(), "--help" | "-h") {
            print_help();
            process::exit(0);
        }

        let value = args
            .next()
            .ok_or_else(|| format!("missing value after `{flag}`"))?;

        match flag.as_str() {
            "--len" => {
                config.len = value
                    .parse()
                    .map_err(|_| format!("invalid --len value `{value}`"))?;
            }
            "--window" => {
                config.window = value
                    .parse()
                    .map_err(|_| format!("invalid --window value `{value}`"))?;
            }
            "--repeat" => {
                config.repeat = value
                    .parse()
                    .map_err(|_| format!("invalid --repeat value `{value}`"))?;
            }
            "--output" => {
                if value.trim().is_empty() {
                    return Err(String::from("--output must not be empty"));
                }

                config.output = Some(value);
            }
            other => return Err(format!("unknown flag `{other}`")),
        }
    }

    if config.len == 0 {
        return Err(String::from("--len must be greater than zero"));
    }

    if config.window == 0 {
        return Err(String::from("--window must be greater than zero"));
    }

    if config.repeat == 0 {
        return Err(String::from("--repeat must be greater than zero"));
    }

    Ok(config)
}

fn print_help() {
    println!("factor-core benchmark");
    println!();
    println!("Usage:");
    println!("  cargo run -p factor-core --release --bin bench");
    println!(
        "  cargo run -p factor-core --release --bin bench -- --len 1000000 --window 252 --repeat 10"
    );
    println!(
        "  cargo run -p factor-core --release --bin bench -- --output target/benchmark-reports/factor-core.md"
    );
}

fn synthetic_prices(len: usize) -> Vec<f64> {
    let mut state = 0x9E37_79B9_7F4A_7C15_u64;
    let mut prices = Vec::with_capacity(len);
    let mut price = 100.0;

    for _ in 0..len {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let centered = ((state >> 33) as f64 / u32::MAX as f64) - 0.5;
        price *= 1.0 + centered * 0.002;
        prices.push(price);
    }

    prices
}

fn run_repeated<F>(repeat: usize, mut f: F) -> Duration
where
    F: FnMut(),
{
    let started = Instant::now();

    for _ in 0..repeat {
        f();
    }

    started.elapsed()
}

fn assert_close_vec(left: &[f64], right: &[f64]) {
    assert_eq!(left.len(), right.len());

    for (index, (left_value, right_value)) in left.iter().zip(right).enumerate() {
        let tolerance = 1e-9_f64.max(right_value.abs() * 1e-12);
        assert!(
            (left_value - right_value).abs() <= tolerance,
            "values differ at index {index}: left={left_value}, right={right_value}"
        );
    }
}

fn average_duration(total: Duration, repeat: usize) -> Duration {
    Duration::from_secs_f64(total.as_secs_f64() / repeat as f64)
}

fn benchmark_report(
    config: &BenchConfig,
    output_len: usize,
    baseline_average: Duration,
    candidate_average: Duration,
    speedup: f64,
) -> String {
    format!(
        "# factor-core rolling mean benchmark\n\n\
         len: {}\n\
         window: {}\n\
         repeat: {}\n\
         output_len: {}\n\
         baseline_avg_ms: {:.3}\n\
         candidate_avg_ms: {:.3}\n\
         speedup: {:.2}x\n\
         correctness: baseline and candidate matched\n",
        config.len,
        config.window,
        config.repeat,
        output_len,
        baseline_average.as_secs_f64() * 1000.0,
        candidate_average.as_secs_f64() * 1000.0,
        speedup
    )
}

fn write_report(path: &str, report: &str) -> Result<(), String> {
    let path = Path::new(path);

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create report directory: {error}"))?;
    }

    fs::write(path, report).map_err(|error| format!("failed to write benchmark report: {error}"))
}

fn main() {
    let config = parse_config().unwrap_or_else(|error| {
        eprintln!("{error}");
        eprintln!("Run with --help for usage.");
        process::exit(2);
    });

    let values = synthetic_prices(config.len);
    let baseline = rolling_mean(&values, config.window).expect("baseline input should be valid");
    let candidate =
        rolling_mean_incremental(&values, config.window).expect("candidate input should be valid");

    assert_close_vec(&baseline, &candidate);

    let baseline_total = run_repeated(config.repeat, || {
        let output = rolling_mean(black_box(&values), black_box(config.window))
            .expect("baseline input should be valid");
        black_box(output);
    });

    let candidate_total = run_repeated(config.repeat, || {
        let output = rolling_mean_incremental(black_box(&values), black_box(config.window))
            .expect("candidate input should be valid");
        black_box(output);
    });

    let baseline_average = average_duration(baseline_total, config.repeat);
    let candidate_average = average_duration(candidate_total, config.repeat);
    let speedup = baseline_average.as_secs_f64() / candidate_average.as_secs_f64();
    let report = benchmark_report(
        &config,
        baseline.len(),
        baseline_average,
        candidate_average,
        speedup,
    );

    print!("{report}");

    if let Some(output) = &config.output {
        write_report(output, &report).unwrap_or_else(|error| {
            eprintln!("{error}");
            process::exit(2);
        });
    }
}
