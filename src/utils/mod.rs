use std::env;
pub fn get_env(env: &str, default: &str) -> String {
    match env::var(env) {
        Ok(s) => s,
        Err(_) => default.to_string(),
    }
}

pub fn get_args(env: &str) -> Vec<String> {
    env.split(';')
    .map(|s| s.to_string())
    .collect()
}