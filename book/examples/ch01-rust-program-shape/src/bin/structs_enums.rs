#[derive(Debug)]
struct StudySession {
    day: String,
    minutes: u32,
    result: SessionResult,
}

#[derive(Debug)]
enum SessionResult {
    Completed { score: f64 },
    Skipped { reason: String },
}

impl StudySession {
    fn completed(day: &str, minutes: u32, score: f64) -> Self {
        Self {
            day: day.to_string(),
            minutes,
            result: SessionResult::Completed { score },
        }
    }

    fn summary(&self) -> String {
        match &self.result {
            SessionResult::Completed { score } => {
                format!(
                    "{}: studied {} minutes, score {score}",
                    self.day, self.minutes
                )
            }
            SessionResult::Skipped { reason } => {
                format!("{}: skipped because {reason}", self.day)
            }
        }
    }
}

fn main() {
    let completed = StudySession::completed("2026-05-01", 45, 78.5);
    let skipped = StudySession {
        day: String::from("2026-05-02"),
        minutes: 0,
        result: SessionResult::Skipped {
            reason: String::from("reviewed Python notes instead"),
        },
    };

    println!("{}", completed.summary());
    println!("{}", skipped.summary());
}
