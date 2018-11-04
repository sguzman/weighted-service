pub fn env(key: &str, default: &str) -> String {
    return match std::env::var(key) {
        Ok(lang) => lang,
        Err(_) => format!("{}", default)
    };
}