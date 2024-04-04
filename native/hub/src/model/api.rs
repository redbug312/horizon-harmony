use reqwest::Client;
use reqwest::header::CONTENT_TYPE;

use crate::model::metro::{track_info, QUERY_FOOTER, QUERY_HEADER};

#[derive(Clone)]
pub struct Api {
    client: Client,
    metro_username: String,
    metro_password: String,
}

impl Api {
    pub fn new() -> Self {
        let username = std::env::var("METRO_USERNAME")
            .expect("METRO_USERNAME must be specified");
        let password = std::env::var("METRO_PASSWORD")
            .expect("METRO_PASSWORD must be specified");
        Api {
            client: reqwest::Client::new(),
            metro_username: username,
            metro_password: password,
        }
    }

    pub async fn fetch_track_info(&self) -> track_info::Reply {
        let username = self.metro_username.clone();
        let password = self.metro_password.clone();
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
