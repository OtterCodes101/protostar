mod app;
mod app_launcher;

use app::App;
use asteroids::{ClientState, CustomElement, Migrate, Reify, client, elements::Spatial};
use clap::Parser;
use protostar::xdg::DesktopFile;
use serde::{Deserialize, Serialize};
use stardust_xr_fusion::{
	project_local_resources,
	values::color::{Rgba, color_space::LinearRgb, rgba_linear},
};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

// Constants from original implementation
const APP_SIZE: f32 = 0.06;
const MODEL_SCALE: f32 = 0.03;
const ACTIVATION_DISTANCE: f32 = 0.05;

const DEFAULT_HEX_COLOR: Rgba<f32, LinearRgb> = rgba_linear!(0.211, 0.937, 0.588, 1.0);

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
	client::run::<Single>(&[&project_local_resources!("../res")]).await
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Single {
	app: Option<App>,
}
impl Migrate for Single {
	type Old = Self;
}
impl ClientState for Single {
	const APP_ID: &'static str = "org.stardustxr.protostar.single";

	fn initial_state_update(&mut self) {
		let desktop_file_path = Args::parse().desktop_file;

		self.app
			.replace(App::new(DesktopFile::parse(desktop_file_path).unwrap()));
	}

	fn reify(&self) -> asteroids::Element<Self> {
		if let Some(app) = self.app.as_ref() {
			app.reify_substate(|state: &mut Self| state.app.as_mut())
		} else {
			Spatial::default().build()
		}
	}
}
