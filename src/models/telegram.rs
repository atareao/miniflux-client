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
        Client::new()
            .post(url)
            .json(&payload)
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
        let message = "*[atareao.es](https://atareao.es)*\nOrigen\n\n";
        assert!(telegram.send_message(message).await.is_ok());
    }
}


