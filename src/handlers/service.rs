use std::collections::HashMap;

use crate::Service;

use super::prebuilt::ports::setup_open_ports;

pub fn setup_services(services: &Vec<Service>, params: &HashMap<String, String>, supervisor_conf: &mut String, image_dockerfile: &mut String) {
    for service in services {
        setup_service(service, params, supervisor_conf, image_dockerfile);
    }
}

fn setup_service(service: &Service, params: &HashMap<String, String>, supervisor_conf: &mut String, image_dockerfile: &mut String) {
    let mut service_params = params.clone();
    if service_params.get("service_name").is_none() {
        service_params.insert("service_name".to_string(), service.name.clone());
    }
    add_build_commands(image_dockerfile, &service_params, &service.build_commands);

    update_supervisord_conf(supervisor_conf, &service_params, &service.name, &service.command);

    add_env(image_dockerfile, &service.env);

    setup_open_ports(&service.ports, &service_params, supervisor_conf)
}

fn update_supervisord_conf(supervisor_conf: &mut String, params: &HashMap<String, String>, name: &String, command: &String) {
    if command == "" {
        return;
    }

    let service_template = replace_params(
        &format!(include_str!("../assets/templates/service.template"), service_name=name, command=command), 
        params
    );
    supervisor_conf.push_str(service_template.as_str());
}

fn add_env(image_dockerfile: &mut String, env: &HashMap<String, String>) {
    if env.len() == 0 {
        return;
    }

    for (key, value) in env {
        image_dockerfile.push_str(&format!("\nENV {} {}", key, value));
    }
    image_dockerfile.push_str(&format!("\n"));
}

fn add_build_commands(image_dockerfile: &mut String, params: &HashMap<String, String>, commands: &Vec<String>) {
    for command in commands {
        let command_template: String = replace_params(&format!("\nRUN {}", command), params);
        image_dockerfile.push_str(command_template.as_str());
    }
    image_dockerfile.push_str(&format!("\n"));
}

fn replace_params(template: &String, params: &HashMap<String, String>) -> String {
    let mut result = template.clone();
    for (param, value) in params {
        result = result.replace(&format!("{{{}}}", param), value);  
    }
    result
}