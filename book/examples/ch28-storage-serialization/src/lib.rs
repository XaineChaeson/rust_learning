#[derive(Debug, Clone, PartialEq)]
pub struct FactorRecord {
    pub timestamp: u64,
    pub symbol: String,
    pub value: f64,
}

pub fn encode_line(record: &FactorRecord) -> String {
    format!("{},{},{}", record.timestamp, record.symbol, record.value)
}

pub fn decode_line(line: &str) -> Option<FactorRecord> {
    let mut fields = line.split(',');
    let timestamp = fields.next()?.parse().ok()?;
    let symbol = fields.next()?.to_string();
    let value = fields.next()?.parse().ok()?;

    if fields.next().is_some() {
        return None;
    }

    Some(FactorRecord {
        timestamp,
        symbol,
        value,
    })
}

pub fn encode_f64_le(value: f64) -> [u8; 8] {
    value.to_le_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_encoding_roundtrips_small_records() {
        let record = FactorRecord {
            timestamp: 1,
            symbol: String::from("AAPL"),
            value: 0.25,
        };

        assert_eq!(decode_line(&encode_line(&record)), Some(record));
    }

    #[test]
    fn binary_encoding_makes_endianness_explicit() {
        assert_eq!(f64::from_le_bytes(encode_f64_le(1.5)), 1.5);
    }
}
