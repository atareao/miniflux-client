use serde::{Serialize, Deserialize};
use reqwest::Client;
use serde_json::Value;
use tracing::debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinifluxClient {
    pub url: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    entry_ids: Vec<u64>,
    status: String
}

#[derive(Debug, Serialize)]
struct OneItem {
    status: String
}


impl MinifluxClient {

    pub fn new(url: String, token: String) -> Self {
        MinifluxClient {
            url,
            token,
        }
    }

    pub async fn get_categories(&self) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let url = format!("https://{}/v1/categories", self.url);
        let client = Client::new();
        let response = client
            .get(&url)
            .header("X-Auth-Token", &self.token)
            .send()
            .await?;
        let content = response.json::<serde_json::Value>().await?;
        Ok(content.as_array().unwrap().to_vec())
    }


    pub async fn get_category_entries(&self, category_id: i32) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let url = format!("https://{}/v1/categories/{}/entries", self.url, category_id);
        let client = Client::new();
        let response = client
            .get(&url)
            .query(&[("status", "unread")])
            .header("X-Auth-Token", &self.token)
            .send()
            .await?;
        let content = response.json::<serde_json::Value>().await?;
        Ok(content["entries"].as_array().unwrap().to_vec())
    }

    pub async fn get_entries(&self) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let url = format!("https://{}/v1/entries", self.url);
        let client = Client::new();
        let response = client
            .get(&url)
            .query(&[("status", "unread")])
            .header("X-Auth-Token", &self.token)
            .send()
            .await?;
        let content = response.json::<serde_json::Value>().await?;
        Ok(content["entries"].as_array().unwrap().to_vec())
    }

    pub async fn refresh_all_feeds(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("https://{}/v1/feeds/refresh", self.url);
        let client = Client::new();
        let response = client
            .put(&url)
            .header("X-Auth-Token", &self.token)
            .send()
            .await?;
        debug!("================");
        debug!("Response: {:?}", response);
        debug!("================");
        Ok(())
    }

    pub async fn get_content(&self, entry_id: u64) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("https://{}/v1/entries/{}/fetch-content", self.url, entry_id);
        let client = Client::new();
        let response = client
            .get(&url)
            .header("X-Auth-Token", &self.token)
            .send()
            .await?;
        let content = response.json::<serde_json::Value>().await?;
        Ok(content["content"].as_str().unwrap().to_string())
    }

    pub async fn mark_as_read(&self, entry_id: u64) -> Result<(), Box<dyn std::error::Error>> {
        self.mark_as_read_some(vec![entry_id]).await
    }

    pub async fn mark_as_read_some(&self, entry_ids: Vec<u64>) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("https://{}/v1/entries", self.url);
        let client = Client::new();
        let data = Data {
            entry_ids,
            status: "read".to_string(),
        };
        debug!("Data: {:?}", data);
        let response = client
            .put(&url)
            .header("X-Auth-Token", &self.token)
            .json(&data)
            .send()
            .await?;
        debug!("================");
        debug!("Response: {:?}", response);
        debug!("================");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::MinifluxClient;
    use dotenv::dotenv;
    use tracing::debug;

    #[tokio::test]
    async fn read_entries() {
        dotenv().ok();
        let miniflux = MinifluxClient::new(
            std::env::var("MINIFLUX_URL").expect("MINIFLUX_URL is mandatory"),
            std::env::var("MINIFLUX_TOKEN").expect("MINIFLUX_TOKEN is mandatory"),
        );
        let entries = miniflux.get_entries().await;
        println!("Entries: {:?}", entries);
        debug!("Entries: {:?}", entries);
        assert!(entries.is_ok());
    }

    #[tokio::test]
    async fn read_categories() {
        dotenv().ok();
        let miniflux = MinifluxClient::new(
            std::env::var("MINIFLUX_URL").expect("MINIFLUX_URL is mandatory"),
            std::env::var("MINIFLUX_TOKEN").expect("MINIFLUX_TOKEN is mandatory"),
        );
        let categories = miniflux.get_categories().await;
        println!("Categories: {:?}", categories);
        debug!("Categories: {:?}", categories);
        assert!(categories.is_ok());
    }

    #[tokio::test]
    async fn read_category_entries() {
        dotenv().ok();
        let miniflux = MinifluxClient::new(
            std::env::var("MINIFLUX_URL").expect("MINIFLUX_URL is mandatory"),
            std::env::var("MINIFLUX_TOKEN").expect("MINIFLUX_TOKEN is mandatory"),
        );
        let categories = miniflux.get_categories().await;
        println!("Categories: {:?}", categories);
        let category_id = categories.unwrap().first().unwrap().as_object().unwrap().get("id").unwrap().as_i64().unwrap() as i32;
        let entries = miniflux.get_category_entries(category_id).await;
        println!("Entries: {:?}", entries);
        debug!("Entries: {:?}", entries);
        assert!(entries.is_ok());
    }

}

