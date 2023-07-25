use std::collections::HashMap;
use std::fs;
use std::path::{PathBuf, Path};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    config: String,
}

#[derive(Serialize, Deserialize)]
pub struct Service {
    name: String,
    command: String,
    build_commands: Vec<String>, // run during image creation from copied volume
    ports: Vec<u32>,
    env: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct CaddyConfig {
    url: String, // use "" for default
    caddyfile: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    caddy: CaddyConfig, // relative to the volume
    service_commands: Vec<Service>,
    // TODO: add validations on what is supported
    params: HashMap<String, String>, // amd64, arm64
}

pub mod handlers;

fn main() {
    let args = Args::parse();
    let raw_config = fs::read_to_string(args.config.as_str()).unwrap();
    let config: Config = serde_json::from_str(raw_config.as_str()).unwrap();
    let mut supervisor_conf: String = include_str!("./assets/enclave/supervisord.conf").to_string();
    let mut image_dockerfile: String = include_str!("./assets/enclave/Dockerfile").to_string();
    let entrypoint: String = include_str!("./assets/enclave/entrypoint.sh").to_string();

    crate::handlers::prebuilt::base::setup_base(&config.params, &mut supervisor_conf, &mut image_dockerfile);

    if config.caddy.caddyfile != "" {
        crate::handlers::prebuilt::caddy::setup_domain(config.caddy, &config.params, &mut supervisor_conf, &mut image_dockerfile);
    }

    crate::handlers::service::setup_services(&config.service_commands, &config.params, &mut supervisor_conf, &mut image_dockerfile);

    let srcdir = PathBuf::from("/app/mount");
    let volume_abs_path = fs::canonicalize(&srcdir).unwrap();
    fs::create_dir_all(volume_abs_path.join("dist")).unwrap();
    fs::create_dir_all(volume_abs_path.join("../assets")).unwrap();
    fs::write("/app/assets/supervisord.conf", &supervisor_conf).unwrap();
    fs::write("/app/Dockerfile", &image_dockerfile).unwrap();
    fs::write("/app/assets/entrypoint.sh", &entrypoint).unwrap();

    if Path::new("/app/mount/dist/enclave.eif").exists() {
        fs::remove_file("/app/mount/dist/enclave.eif").unwrap();
    }
}