use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

#[cfg(test)]
const URL: &str = "https://tdx.transportdata.tw/api/basic/v2/Rail/Metro/Alert/TYMC";

#[derive(Debug, Serialize)]
struct Query {
    #[cfg(test)]
    #[serde(skip)]
    access_token: String,
    top: usize,
}

impl Query {
    #[cfg(test)]
    fn new(access_token: String) -> Self {
        Query {
            access_token,
            top: 30,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Reply {
    #[serde(with = "rfc3339")]
    update_time: OffsetDateTime,
    update_interval: u64,
    #[serde(with = "rfc3339")]
    src_update_time: OffsetDateTime,
    src_update_interval: u64,
    authority_code: String,
    alerts: Vec<Alert>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Alert {
    #[serde(rename = "AlertID")]
    id: String,
    title: String,
    description: String,
    status: u64,
    #[serde(with = "rfc3339")]
    publish_time: OffsetDateTime,
    #[serde(with = "rfc3339")]
    update_time: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use reqwest::header::{ACCEPT_ENCODING, AUTHORIZATION};
    use time::macros::datetime;
    use super::*;

    #[test]
    fn serialize_query_into_url() {
        let client = reqwest::ClientBuilder::new()
            .gzip(true)
            .build()
            .unwrap();

        let access_token = "ACCESS-TOKEN".to_owned();
        let query = Query::new(access_token);

        let authorization = format!("Bearer {}", query.access_token);
        let request = client.get(URL)
            .header(AUTHORIZATION, &authorization)
            .header(ACCEPT_ENCODING, "gzip")
            .query(&query)
            .build()
            .unwrap();

        let actual = request.url().as_str();
        let expect = format!("{URL}?top=30");
        assert_eq!(actual, expect);
    }

    #[test]
    fn deserialize_reply_from_json() {
        let reply = Reply {
            update_time: datetime!(2024-03-17 18:10:42 +8),
            update_interval: 60,
            src_update_time: datetime!(2024-03-17 18:10:01 +8),
            src_update_interval: 60,
            authority_code: "TYMC".to_owned(),
            alerts: vec![
                Alert {
                    id: "20240317181001".to_owned(),
                    title: "正常營運".to_owned(),
                    description: "正常營運".to_owned(),
                    status: 1,
                    publish_time: datetime!(2024-03-17 18:10:01 +8),
                    update_time: datetime!(2024-03-17 18:10:01 +8),
                }
            ],
        };

        let json = include_str!("../../../data/tdx-alert-reply.json");

        let actual: Reply = serde_json::from_str(&json).unwrap();
        let expect = reply;
        assert_eq!(actual, expect);
    }

    #[tokio::test]
    #[ignore = "requires tdx access token"]
    async fn fetch_reply_from_server() {
        let client = reqwest::ClientBuilder::new()
            .gzip(true)
            .build()
            .unwrap();

        let access_token = std::env::var("TDX_ACCESS_TOKEN")
            .expect("TDX_ACCESS_TOKEN must be specified");
        let query = Query::new(access_token);

        let authorization = format!("Bearer {}", query.access_token);
        let request = client.get(URL)
            .header(AUTHORIZATION, &authorization)
            .header(ACCEPT_ENCODING, "gzip")
            .query(&query);

        let response = request.send().await.unwrap();
        let body = response.text().await.unwrap();

        let reply: Reply = serde_json::from_str(&body).unwrap();
        println!("{reply:?}");
    }
}
