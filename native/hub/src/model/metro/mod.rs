use serde::{Deserialize, Deserializer};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

mod parking_lot;
pub mod track_info;

pub const QUERY_HEADER: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
xmlns:xsd="http://www.w3.org/2001/XMLSchema"
xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
<soap:Body>
"#;

pub const QUERY_FOOTER: &str = r#"</soap:Body>
</soap:Envelope>
"#;

pub fn parse_optional_date_time<'de, D>(deserializer: D)
    -> Result<Option<OffsetDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;
    if string.is_empty() {
        return Ok(None);
    }
    let mut words = string.split_whitespace();
    let date = words.next().unwrap();
    let time = words.next().unwrap();
    let standarized = [date, "T", time, "+08:00"].concat();
    let result = OffsetDateTime::parse(&standarized, &Rfc3339).map(Some);
    result.map_err(serde::de::Error::custom)
}

#[cfg(test)]
fn remove_newline(input: impl AsRef<str>) -> String {
    let input = input.as_ref();
    let mut output = String::with_capacity(input.len());
    input.split("\n").for_each(|line| output.push_str(line));
    output
}
