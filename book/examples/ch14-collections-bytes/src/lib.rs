use std::collections::{BTreeMap, HashMap};

pub fn latest_prices(rows: &[(&str, f64)]) -> HashMap<String, f64> {
    let mut prices = HashMap::new();

    for (symbol, price) in rows {
        prices.insert((*symbol).to_string(), *price);
    }

    prices
}

pub fn ordered_by_timestamp(rows: &[(u64, f64)]) -> BTreeMap<u64, f64> {
    rows.iter().copied().collect()
}

pub fn parse_ascii_price(bytes: &[u8]) -> Option<f64> {
    let text = std::str::from_utf8(bytes).ok()?;
    text.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashmap_models_latest_value_lookup() {
        let prices = latest_prices(&[("AAPL", 100.0), ("AAPL", 101.0), ("MSFT", 50.0)]);

        assert_eq!(prices.get("AAPL"), Some(&101.0));
    }

    #[test]
    fn btree_map_preserves_ordered_iteration() {
        let ordered = ordered_by_timestamp(&[(3, 30.0), (1, 10.0), (2, 20.0)]);
        let keys = ordered.keys().copied().collect::<Vec<_>>();

        assert_eq!(keys, vec![1, 2, 3]);
    }

    #[test]
    fn bytes_are_not_strings_until_validated() {
        assert_eq!(parse_ascii_price(b"101.25"), Some(101.25));
    }
}
