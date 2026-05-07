pub fn last(values: &[f64]) -> Option<f64> {
    values.last().copied()
}

pub fn window(values: &[f64], start: usize, len: usize) -> Option<&[f64]> {
    let end = start.checked_add(len)?;

    if start > values.len() || end > values.len() {
        return None;
    }

    Some(&values[start..end])
}

pub fn returns(prices: &[f64]) -> Vec<f64> {
    prices
        .windows(2)
        .map(|pair| pair[1] / pair[0] - 1.0)
        .collect()
}

pub fn symbol_label(symbol: &str, field: &str) -> String {
    format!("{symbol}.{field}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_helpers_borrow_input() {
        let values = vec![10.0, 20.0, 30.0, 40.0];

        assert_eq!(last(&values), Some(40.0));
        assert_eq!(window(&values, 1, 2), Some(&[20.0, 30.0][..]));
        assert_eq!(values.len(), 4);
    }

    #[test]
    fn label_accepts_owned_and_borrowed_strings() {
        let symbol = String::from("AAPL");

        assert_eq!(symbol_label(&symbol, "close"), "AAPL.close");
        assert_eq!(symbol_label("MSFT", "volume"), "MSFT.volume");
    }
}
