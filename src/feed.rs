use curl::easy::Easy;
use feed_rs::{model::Feed, parser};
use std::io;

pub fn get(feed_path: &str) -> Result<Vec<Feed>, std::io::Error> {
    get_urls(feed_path)?
        .iter()
        .map(|url| fetch_feed(url))
        .collect()
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

fn fetch_feed(url: &str) -> Result<Feed, std::io::Error> {
    let mut feed_content = Vec::new();
    let mut easy = Easy::new();
    easy.url(url)?;
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            feed_content.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }
    let contents =
        String::from_utf8(feed_content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e));
    parser::parse(contents?.as_bytes()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
