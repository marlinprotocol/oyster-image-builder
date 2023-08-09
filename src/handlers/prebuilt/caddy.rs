use std::collections::HashMap;

use crate::{CaddyConfig, handlers::service::setup_services, Service};

pub fn setup_domain(caddy_config: CaddyConfig, params: &HashMap<String, String>, supervisor_conf: &mut String, image_dockerfile: &mut String) {
    let domain_services: Vec<Service> = serde_json::from_str(include_str!("../../config/prebuilt/caddy.json")).unwrap();

    let mut caddy_params = params.clone();
    if caddy_config.url == "" {
        caddy_params.insert(
            "caddy.url".to_string(), 
            format!("https://caddyserver.com/api/download?os=linux&arch={arch}", arch=params["ARCH"])
        );
    } else {
        caddy_params.insert("caddy.url".to_string(), caddy_config.url.to_string());
    }
    caddy_params.insert("caddy.caddyfile".to_string(), caddy_config.caddyfile.to_string());
    setup_services(&domain_services, &caddy_params, supervisor_conf, image_dockerfile);
}