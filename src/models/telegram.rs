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
        let params = vec![
            ("chat_id", self.chat_id.as_str()),
            ("message_thread_id", self.thread_id.as_str()),
            ("text", message),
            ("parse_mode", "MarkdownV2"),
        ];
        Client::new()
            .post(url)
            .form(&params)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
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
    }
}


