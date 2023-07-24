use std::collections::HashMap;

use crate::{handlers::service::setup_services, Service};

pub fn setup_open_ports(open_ports: &Vec<u32>, params: &HashMap<String, String>, supervisor_conf: &mut String) {
    let port_services: Vec<Service> = serde_json::from_str(include_str!("../../config/prebuilt/ports.json")).unwrap();

    for port in open_ports {
        let mut port_params = params.clone();
        port_params.insert("port".to_string(), port.to_string());
        setup_services(&port_services, &port_params, supervisor_conf, &mut String::new());
    }
}