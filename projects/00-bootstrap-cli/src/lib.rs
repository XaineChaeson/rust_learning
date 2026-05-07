use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    pub day: String,
    pub study_minutes: u32,
    pub exercise_score: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    pub count: usize,
    pub total_study_minutes: u32,
    pub average_study_minutes: f64,
    pub average_score: f64,
    pub best_day: String,
    pub best_score: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataError {
    MissingHeader,
    InvalidHeader {
        expected: &'static str,
        found: String,
    },
    NoRows,
    WrongColumnCount {
        line: usize,
        expected: usize,
        found: usize,
    },
    InvalidStudyMinutes {
        line: usize,
        value: String,
    },
    InvalidExerciseScore {
        line: usize,
        value: String,
    },
}

impl fmt::Display for DataError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::MissingHeader => write!(formatter, "CSV is missing a header row"),
            DataError::InvalidHeader { expected, found } => {
                write!(
                    formatter,
                    "invalid header: expected `{expected}`, found `{found}`"
                )
            }
            DataError::NoRows => write!(formatter, "CSV does not contain any data rows"),
            DataError::WrongColumnCount {
                line,
                expected,
                found,
            } => write!(
                formatter,
                "line {line} has {found} columns, but {expected} columns are required"
            ),
            DataError::InvalidStudyMinutes { line, value } => write!(
                formatter,
                "line {line} has invalid study_minutes value `{value}`"
            ),
            DataError::InvalidExerciseScore { line, value } => write!(
                formatter,
                "line {line} has invalid exercise_score value `{value}`"
            ),
        }
    }
}

impl Error for DataError {}

pub fn parse_observations(csv: &str) -> Result<Vec<Observation>, DataError> {
    let mut lines = csv.lines().enumerate().filter_map(|(index, line)| {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            None
        } else {
            Some((index + 1, trimmed))
        }
    });

    let (_, header) = lines.next().ok_or(DataError::MissingHeader)?;
    let expected_header = "day,study_minutes,exercise_score";

    if header != expected_header {
        return Err(DataError::InvalidHeader {
            expected: expected_header,
            found: header.to_string(),
        });
    }

    let mut observations = Vec::new();

    for (line_number, line) in lines {
        let columns: Vec<&str> = line.split(',').map(str::trim).collect();

        if columns.len() != 3 {
            return Err(DataError::WrongColumnCount {
                line: line_number,
                expected: 3,
                found: columns.len(),
            });
        }

        let study_minutes =
            columns[1]
                .parse::<u32>()
                .map_err(|_| DataError::InvalidStudyMinutes {
                    line: line_number,
                    value: columns[1].to_string(),
                })?;

        let exercise_score =
            columns[2]
                .parse::<f64>()
                .map_err(|_| DataError::InvalidExerciseScore {
                    line: line_number,
                    value: columns[2].to_string(),
                })?;

        if !exercise_score.is_finite() {
            return Err(DataError::InvalidExerciseScore {
                line: line_number,
                value: columns[2].to_string(),
            });
        }

        observations.push(Observation {
            day: columns[0].to_string(),
            study_minutes,
            exercise_score,
        });
    }

    if observations.is_empty() {
        Err(DataError::NoRows)
    } else {
        Ok(observations)
    }
}

pub fn summarize(observations: &[Observation]) -> Option<Summary> {
    if observations.is_empty() {
        return None;
    }

    let count = observations.len();
    let total_study_minutes = observations
        .iter()
        .map(|observation| observation.study_minutes)
        .sum::<u32>();
    let total_score = observations
        .iter()
        .map(|observation| observation.exercise_score)
        .sum::<f64>();
    let best = observations
        .iter()
        .max_by(|left, right| {
            left.exercise_score
                .partial_cmp(&right.exercise_score)
                .unwrap_or(Ordering::Equal)
        })
        .expect("observations is known to be non-empty");

    Some(Summary {
        count,
        total_study_minutes,
        average_study_minutes: total_study_minutes as f64 / count as f64,
        average_score: total_score / count as f64,
        best_day: best.day.clone(),
        best_score: best.exercise_score,
    })
}

pub fn score_trend(observations: &[Observation]) -> Vec<f64> {
    observations
        .windows(2)
        .map(|window| window[1].exercise_score - window[0].exercise_score)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_csv() {
        let csv = "day,study_minutes,exercise_score\n2026-05-01,45,78.5\n";

        let observations = parse_observations(csv).expect("valid CSV should parse");

        assert_eq!(
            observations,
            vec![Observation {
                day: String::from("2026-05-01"),
                study_minutes: 45,
                exercise_score: 78.5,
            }]
        );
    }

    #[test]
    fn rejects_csv_without_rows() {
        let csv = "day,study_minutes,exercise_score\n";

        let error = parse_observations(csv).expect_err("CSV without rows should fail");

        assert_eq!(error, DataError::NoRows);
    }

    #[test]
    fn summarizes_observations() {
        let observations = vec![
            Observation {
                day: String::from("2026-05-01"),
                study_minutes: 40,
                exercise_score: 80.0,
            },
            Observation {
                day: String::from("2026-05-02"),
                study_minutes: 80,
                exercise_score: 90.0,
            },
        ];

        let summary = summarize(&observations).expect("non-empty observations have a summary");

        assert_eq!(summary.count, 2);
        assert_eq!(summary.total_study_minutes, 120);
        assert_eq!(summary.average_study_minutes, 60.0);
        assert_eq!(summary.average_score, 85.0);
        assert_eq!(summary.best_day, "2026-05-02");
    }
}
