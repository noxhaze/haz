use crate::config::Config;

#[test]
fn creation() {
    let config: std::io::Result<Config> = Config::new("test/test.haz");
    assert!(config.is_ok());
}
