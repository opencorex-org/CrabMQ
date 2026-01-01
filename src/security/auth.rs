#![allow(dead_code)]
pub fn validate(username: &str, password: &str) -> bool {
    // Very basic placeholder: accept any non-empty user/pass
    !(username.is_empty() || password.is_empty())
}
