use serde::{Deserialize, Serialize};

#[cfg(test)]
const URL: &str = "https://api.metro.taipei/MetroAPI/ParkingLot.asmx";

#[cfg(test)]
const XMLNS: &str = "http://tempuri.org/";

#[derive(Debug, Serialize)]
#[serde(rename = "getParkingLot")]
struct Query {
    #[serde(rename = "@xmlns")]
    xmlns: &'static str,
    #[serde(rename = "userName")]
    username: String,
    #[serde(rename = "passWord")]
    password: String,
}

impl Query {
    #[cfg(test)]
    fn new(username: String, password: String) -> Self {
        Query {
            xmlns: XMLNS,
            username,
            password,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
struct Reply(Vec<ParkingLot>);

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ParkingLot {
    station_name: String,
    park_name: String,
    park_type: String,
    park_now_no: String,
    park_total_no: String,
}

#[cfg(test)]
mod tests {
    use crate::model::metro::{QUERY_HEADER, QUERY_FOOTER, remove_newline};
    use reqwest::header::CONTENT_TYPE;
    use super::*;

    #[test]
    fn serialize_query_into_xml() {
        let username = "USERNAME".to_owned();
        let password = "PASSWORD".to_owned();
        let query = Query::new(username, password);

        let xml = quick_xml::se::to_string(&query)
            .expect("builtin type must be serializable");

        let actual = [QUERY_HEADER, &xml, QUERY_FOOTER].concat();
        let expect = include_str!("../../../data/metro-parking-lot-query.xml");
        assert_eq!(remove_newline(actual), remove_newline(expect));
    }

    #[test]
    fn deserialize_reply_from_json() {
        let reply = Reply(
            vec![
                ParkingLot {
                    station_name: "R28淡水".to_owned(),
                    park_name: "R28淡水[汽車]".to_owned(),
                    park_type: "汽車".to_owned(),
                    park_now_no: "292".to_owned(),
                    park_total_no: "553".to_owned(),
                },
                ParkingLot {
                    station_name: "R27紅樹林".to_owned(),
                    park_name: "R27紅樹林[汽車]".to_owned(),
                    park_type: "汽車".to_owned(),
                    park_now_no: "118".to_owned(),
                    park_total_no: "215".to_owned(),
                },
                ParkingLot {
                    station_name: "R24忠義".to_owned(),
                    park_name: "R24忠義[汽車]".to_owned(),
                    park_type: "汽車".to_owned(),
                    park_now_no: "7".to_owned(),
                    park_total_no: "44".to_owned(),
                },
            ],
        );

        let json = include_str!("../../../data/metro-parking-lot-reply.json");

        let actual: Reply = serde_json::from_str(&json).unwrap();
        let expect = reply;
        assert_eq!(actual, expect);
    }

    #[tokio::test]
    #[ignore = "requires metro api username and password"]
    async fn fetch_reply_from_server() {
        let client = reqwest::Client::new();

        let username = std::env::var("METRO_USERNAME")
            .expect("METRO_USERNAME must be specified");
        let password = std::env::var("METRO_PASSWORD")
            .expect("METRO_PASSWORD must be specified");
        let query = Query::new(username, password);

        let xml = quick_xml::se::to_string(&query)
            .expect("builtin type must be serializable");
        let data = [QUERY_HEADER, &xml, QUERY_FOOTER].concat();
        let request = client.post(URL)
            .header(CONTENT_TYPE, "text/xml; charset=utf-8")
            .body(data);

        let response = request.send().await.unwrap();
        let body = response.text().await.unwrap();
        let json = body.split(r#"<?xml version="1.0" encoding="utf-8"?>"#).next().unwrap();

        let reply: Reply = serde_json::from_str(&json).unwrap();
        println!("{reply:?}");
    }
}
