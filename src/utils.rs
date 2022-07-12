
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use axum::http::{HeaderMap, header};

use crate::murmur::murmur_hash3_x86_32;


pub fn u32_to_b62(hash: u32) -> String {
    // let dict = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let dict = "deDRSCNOPQXbIJqLMghY1K0lyair67zUVWfZuv5jTABkwxmn894stGHop23cEF";
    let mut n = hash;
    let mut chars: Vec<char> = vec![];
    while n > 0 {
        let i = (n % 62) as usize;
        let c = dict.chars().nth(i).unwrap();
        chars.push(c);
        n /= 62;
    }
    chars.reverse();
    chars.into_iter().collect::<String>()
}

pub fn short_url(url: &str) -> String {
    let hash = murmur_hash3_x86_32(url.as_ptr(), url.len(), 1234);
    u32_to_b62(hash)
}

pub fn get_cors_header(mut headers: HeaderMap) -> HeaderMap {
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, "*".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_MAX_AGE, "3600".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "*".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true".parse().unwrap());
    headers
}

pub fn get_timestamp() -> String {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_short_url() {
        assert_eq!("qy0nm".to_string(), short_url("https://silente.dev"));
    }
}