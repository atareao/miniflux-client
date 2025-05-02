mod models;

use models::{MatrixClient, MinifluxClient};
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
    loop {
        if let Err(e) = miniflux.refresh_all_feeds().await {
            error!("Error: {}", e);
        }
        let entries = miniflux.get_entries().await;
        match entries {
            Ok(entries) => {
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
                    let content = miniflux.get_content(id).await.unwrap_or("".to_string());
                    let message = format!("<h3><a href=\"{url}\">{title}</a></h3><ul><li>{feed_title}</li><li>{published_at}</li><li>{author}</li></ul><details><summary>{resume}</summary>{content}</details><hr>");
                    if let Ok(response) = matrix.post(&message).await {
                        debug!("Response: {:?}", response);
                        if let Err(response) = miniflux.mark_as_read(id).await {
                            error!("Error: {}", response);
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
