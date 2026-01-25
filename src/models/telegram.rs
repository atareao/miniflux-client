use reqwest::Client;
use serde::{Serialize, Deserialize};
use tracing::debug;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TelegramClient{
    token: String,
    chat_id: String,
    #[serde(default = "default_thread_id")]
    thread_id: String,
}

#[derive(Serialize)]
struct TelegramMessage {
    message_thread_id: String,
    chat_id: String,
    text: String,
    parse_mode: String,
}

fn default_thread_id() -> String{
    "0".to_string()
}

const URL: &str = "https://api.telegram.org";

impl TelegramClient{
    pub fn new(token: String, chat_id: String, thread_id: String) -> Self{
        Self{
            token,
            chat_id,
            thread_id,
        }
    }

    pub async fn send_message(&self, message: &str) -> Result<String, reqwest::Error>{
        debug!("Sending Telegram message: {}", message);
        let url = format!("{URL}/bot{}/sendMessage", self.token);
        let payload = TelegramMessage{
            message_thread_id: self.thread_id.clone(),
            chat_id: self.chat_id.clone(),
            text: message.into(),
            parse_mode: "MarkdownV2".into(),
        };
        let response = Client::new()
            .post(&url)
            .json(&payload)
            .send()
            .await?;
        
        let status = response.status();
        let body = response.text().await?;
        
        if !status.is_success() {
            debug!("Telegram API error - Status: {}, Body: {}", status, body);
        } else {
            debug!("Telegram message sent successfully");
        }
        
        Ok(body)
    }
}



#[cfg(test)]
mod test{
    use super::TelegramClient;
    use dotenv::dotenv;
    use std::{env, str::FromStr};
    use tracing_subscriber::{
        EnvFilter,
        layer::SubscriberExt,
        util::SubscriberInitExt
    };

    #[tokio::test]
    async fn telegram(){
        tracing_subscriber::registry()
            .with(EnvFilter::from_str("debug").unwrap())
            .with(tracing_subscriber::fmt::layer())
        .init();
        dotenv().ok();
        let token = env::var("TELEGRAM_TOKEN")
            .expect("Cant get token");
        let chat_id = env::var("TELEGRAM_CHAT_ID")
            .expect("Cant get chat_id")
            .parse()
            .expect("Cant convert chat_id");
        let thread_id = env::var("TELEGRAM_THREAD_ID")
            .expect("Cant get thread_id")
            .parse()
            .expect("Cant convert thread_id");
        let telegram = TelegramClient::new(token, chat_id, thread_id);
        assert!(telegram.send_message("Prueba").await.is_ok());
        let message = "*[atareao.es](https://atareao.es)*\nOrigen\n\n";
        assert!(telegram.send_message(message).await.is_ok());
    }

    #[test]
    fn test_telegram_client_creation() {
        let token = "test_token".to_string();
        let chat_id = "12345".to_string();
        let thread_id = "67890".to_string();
        let client = TelegramClient::new(token.clone(), chat_id.clone(), thread_id.clone());
        assert_eq!(client.token, token);
        assert_eq!(client.chat_id, chat_id);
        assert_eq!(client.thread_id, thread_id);
    }

    #[test]
    fn test_telegram_client_with_default_thread_id() {
        let json = r#"{"token":"test_token","chat_id":"12345"}"#;
        let client: TelegramClient = serde_json::from_str(json).unwrap();
        assert_eq!(client.thread_id, "0");
    }

    #[test]
    fn test_telegram_client_clone() {
        let client = TelegramClient::new(
            "token".to_string(),
            "12345".to_string(),
            "67890".to_string(),
        );
        let cloned = client.clone();
        assert_eq!(client.token, cloned.token);
        assert_eq!(client.chat_id, cloned.chat_id);
        assert_eq!(client.thread_id, cloned.thread_id);
    }

    #[test]
    fn test_telegram_client_serialize() {
        let client = TelegramClient::new(
            "token123".to_string(),
            "chat456".to_string(),
            "thread789".to_string(),
        );
        let serialized = serde_json::to_string(&client).unwrap();
        assert!(serialized.contains("token123"));
        assert!(serialized.contains("chat456"));
        assert!(serialized.contains("thread789"));
    }

    #[test]
    fn test_telegram_client_deserialize() {
        let json = r#"{"token":"token123","chat_id":"chat456","thread_id":"thread789"}"#;
        let client: TelegramClient = serde_json::from_str(json).unwrap();
        assert_eq!(client.token, "token123");
        assert_eq!(client.chat_id, "chat456");
        assert_eq!(client.thread_id, "thread789");
    }
}


