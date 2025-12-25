pub fn get(feed_path: &str) -> Result<Vec<String>, std::io::Error> {
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
