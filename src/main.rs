use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use serde::Deserialize;

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
    url: String, // use "" for default
    caddyfile: String,
}

#[derive(Deserialize)]
struct Config {
    caddy: CaddyConfig, // relative to the volume
    service_commands: Vec<Service>,
    params: HashMap<String, String>,
}

pub mod handlers;

fn main() {
    let args = Args::parse();
    let raw_config = fs::read_to_string(args.config.as_str()).unwrap();
    let mut config: Config = serde_json::from_str(raw_config.as_str()).unwrap();
    let mut supervisor_conf: String = include_str!("./assets/enclave/supervisord.conf").to_string();
    let mut image_dockerfile: String = include_str!("./assets/enclave/Dockerfile").to_string();
    let entrypoint: String = include_str!("./assets/enclave/entrypoint.sh").to_string();

    if !config.params.get("ARCH").is_none() {
        panic!("Enclave-Builder: ARCH is a reserved parameter and cannot be set in the config file");
    } else {
        config.params.insert("ARCH".to_string(), args.arch);
    }

    crate::handlers::prebuilt::base::setup_base(&config.params, &mut supervisor_conf, &mut image_dockerfile);

    if config.caddy.caddyfile != "" {
        crate::handlers::prebuilt::caddy::setup_domain(config.caddy, &config.params, &mut supervisor_conf, &mut image_dockerfile);
    }

    crate::handlers::service::setup_services(&config.service_commands, &config.params, &mut supervisor_conf, &mut image_dockerfile);

    // TODO: move path to defaults config
    let base_dir: PathBuf = PathBuf::from("/app");
    fs::write(&base_dir.join("Dockerfile"), &image_dockerfile).unwrap();

    let assets_path: PathBuf = base_dir.join("assets");
    fs::create_dir_all(&assets_path).unwrap();
    fs::write(&assets_path.join("supervisord.conf"), &supervisor_conf).unwrap();
    fs::write(&assets_path.join("entrypoint.sh"), &entrypoint).unwrap();

    println!("Enclave-Builder: Service setup complete to build enclave");
}