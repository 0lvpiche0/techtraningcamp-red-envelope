use std::env;
pub fn get_env(env: &str, default: &str) -> String {
    match env::var(env) {
        Ok(s) => s,
        Err(_) => default.to_string(),
    }
}