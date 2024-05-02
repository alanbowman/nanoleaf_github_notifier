use reqwest::{
    header::{HeaderValue, ACCEPT, ETAG, IF_NONE_MATCH, USER_AGENT},
    Error,
};
use std::time::Duration;

pub struct GithubClient<'a> {
    client: reqwest::blocking::Client,
    api_key: &'a str,
    last_etag: Option<HeaderValue>,
    poll_interval: Option<HeaderValue>,
}

impl<'a> GithubClient<'a> {
    pub fn new(api_key: &'a str) -> GithubClient<'a> {
        GithubClient {
            client: reqwest::blocking::Client::new(),
            api_key,
            last_etag: None,
            poll_interval: None,
        }
    }

    pub fn check_for_notifications(&mut self) -> Result<(usize, Duration), Error> {
        let mut request = self
            .client
            .get("https://api.github.com/notifications")
            .bearer_auth(self.api_key)
            .header(USER_AGENT, "nanoleaf_notifier")
            .header(ACCEPT, "application/json");

        if let Some(etag) = &self.last_etag {
            request = request.header(IF_NONE_MATCH, etag);
        }

        let response = request.send()?;

        println!("{:#?}", response);

        self.last_etag = response.headers().get(ETAG).cloned();
        self.poll_interval = response.headers().get("X-Poll-Interval").cloned();

        if response.status().is_success() {
            // get the number of notifications
            let notifications: serde_json::Value = response.json().unwrap();
            println!("{notifications:#?}");

            match notifications {
                serde_json::Value::Array(n) => {
                    let notification_count = n.len();
                    println!("Got {notification_count} notifications");

                    let poll_interval = if self.poll_interval.is_none() {
                        20
                    } else {
                        let poll_header = self.poll_interval.clone().unwrap();

                        // surely not...
                        poll_header.to_str().unwrap().parse::<u64>().unwrap()
                    };

                    return Ok((notification_count, Duration::from_secs(poll_interval)));
                }
                _ => todo!("Different JSON types"),
            }
        }

        Ok((0, Duration::from_secs(20)))
    }
}
