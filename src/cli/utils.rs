pub fn current_dir() -> String {
    match std::env::current_dir() {
        Ok(path) => path.to_str().unwrap_or_else(|| {
            "."
        }).to_string(),
        Err(_) => panic!("Unable to get current directory."),
    }
}
