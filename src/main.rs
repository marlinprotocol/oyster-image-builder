use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the config file
    #[arg(short, long)]
    config: String,
    /// Architecture to build for
    #[arg(short, long)]
    arch: String,
}

#[derive(Deserialize)]
pub struct Service {
    name: String,
    command: String,
    build_commands: Vec<String>, // run during image creation from copied volume
    ports: Vec<u32>,
    env: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct CaddyConfig {
    url: Option<String>,
    caddyfile: Option<String>,
}

#[derive(Deserialize)]
struct Config {
    caddy: CaddyConfig, // relative to the volume
    service_commands: Vec<Service>,
    params: HashMap<String, String>,
}

pub mod handlers;

fn main() {
    let args: Args = Args::parse();
    let raw_config = fs::read_to_string(args.config.as_str()).unwrap();
    let mut json_config: Value =
        serde_json::from_str(&raw_config).expect("Failed to parse raw config");

    set_default_values(&mut json_config);

    let mut config: Config =
        serde_json::from_value(json_config).expect("Failed to deserialize JSON Config");

    let mut supervisor_conf: String = include_str!("./assets/enclave/supervisord.conf").to_string();
    let mut image_dockerfile: String = include_str!("./assets/enclave/Dockerfile").to_string();
    let entrypoint: String = include_str!("./assets/enclave/entrypoint.sh").to_string();

    if !config.params.get("ARCH").is_none() {
        panic!(
            "Enclave-Builder: ARCH is a reserved parameter and cannot be set in the config file"
        );
    } else {
        config.params.insert("ARCH".to_string(), args.arch);
    }

    crate::handlers::prebuilt::base::setup_base(
        &config.params,
        &mut supervisor_conf,
        &mut image_dockerfile,
    );

    if !config.caddy.caddyfile.is_none()
        && config.caddy.caddyfile.as_ref().unwrap().to_string() != ""
    {
        crate::handlers::prebuilt::caddy::setup_domain(
            config.caddy,
            &config.params,
            &mut supervisor_conf,
            &mut image_dockerfile,
        );
    }

    crate::handlers::service::setup_services(
        &config.service_commands,
        &config.params,
        &mut supervisor_conf,
        &mut image_dockerfile,
    );

    // TODO: move path to defaults config
    let base_dir: PathBuf = PathBuf::from("/app");
    fs::write(&base_dir.join("Dockerfile"), &image_dockerfile).unwrap();

    let assets_path: PathBuf = base_dir.join("assets");
    fs::create_dir_all(&assets_path).unwrap();
    fs::write(&assets_path.join("supervisord.conf"), &supervisor_conf).unwrap();
    fs::write(&assets_path.join("entrypoint.sh"), &entrypoint).unwrap();

    println!("Enclave-Builder: Service setup complete to build enclave");
}

fn set_default_values(json_config: &mut Value) {
    fn ensure_field_exists(
        json_obj: &mut serde_json::Map<String, Value>,
        field: &str,
        default: Value,
    ) {
        if json_obj.get(field).is_none() {
            json_obj.insert(field.to_string(), default);
        }
    }

    let json_obj = json_config.as_object_mut().unwrap();

    ensure_field_exists(json_obj, "caddy", json!({}));
    ensure_field_exists(json_obj, "params", json!({}));
    ensure_field_exists(json_obj, "service_commands", json!([]));

    if let Some(services) = json_obj
        .get_mut("service_commands")
        .and_then(|v| v.as_array_mut())
    {
        for service in services {
            let service_obj = service.as_object_mut().unwrap();
            ensure_field_exists(service_obj, "ports", json!([]));
            ensure_field_exists(service_obj, "env", json!({}));
        }
    }
}
