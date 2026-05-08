fn main() {
    let returns = [0.01, -0.02, 0.015, 0.03, -0.01, 0.005];

    let positive_days = returns
        .iter()
        .filter(|daily_return| **daily_return > 0.0)
        .collect::<Vec<_>>();
    let cumulative_return = returns
        .iter()
        .fold(1.0, |capital, daily_return| capital * (1.0 + daily_return))
        - 1.0;
    let average_return = returns.iter().sum::<f64>() / returns.len() as f64;

    println!("positive days: {}", positive_days.len());
    println!("average return: {average_return:.4}");
    println!("cumulative return: {cumulative_return:.4}");
}
