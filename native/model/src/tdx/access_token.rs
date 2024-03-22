use serde::{Deserialize, Serialize};

#[cfg(test)]
const URL: &str = "https://tdx.transportdata.tw/auth/realms/TDXConnect/protocol/openid-connect/token";

#[cfg(test)]
const GRANT_TYPE: &str = "client_credentials";

#[derive(Debug, Serialize)]
struct Query {
    grant_type: &'static str,
    client_id: String,
    client_secret: String,
}

impl Query {
    #[cfg(test)]
    fn new(client_id: String, client_secret: String) -> Self {
        Query {
            grant_type: GRANT_TYPE,
            client_id,
            client_secret,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
struct Reply {
    access_token: String,
    expires_in: u64,
    token_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires tdx client id and secret"]
    async fn obtain_access_token() {
        let client = reqwest::Client::new();

        let id = std::env::var("TDX_ID")
            .expect("TDX_ID must be specified");
        let secret = std::env::var("TDX_SECRET")
            .expect("TDX_SECRET must be specified");
        let query = Query::new(id, secret);

        let request = client.post(URL).form(&query);

        let response = request.send().await.unwrap();
        let body = response.text().await.unwrap();

        let reply: Reply = serde_json::from_str(&body).unwrap();
        assert_eq!(reply.token_type, "Bearer");
    }
}
