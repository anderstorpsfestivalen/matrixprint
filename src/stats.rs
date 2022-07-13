use crate::error::Error;

pub struct Stats {
    url: String,
    key: String,
    client: reqwest::Client,
}

impl Stats {
    pub fn new(endpoint: &str, key: &str) -> Result<Stats, Error> {
        if key == "" {
            return Err(Error::NoStatsKeySpecificed)
        }

        Ok(Stats{
            url: endpoint.to_string(),
            key: key.to_string(),
            client: reqwest::Client::new(),
        })

    }

    pub async fn print(&self) -> Result<(), Error> {

        self.client.get(&self.url)
        .query(&[("key", &self.key)])
        .send().await?;

        Ok(())
    }
}