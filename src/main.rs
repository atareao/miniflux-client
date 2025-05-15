mod models;

use models::{MatrixClient, MinifluxClient, Model};
use serde_json::{json, Value};
use std::{env, time};
use tracing::{debug, error, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    info!("Start");
    let sleep_time = time::Duration::from_secs(
        env::var("SLEEP_TIME")
            .unwrap_or_else(|_| "1800".to_string())
            .parse::<u64>()
            .unwrap_or(1800),
    );
    let miniflux = MinifluxClient::new(
        env::var("MINIFLUX_URL").expect("MINIFLUX_URL is mandatory"),
        env::var("MINIFLUX_TOKEN").expect("MINIFLUX_TOKEN is mandatory"),
    );
    let matrix = MatrixClient::new(
        env::var("MATRIX_URL").expect("MATRIX_URL is mandatory"),
        env::var("MATRIX_TOKEN").expect("MATRIX_TOKEN is mandatory"),
        env::var("MATRIX_ROOM").expect("MATRIX_ROOM is mandatory"),
    );
    let model = Model::new(
        std::env::var("MODEL_URL").expect("MODEL_URL is mandatory"),
        std::env::var("MODEL_API_KEY").expect("MODEL_API_KEY is mandatory"),
        std::env::var("MODEL_NAME").expect("MODEL_NAME is mandatory"),
        std::env::var("MODEL_VERSION").expect("MODEL_VERSION is mandatory"),
        std::env::var("MODEL_DESCRIPTION").expect("MODEL_DESCRIPTION is mandatory"),
        std::env::var("MODEL_PROMPT").expect("MODEL_PROMPT is mandatory"),
        std::env::var("MAX_TOKENS")
            .expect("MAX_TOKENS is mandatory")
            .parse::<u32>()
            .unwrap(),
    );
    loop {
        if let Err(e) = miniflux.refresh_all_feeds().await {
            error!("Error: {}", e);
        }
        let entries = miniflux.get_entries().await;
        match entries {
            Ok(entries) => {
                let mut news = Vec::new();
                for entry in entries.as_slice() {
                    debug!("Entry: {}", entry);
                    let id = entry["id"].as_u64().unwrap_or(0);
                    let title = entry["title"].as_str().unwrap_or("No title");
                    let url = entry["url"].as_str().unwrap_or("No URL");
                    let resume = entry["content"].as_str().unwrap_or("No content");
                    let author = entry["author"].as_str().unwrap_or("No author");
                    let feed = entry["feed"].as_object().unwrap();
                    let feed_title = feed["title"].as_str().unwrap_or("No feed title");
                    let published_at = entry["published_at"].as_str().unwrap_or("No published_at");
                    news.push(json!({
                        "url": url,
                        "title": title,
                        "feed_title": feed_title,
                        "published_at": published_at,
                        "author": author,
                        "resume": resume,
                    }));
                    if let Err(response) = miniflux.mark_as_read(id).await {
                        error!("Error: {}", response);
                    }
                }
                if news.is_empty() {
                    info!("No new entries");
                    match matrix.post("No hay nuevas noticias").await {
                        Ok(response) => {
                            debug!("Response: {:?}", response);
                        }
                        Err(e) => {
                            error!("Error: {}", e);
                        }
                    }
                }else{
                    match model.process_news(&news).await{
                        Ok(message) => {
                            debug!("Message: {:?}", message);
                            match serde_json::from_str::<Value>(&message) {
                                Ok(value) => {
                                    debug!("Value: {:?}", value);
                                    let news = value.get("news")
                                        .and_then(|v| v.as_array())
                                        .unwrap_or(&vec![])
                                        .iter()
                                        .map(|v| format!(
                                            "<h3><a href=\"{}\">{}</a></h3><p>{}</p><br>",
                                            v.get("url").unwrap().as_str().unwrap_or(""),
                                            v.get("title").unwrap().as_str().unwrap_or(""),
                                            v.get("summary").unwrap().as_str().unwrap_or("")
                                        )).collect::<Vec<_>>()
                                        .join("");
                                        match matrix.post(&news).await {
                                            Ok(response) => {
                                                debug!("Response: {:?}", response);
                                            }
                                            Err(e) => {
                                                error!("Error: {}", e);
                                            }
                                        }
                                },
                                Err(e) => {
                                    error!("Error: {}", e);
                                }
                            }
                        },
                        Err(e) => {
                            error!("Error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
        info!("Sleeping for {:?} seconds", sleep_time);
        tokio::time::sleep(sleep_time).await;
    }
}
