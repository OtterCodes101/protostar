mod app;
mod app_launcher;

use app::App;
use asteroids::client;
use clap::Parser;
use manifest_dir_macros::directory_relative_path;
use serde::{Deserialize, Serialize};
use stardust_xr_fusion::{project_local_resources, root::RootAspect};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
	// #[clap(short, long)]
	desktop_file: PathBuf,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
	tracing_subscriber::fmt()
		.compact()
		.with_env_filter(EnvFilter::from_env("LOG_LEVEL"))
		.init();
	client::run::<Single>(&[&project_local_resources!("../res")]);
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Single {
	app: Option<App>,
}
