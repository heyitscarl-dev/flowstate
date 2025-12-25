use flowstate::config::load_config;
use std::env;

#[test]
fn test_config_load_and_count() {
    unsafe {
        env::set_var("FLOW_CONFIG", "tests/test_config.yaml");
    }

    let config = load_config().expect("Failed to load test config");

    assert_eq!(config.hosts.len(), 2);
    assert_eq!(config.hosts[0].label, "Test Host 1");
    assert_eq!(config.hosts[0].on, "http://example.com");
    assert_eq!(config.hosts[1].label, "Test Host 2");
    assert_eq!(config.hosts[1].on, "http://example.org");
}
