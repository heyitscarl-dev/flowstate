use flowstate::config::load_config;
use flowstate::model::host::Host;
use std::env;

#[test]
fn test_config_load_and_count() {
    unsafe {
        env::set_var("FLOW_CONFIG", "tests/test_config.yaml");
    }

    let config = load_config().expect("Failed to load test config");

    assert_eq!(config.hosts.len(), 2);

    // Check first host
    match &config.hosts[0] {
        Host::HTTP(http_host) => {
            assert_eq!(http_host.label, "Test Host 1");
            assert_eq!(http_host.url, "http://example.com");
            assert!(http_host.status.is_none());
            assert!(http_host.regex.is_none());
        }
    }

    // Check second host with optional fields
    match &config.hosts[1] {
        Host::HTTP(http_host) => {
            assert_eq!(http_host.label, "Test Host 2");
            assert_eq!(http_host.url, "http://example.org");
            assert_eq!(http_host.status.unwrap().as_u16(), 200);
            assert!(http_host.regex.is_some());
        }
    }
}
