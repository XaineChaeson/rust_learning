#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Window<'a> {
    values: &'a [f64],
}

impl<'a> Window<'a> {
    pub fn new(values: &'a [f64]) -> Self {
        Self { values }
    }

    pub fn values(&self) -> &'a [f64] {
        self.values
    }

    pub fn last(&self) -> Option<f64> {
        self.values.last().copied()
    }
}

pub fn rolling_windows<'a>(values: &'a [f64], window: usize) -> Vec<Window<'a>> {
    if window == 0 || window > values.len() {
        return Vec::new();
    }

    values.windows(window).map(Window::new).collect()
}

pub fn choose_longer<'a>(left: &'a [f64], right: &'a [f64]) -> &'a [f64] {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_borrow_the_original_series() {
        let values = vec![1.0, 2.0, 3.0, 4.0];
        let windows = rolling_windows(&values, 3);

        assert_eq!(windows[0].values(), &[1.0, 2.0, 3.0]);
        assert_eq!(windows[1].last(), Some(4.0));
        assert_eq!(values.len(), 4);
    }

    #[test]
    fn returned_borrow_cannot_outlive_inputs() {
        assert_eq!(choose_longer(&[1.0], &[1.0, 2.0]), &[1.0, 2.0]);
    }
}
