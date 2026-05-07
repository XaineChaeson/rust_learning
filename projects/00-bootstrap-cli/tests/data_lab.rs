use bootstrap_cli::{DataError, parse_observations, score_trend, summarize};

#[test]
fn parses_sample_dataset() {
    let csv = include_str!("../data/study_log.csv");

    let observations = parse_observations(csv).expect("sample data should parse");

    assert_eq!(observations.len(), 7);
    assert_eq!(observations[0].day, "2026-05-01");
    assert_eq!(observations[0].study_minutes, 45);
    assert_eq!(observations[0].exercise_score, 78.5);
}

#[test]
fn reports_best_day() {
    let csv = include_str!("../data/study_log.csv");
    let observations = parse_observations(csv).expect("sample data should parse");

    let summary = summarize(&observations).expect("sample data has rows");

    assert_eq!(summary.best_day, "2026-05-06");
    assert_eq!(summary.best_score, 94.0);
}

#[test]
fn rejects_invalid_number() {
    let csv = "day,study_minutes,exercise_score\n2026-05-01,many,78.5\n";

    let error = parse_observations(csv).expect_err("invalid minutes should fail");

    assert_eq!(
        error,
        DataError::InvalidStudyMinutes {
            line: 2,
            value: String::from("many"),
        }
    );
}

#[test]
fn computes_score_trend() {
    let csv = "day,study_minutes,exercise_score\n2026-05-01,45,70.0\n2026-05-02,60,75.5\n2026-05-03,30,72.0\n";
    let observations = parse_observations(csv).expect("valid data should parse");

    let trend = score_trend(&observations);

    assert_eq!(trend, vec![5.5, -3.5]);
}
