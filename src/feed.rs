use feed_rs::{model::Feed, parser};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize, Clone)]
pub struct CachedFeed {
    pub url: String,
    pub feed: Feed,
}

pub async fn get(
    feed_path: &str,
) -> Result<(Vec<Feed>, Vec<String>), std::io::Error> {
    let urls = get_urls(feed_path)?;
    let mut tasks = vec![];
    for url in urls.clone() {
        tasks.push(tokio::spawn(fetch_feed(url.clone())));
    }
    let mut feeds = vec![];
    let mut successful_urls = vec![];
    for (task, url) in tasks.into_iter().zip(urls.iter()) {
        if let Ok(Ok(feed)) = task.await {
            feeds.push(feed);
            successful_urls.push(url.clone());
        }
    }
    Ok((feeds, successful_urls))
}

fn get_urls(feed_path: &str) -> Result<Vec<String>, std::io::Error> {
    match std::fs::read_to_string(feed_path) {
        Ok(content) => {
            let feed_urls = content
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect();
            Ok(feed_urls)
        }
        Err(e) => Err(e),
    }
}

async fn fetch_feed(url: String) -> Result<Feed, std::io::Error> {
    let response = reqwest::get(&url).await.map_err(io::Error::other)?;
    let body = response
        .text()
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    parser::parse(body.as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

const CACHE_PREFIX: &str = "com.benmandrew.remy";

fn get_cache_path() -> Result<std::path::PathBuf, std::io::Error> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(CACHE_PREFIX);
    xdg_dirs.find_cache_file("feed_cache.json").ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "cache not found")
    })
}

pub async fn load_cached_feeds() -> Result<Vec<Feed>, std::io::Error> {
    let cache_path = get_cache_path()?;
    let content = tokio::fs::read_to_string(cache_path).await?;
    let cached: Vec<CachedFeed> = serde_json::from_str(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(cached.into_iter().map(|c| c.feed).collect())
}

pub async fn save_cached_feeds(
    feeds: &[Feed],
    urls: &[String],
) -> Result<(), std::io::Error> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(CACHE_PREFIX);
    let cache_path = xdg_dirs
        .place_cache_file("feed_cache.json")
        .map_err(io::Error::other)?;
    let cached: Vec<CachedFeed> = feeds
        .iter()
        .zip(urls.iter())
        .map(|(feed, url)| CachedFeed {
            url: url.clone(),
            feed: feed.clone(),
        })
        .collect();
    let json = serde_json::to_string_pretty(&cached)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let temp_path = cache_path.with_extension("tmp");
    tokio::fs::write(&temp_path, json).await?;
    tokio::fs::rename(temp_path, cache_path).await?;
    Ok(())
}
