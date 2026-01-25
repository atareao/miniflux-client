use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use urlencoding::encode;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, debug};
use reqwest::{Client, header::{HeaderMap, HeaderValue,
    HeaderName}};
use super::CustomError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatrixClient{
    server: String,
    token: String,
    room: String,
}

impl MatrixClient {

    pub fn new(server: String, token: String, room: String) -> Self{
        MatrixClient{
            server,
            token,
            room,
        }
    }

    pub async fn post(&self, message: &str) -> Result<String, CustomError>{
        info!("post_with_matrix");
        debug!("Post with matrix: {}", message);
        let url = format!(
            "https://{}/_matrix/client/v3/rooms/{}:{}/send/m.room.message/{}",
            self.server,
            encode(&self.room),
            self.server,
            Self::ts(),
        );
        debug!("Url: {}", url);
        let body = json!({
            "msgtype": "m.text",
            "format": "org.matrix.custom.html",
            "body": message,
            "formatted_body": message,
        });
        debug!("Body: {}", body);
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.append(HeaderName::from_str("Authorization").unwrap(),
                          HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap());
        debug!("Header: {:?}", header_map);
        Self::_put(&url, header_map, &body)
            .await
    }

    async fn _put(url: &str, header_map: HeaderMap, body: &Value) -> Result<String, CustomError>{
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        let content = serde_json::to_string(body).unwrap();
        let response = client.put(url).body(content).send()
            .await?;
        
        let status = response.status();
        let response_body = response.text().await?;
        
        if !status.is_success() {
            debug!("Matrix API error - Status: {}, Body: {}", status, response_body);
        } else {
            debug!("Matrix message sent successfully");
        }
        
        Ok(response_body)
    }

    fn ts() -> f64{
        debug!("ts");
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap() .as_secs_f64()
    }
}

#[cfg(test)]
mod test {
    use super::MatrixClient;
    use dotenv::dotenv;
    use tracing_subscriber::{
        EnvFilter,
        layer::SubscriberExt,
        util::SubscriberInitExt
    };

    #[tokio::test]
    async fn post() {
        dotenv().ok();
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .init();
        let matrix = MatrixClient::new(
            std::env::var("MATRIX_URL").expect("MATRIX_URL is mandatory"),
            std::env::var("MATRIX_TOKEN").expect("MATRIX_TOKEN is mandatory"),
            std::env::var("MATRIX_ROOM").expect("MATRIX_ROOM is mandatory"),
        );
        let response = matrix.post("Hola mundo").await;
        println!("Response: {:?}", response);
        let message = "<h3>Ejemplo</h3><details><summary>Organizadores del festival destrozados</summary><p>Una rampa de coche mató a 11 personas en un festival de Vancouver, dejando a la comunidad en profundo dolor</p></details>";
        let response = matrix.post(message).await;
        println!("Response: {:?}", response);
    }

    #[test]
    fn test_matrix_client_creation() {
        let server = "matrix.example.com".to_string();
        let token = "test_token".to_string();
        let room = "test_room".to_string();
        let client = MatrixClient::new(server.clone(), token.clone(), room.clone());
        assert_eq!(client.server, server);
        assert_eq!(client.token, token);
        assert_eq!(client.room, room);
    }

    #[test]
    fn test_matrix_client_clone() {
        let client = MatrixClient::new(
            "matrix.example.com".to_string(),
            "token".to_string(),
            "room".to_string(),
        );
        let cloned = client.clone();
        assert_eq!(client.server, cloned.server);
        assert_eq!(client.token, cloned.token);
        assert_eq!(client.room, cloned.room);
    }

    #[test]
    fn test_matrix_client_serialize() {
        let client = MatrixClient::new(
            "matrix.example.com".to_string(),
            "token123".to_string(),
            "myroom".to_string(),
        );
        let serialized = serde_json::to_string(&client).unwrap();
        assert!(serialized.contains("matrix.example.com"));
        assert!(serialized.contains("token123"));
        assert!(serialized.contains("myroom"));
    }

    #[test]
    fn test_matrix_client_deserialize() {
        let json = r#"{"server":"matrix.example.com","token":"token123","room":"myroom"}"#;
        let client: MatrixClient = serde_json::from_str(json).unwrap();
        assert_eq!(client.server, "matrix.example.com");
        assert_eq!(client.token, "token123");
        assert_eq!(client.room, "myroom");
    }

    #[test]
    fn test_ts_returns_positive_value() {
        let ts = MatrixClient::ts();
        assert!(ts > 0.0);
    }

    #[test]
    fn test_ts_returns_different_values() {
        let ts1 = MatrixClient::ts();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let ts2 = MatrixClient::ts();
        assert!(ts2 >= ts1);
    }
}

