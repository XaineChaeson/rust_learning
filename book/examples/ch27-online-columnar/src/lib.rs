#[derive(Debug, Clone)]
pub struct OnlineMean {
    window: usize,
    values: Vec<f64>,
    cursor: usize,
    sum: f64,
    filled: usize,
}

impl OnlineMean {
    pub fn new(window: usize) -> Self {
        Self {
            window,
            values: vec![0.0; window],
            cursor: 0,
            sum: 0.0,
            filled: 0,
        }
    }

    pub fn update(&mut self, value: f64) -> Option<f64> {
        if self.window == 0 {
            return None;
        }

        if self.filled < self.window {
            self.filled += 1;
        } else {
            self.sum -= self.values[self.cursor];
        }

        self.values[self.cursor] = value;
        self.sum += value;
        self.cursor = (self.cursor + 1) % self.window;

        if self.filled == self.window {
            Some(self.sum / self.window as f64)
        } else {
            None
        }
    }
}

pub fn filter_prices_above(prices: &[f64], threshold: f64) -> Vec<f64> {
    prices
        .iter()
        .copied()
        .filter(|price| *price > threshold)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn online_mean_updates_incrementally() {
        let mut mean = OnlineMean::new(3);

        assert_eq!(mean.update(1.0), None);
        assert_eq!(mean.update(2.0), None);
        assert_eq!(mean.update(3.0), Some(2.0));
        assert_eq!(mean.update(4.0), Some(3.0));
    }
}
