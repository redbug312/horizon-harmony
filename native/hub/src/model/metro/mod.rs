mod parking_lot;
mod track_info;

#[cfg(test)]
const QUERY_HEADER: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
xmlns:xsd="http://www.w3.org/2001/XMLSchema"
xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
<soap:Body>
"#;

#[cfg(test)]
const QUERY_FOOTER: &str = r#"</soap:Body>
</soap:Envelope>
"#;

#[cfg(test)]
fn remove_newline(input: impl AsRef<str>) -> String {
    let input = input.as_ref();
    let mut output = String::with_capacity(input.len());
    input.split("\n").for_each(|line| output.push_str(line));
    output
}
