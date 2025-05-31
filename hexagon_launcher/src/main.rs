mod app;
mod hex;

use app::App;
use asteroids::{
	ClientState, Element, ElementTrait, Migrate, Reify, Transformable, client,
	elements::{Button, Model, ModelPart, Spatial},
};
use glam::Quat;
use hex::Hex;
use protostar::xdg::{get_desktop_files, parse_desktop_file};
use serde::{Deserialize, Serialize};
use stardust_xr_fusion::{
	core::values::color::{Rgba, color_space::LinearRgb, rgba_linear},
	drawable::MaterialParameter,
	project_local_resources,
	spatial::Transform,
};
use std::f32::consts::PI;
use tracing_subscriber::EnvFilter;

// Constants from original implementation
const APP_SIZE: f32 = 0.06;
const PADDING: f32 = 0.005;
const MODEL_SCALE: f32 = 0.03;
const ACTIVATION_DISTANCE: f32 = 0.05;

const DEFAULT_HEX_COLOR: Rgba<f32, LinearRgb> = rgba_linear!(0.211, 0.937, 0.588, 1.0);
const BTN_SELECTED_COLOR: Rgba<f32, LinearRgb> = rgba_linear!(0.0, 1.0, 0.0, 1.0);
const BTN_COLOR: Rgba<f32, LinearRgb> = rgba_linear!(1.0, 1.0, 0.0, 1.0);

#[tokio::main(flavor = "current_thread")]
async fn main() {
	color_eyre::install().unwrap();
	tracing_subscriber::fmt()
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	client::run::<HexagonLauncher>(&[&project_local_resources!("../res")]).await
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HexagonLauncher {
	/// if the hexagon launcher is expanded
	open: bool,
	#[serde(skip)]
	/// position in the vector is mapped to hex coordinates
	apps: Vec<App>,
}
impl Migrate for HexagonLauncher {
	type Old = Self;
}

impl ClientState for HexagonLauncher {
	const APP_ID: &'static str = "org.protostar.hexagon_launcher";

	fn initial_state_update(&mut self) {
		// Load desktop files
		self.apps = get_desktop_files()
			.filter_map(|d| parse_desktop_file(d).ok())
			.filter(|d| !d.no_display)
			.map(App::new)
			.collect();

		// Sort by name
		self.apps
			.sort_by_key(|app| app.desktop_entry.name.clone().unwrap_or_default());
	}
	fn reify(&self) -> Element<Self> {
		// Build UI based on current state
		let center_button = self.create_center_button();
		let app_grid = self.create_app_grid();

		Spatial::default()
			.zoneable(true)
			.with_children([center_button, app_grid])
	}
}

impl HexagonLauncher {
	fn create_center_button(&self) -> Element<Self> {
		let model = Model::namespaced("protostar", "hexagon/hexagon")
			.transform(Transform::from_rotation_scale(
				Quat::from_rotation_x(PI / 2.0) * Quat::from_rotation_y(PI),
				[MODEL_SCALE; 3],
			))
			.part(ModelPart::new("Hex").mat_param(
				"color",
				MaterialParameter::Color(if self.open {
					BTN_SELECTED_COLOR
				} else {
					BTN_COLOR
				}),
			))
			.build();

		Button::new(|state: &mut Self| {
			state.open = !state.open;
		})
		.size([APP_SIZE; 2])
		.with_children([model])
	}

	fn create_app_grid(&self) -> Element<Self> {
		// Create a spatial that contains all app launchers
		if !self.open {
			// Return empty if not open
			return Spatial::default().build();
		}

		// Create each app launcher using reify
		let app_elements: Vec<Element<Self>> = self
			.apps
			.iter()
			.enumerate()
			.map(|(i, app)| {
				let app = app.reify_substate(move |state: &mut Self| state.apps.get_mut(i));
				Spatial::default()
					.pos(Hex::spiral(i + 1).get_coords())
					.with_children([app])
			})
			.collect();

		Spatial::default().with_children(app_elements)
	}
}
