fn main() {
    for raw_score in ["88.5", "not-a-number", "101.0"] {
        match parse_score(raw_score) {
            Ok(score) => println!("parsed score: {score}"),
            Err(error) => println!("could not parse `{raw_score}`: {error}"),
        }
    }
}

fn parse_score(raw: &str) -> Result<f64, String> {
    let score = raw
        .parse::<f64>()
        .map_err(|_| String::from("score must be a number"))?;

    if !(0.0..=100.0).contains(&score) {
        return Err(String::from("score must be between 0 and 100"));
    }

    Ok(score)
}
