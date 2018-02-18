//! Detect what CI this code is running on and extract the information it provides from env vars.

pub mod providers;

fn env(var: &str) -> Option<String> {
    let env_var = std::env::var(var).unwrap_or_default();
    if !env_var.is_empty() {
        Some(env_var)
    } else {
        None
    }
}
