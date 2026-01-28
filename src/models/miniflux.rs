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
        
        let status = response.status();
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error body".to_string());
            debug!("Miniflux API error (get_categories) - Status: {}, Body: {}", status, error_body);
            return Err(format!("Miniflux API error: {}", error_body).into());
        }
        
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
        
        let status = response.status();
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error body".to_string());
            debug!("Miniflux API error (get_category_entries) - Status: {}, Body: {}", status, error_body);
            return Err(format!("Miniflux API error: {}", error_body).into());
        }
        
        let content = response.json::<serde_json::Value>().await?;
        Ok(content["entries"].as_array().unwrap().to_vec())
    }

    pub async fn get_entries(&self, limit: usize) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let url = format!("https://{}/v1/entries", self.url);
        let client = Client::new();
        let response = client
            .get(&url)
            .query(&[
                ("status", "unread"),
                ("limit", &limit.to_string()),
                ])
            .header("X-Auth-Token", &self.token)
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error body".to_string());
            debug!("Miniflux API error (get_entries) - Status: {}, Body: {}", status, error_body);
            return Err(format!("Miniflux API error: {}", error_body).into());
        }
        
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
        
        let status = response.status();
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error body".to_string());
            debug!("Miniflux API error (refresh_all_feeds) - Status: {}, Body: {}", status, error_body);
            return Err(format!("Miniflux API error: {}", error_body).into());
        } else {
            debug!("All feeds refreshed successfully");
        }
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
        
        let status = response.status();
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error body".to_string());
            debug!("Miniflux API error (get_content) - Status: {}, Body: {}", status, error_body);
            return Err(format!("Miniflux API error: {}", error_body).into());
        }
        
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
        debug!("Marking entries as read: {:?}", data);
        let response = client
            .put(&url)
            .header("X-Auth-Token", &self.token)
            .json(&data)
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error body".to_string());
            debug!("Miniflux API error (mark_as_read) - Status: {}, Body: {}", status, error_body);
            return Err(format!("Miniflux API error: {}", error_body).into());
        } else {
            debug!("Entries marked as read successfully");
        }
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

    #[test]
    fn test_miniflux_client_creation() {
        let url = "example.com".to_string();
        let token = "test_token".to_string();
        let client = MinifluxClient::new(url.clone(), token.clone());
        assert_eq!(client.url, url);
        assert_eq!(client.token, token);
    }

    #[test]
    fn test_miniflux_client_clone() {
        let client = MinifluxClient::new("example.com".to_string(), "token".to_string());
        let cloned = client.clone();
        assert_eq!(client.url, cloned.url);
        assert_eq!(client.token, cloned.token);
    }

    #[test]
    fn test_miniflux_client_serialize() {
        let client = MinifluxClient::new("example.com".to_string(), "token123".to_string());
        let serialized = serde_json::to_string(&client).unwrap();
        assert!(serialized.contains("example.com"));
        assert!(serialized.contains("token123"));
    }

    #[test]
    fn test_miniflux_client_deserialize() {
        let json = r#"{"url":"example.com","token":"token123"}"#;
        let client: MinifluxClient = serde_json::from_str(json).unwrap();
        assert_eq!(client.url, "example.com");
        assert_eq!(client.token, "token123");
    }

}

