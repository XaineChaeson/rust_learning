pub mod market {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Bar {
        symbol: String,
        close: f64,
    }

    impl Bar {
        pub fn new(symbol: impl Into<String>, close: f64) -> Self {
            Self {
                symbol: symbol.into(),
                close,
            }
        }

        pub fn symbol(&self) -> &str {
            &self.symbol
        }

        pub fn close(&self) -> f64 {
            self.close
        }
    }
}

pub mod factors {
    use crate::market::Bar;

    pub fn close_to_close_return(previous: &Bar, current: &Bar) -> Option<f64> {
        if previous.symbol() != current.symbol() || previous.close() <= 0.0 {
            return None;
        }

        Some(current.close() / previous.close() - 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::factors::close_to_close_return;
    use super::market::Bar;

    #[test]
    fn module_boundary_hides_fields_but_exposes_api() {
        let previous = Bar::new("AAPL", 100.0);
        let current = Bar::new("AAPL", 105.0);

        assert_eq!(previous.symbol(), "AAPL");
        let output = close_to_close_return(&previous, &current).expect("same symbol");
        assert!((output - 0.05).abs() < 1e-12);
    }
}
