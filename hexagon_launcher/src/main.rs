mod app;
mod hex;

use app::App;
use asteroids::{
	ClientState, CustomElement, Element, Migrate, Reify, Transformable, client,
	elements::{Button, Grabbable, Model, ModelPart, Spatial},
};
use glam::Quat;
use hex::Hex;
use mint::{Quaternion, Vector3};
use protostar::xdg::{get_desktop_files, parse_desktop_file};
use serde::{Deserialize, Serialize};
use stardust_xr_fusion::{
	core::values::color::{Rgba, color_space::LinearRgb, rgba_linear},
	drawable::MaterialParameter,
	fields::{CylinderShape, Shape},
	project_local_resources,
	spatial::Transform,
};
use stardust_xr_molecules::PointerMode;
use std::f32::consts::PI;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

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

	let registry = tracing_subscriber::registry();
	#[cfg(feature = "tracy")]
	let registry = registry.with({
		use tracing_subscriber::Layer;
		tracing_tracy::TracyLayer::new(tracing_tracy::DefaultConfig::default())
			.with_filter(tracing::level_filters::LevelFilter::DEBUG)
	});
	let log_layer = tracing_subscriber::fmt::Layer::new()
		.with_thread_names(true)
		.with_ansi(true)
		.with_line_number(true)
		.with_filter(EnvFilter::from_default_env());
	registry.with(log_layer).init();

	client::run::<HexagonLauncher>(&[&project_local_resources!("../res")]).await
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HexagonLauncher {
	/// if the hexagon launcher is expanded
	open: bool,
	pos: Vector3<f32>,
	rot: Quaternion<f32>,
	#[serde(skip)]
	/// position in the vector is mapped to hex coordinates
	apps: Vec<App>,
}

impl Default for HexagonLauncher {
	fn default() -> Self {
		Self {
			open: Default::default(),
			pos: [0.0; 3].into(),
			rot: Quat::IDENTITY.into(),
			apps: Default::default(),
		}
	}
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
			.sort_by_key(|app| app.app.name().unwrap_or_default().to_string());
	}
	#[tracing::instrument]
	fn reify(&self) -> Element<Self> {
		// Build UI based on current state
		Grabbable::new(
			Shape::Cylinder(CylinderShape {
				radius: APP_SIZE / 2.0,
				length: 0.01,
			}),
			self.pos,
			self.rot,
			|state: &mut Self, pos, rot| {
				state.pos = pos;
				state.rot = rot;
			},
		)
		.pointer_mode(PointerMode::Align)
		.zoneable(false)
		.build()
		.child({
			Button::new(|state: &mut HexagonLauncher| {
				state.open = !state.open;
			})
			.size([APP_SIZE; 2])
			.build()
		})
		.child(
			Model::namespaced("protostar", "hexagon/hexagon")
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
				.build(),
		)
		.children(
			self.open
				.then(|| {
					self.apps.iter().enumerate().map(|(i, app)| {
						Spatial::default()
							.pos(Hex::spiral(i + 1).get_coords())
							.build()
							.identify(&app.app.name())
							.child(app.reify_substate(move |state: &mut HexagonLauncher| {
								state.apps.get_mut(i)
							}))
					})
				})
				.into_iter()
				.flatten(),
		)
	}
}
