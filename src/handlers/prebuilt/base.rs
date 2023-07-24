use std::collections::HashMap;

use crate::{handlers::service::setup_services, Service};


pub fn setup_base(params: &HashMap<String, String>, supervisor_conf: &mut String, image_dockerfile: &mut String) {
    let base_services: Vec<Service> = serde_json::from_str(include_str!("../../config/prebuilt/base.json")).unwrap();

    setup_services(&base_services, params, supervisor_conf, image_dockerfile);
}