use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::parse_optional_date_time;
pub const URL: &str = "https://api.metro.taipei/metroapi/TrackInfo.asmx";

const XMLNS: &str = "http://tempuri.org/";

#[derive(Debug, Serialize)]
#[serde(rename = "getTrackInfo")]
pub struct Query {
    #[serde(rename = "@xmlns")]
    xmlns: &'static str,
    #[serde(rename = "userName")]
    username: String,
    #[serde(rename = "passWord")]
    password: String,
}

impl Query {
    pub fn new(username: String, password: String) -> Self {
        Query {
            xmlns: XMLNS,
            username,
            password,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Reply(pub Vec<TrackInfo>);

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrackInfo {
    pub train_number: String,
    pub station_name: String,
    pub destination_name: String,
    pub count_down: String,
    #[serde(deserialize_with = "parse_optional_date_time")]
    pub now_date_time: Option<OffsetDateTime>,
}

#[cfg(test)]
mod tests {
    use crate::model::metro::{QUERY_HEADER, QUERY_FOOTER, remove_newline};
    use reqwest::header::CONTENT_TYPE;
    use time::macros::datetime;
    use super::*;

    #[test]
    fn serialize_query_into_xml() {
        let username = "USERNAME".to_owned();
        let password = "PASSWORD".to_owned();
        let query = Query::new(username, password);

        let xml = quick_xml::se::to_string(&query)
            .expect("builtin type must be serializable");

        let actual = [QUERY_HEADER, &xml, QUERY_FOOTER].concat();
        let expect = include_str!("../../../data/metro-track-info-query.xml");
        assert_eq!(remove_newline(actual), remove_newline(expect));
    }

    #[test]
    fn deserialize_reply_from_json() {
        let reply = Reply(
            vec![
                TrackInfo {
                    train_number: "".to_owned(),
                    station_name: "松山機場站".to_owned(),
                    destination_name: "南港展覽館站".to_owned(),
                    count_down: "00:34".to_owned(),
                    now_date_time: Some(datetime!(2024-04-04 23:27:09 +8)),
                },
                TrackInfo {
                    train_number: "".to_owned(),
                    station_name: "中山國中站".to_owned(),
                    destination_name: "南港展覽館站".to_owned(),
                    count_down: "09:47".to_owned(),
                    now_date_time: Some(datetime!(2024-04-04 23:27:09 +8)),
                },
                TrackInfo {
                    train_number: "".to_owned(),
                    station_name: "南京復興站".to_owned(),
                    destination_name: "南港展覽館站".to_owned(),
                    count_down: "07:31".to_owned(),
                    now_date_time: Some(datetime!(2024-04-04 23:27:09 +8)),
                },
            ],
        );

        let json = include_str!("../../../data/metro-track-info-reply.json");

        let actual: Reply = serde_json::from_str(&json).unwrap();
        let expect = reply;
        assert_eq!(actual, expect);

        // Regression test for invalid track info. The model should distinguish
        // them by filtering now_date_time field.
        let json_with_invalid_item =
            include_str!("../../../data/metro-track-info-reply-with-invalid-item.json");
        let reply: Reply = serde_json::from_str(&json_with_invalid_item).unwrap();
        assert!(reply.0[0].now_date_time.is_none());
        assert!(reply.0[1].now_date_time.is_some());
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
