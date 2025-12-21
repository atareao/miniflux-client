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
        Ok(client.put(url).body(content).send()
            .await?
            .text()
            .await?)
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
    use tracing::debug;

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
        let message = "<h3>Noticias Internacionales</h3><ul><li>Organizadores del festival destrozados: Una rampa de coche mató a 11 personas en un festival de Vancouver, dejando a la comunidad en profundo dolor.</li><li>Juicio a los \"ladrones abuelos\": Sospechosos acusados de robar a Kim Kardashian a punta de pistola en París en octubre de 2016, quienes sorprendentemente ni siquiera sabían quién era ella.</li><li>Elecciones en el Ártico canadiense: Nunavut presenta desafíos únicos para realizar elecciones debido a su cultura, paisaje y clima extremo.</li><li>Los primeros 100 días de Trump: Un análisis sobre cómo las acciones del presidente están transformando rápidamente diversos aspectos de la vida estadounidense.</li></ul>";
        let response = matrix.post(message).await;
        debug!("Response: {:?}", response);
        println!("Response: {:?}", response);
        assert!(response.is_ok());
    }
}

