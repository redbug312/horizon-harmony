use reqwest::Client;
use reqwest::header::CONTENT_TYPE;

use crate::model::metro::{track_info, QUERY_FOOTER, QUERY_HEADER};

#[derive(Clone)]
pub struct Api {
    client: Client,
    metro_username: &'static str,
    metro_password: &'static str,
}

impl Api {
    pub fn new() -> Self {
        Api {
            client: reqwest::Client::new(),
            metro_username: std::env!("METRO_USERNAME"),
            metro_password: std::env!("METRO_PASSWORD"),
        }
    }

    pub async fn fetch_track_info(&self) -> track_info::Reply {
        let username = self.metro_username.to_owned();
        let password = self.metro_password.to_owned();
        let query = track_info::Query::new(username, password);

        let xml = quick_xml::se::to_string(&query)
            .expect("builtin type must be serializable");
        let data = [QUERY_HEADER, &xml, QUERY_FOOTER].concat();
        let request = self.client
            .post(track_info::URL)
            .header(CONTENT_TYPE, "text/xml; charset=utf-8")
            .body(data);

        let response = request.send().await.unwrap();
        let body = response.text().await.unwrap();
        let json = body.split(r#"<?xml version="1.0" encoding="utf-8"?>"#).next().unwrap();

        let reply: track_info::Reply = serde_json::from_str(&json).unwrap();
        reply
    }
}
