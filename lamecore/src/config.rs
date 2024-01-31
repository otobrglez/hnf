const HN_API_BASE: &'static str = "https://hacker-news.firebaseio.com/v0";

pub fn hn_api_url(path: &str) -> String {
    format!("{}{}", HN_API_BASE, path).to_string()
}
