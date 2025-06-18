use asteroids::{
	CustomElement, Element, Reify, Transformable,
	elements::{Model, ModelPart, Spatial, Text},
};
use glam::Quat;
use mint::Vector3;
use protostar::{
	application::Application,
	xdg::{DesktopFile, Icon, IconType},
};
use stardust_xr_fusion::{
	core::values::{ResourceID, color::rgba_linear},
	drawable::{MaterialParameter, TextBounds, TextFit, XAlign, YAlign},
	spatial::Transform,
};
use std::f32::consts::PI;

const APP_SIZE: f32 = 0.06;
const ACTIVATION_DISTANCE: f32 = 0.5;

#[derive(Debug, Clone)]
pub struct App {
	pub app: Application,
	position: Vector3<f32>,
	currently_shown: bool,
}

impl App {
	pub fn create_from_desktop_file(desktop_file: DesktopFile) -> color_eyre::Result<Self> {
		let application = Application::create(desktop_file)?;
		Ok(App {
			app: application,
			position: [0.0; 3].into(),
			currently_shown: true,
		})
	}

	fn model_from_icon(icon: &Icon) -> color_eyre::Result<Element<Self>> {
		match &icon.icon_type {
			IconType::Png => {
				let t = Transform::from_rotation_scale(
					Quat::from_rotation_x(PI / 2.0) * Quat::from_rotation_y(PI),
					[APP_SIZE * 0.5; 3],
				);

				Ok(Model::namespaced("protostar", "hexagon/hexagon")
					.transform(t)
					.part(ModelPart::new("Hex").mat_param(
						"color",
						MaterialParameter::Color(rgba_linear!(0.0, 1.0, 1.0, 1.0)),
					))
					.part(ModelPart::new("Icon").mat_param(
						"diffuse",
						MaterialParameter::Texture(ResourceID::Direct(icon.path.clone())),
					))
					.build())
			}
			IconType::Gltf => Ok(Model::direct(&icon.path)?.scl([0.05; 3]).build()),
			_ => panic!("Invalid Icon Type"),
		}
	}
}

impl Reify for App {
	fn reify(&self) -> Element<Self> {
		let mut element = Spatial::default().build();

		if let Some(icon) = self.app.icon(128, false) {
			if let Ok(model) = Self::model_from_icon(&icon) {
				element = element.child(model);
			}
		} else {
			element = element.child(
				Model::namespaced("protostar", "hexagon/hexagon")
					.transform(Transform::from_rotation_scale(
						Quat::from_rotation_x(PI / 2.0) * Quat::from_rotation_y(PI),
						[APP_SIZE * 0.5; 3],
					))
					.build(),
			);
		}

		if let Some(name) = self.app.name() {
			element = element.child(
				Text::default()
					.text(name)
					.transform(Transform::from_translation_rotation(
						[0.0, 0.1, -(APP_SIZE * 4.0)],
						Quat::from_rotation_x(PI * 0.5),
					))
					.character_height(APP_SIZE * 2.0)
					.bounds(TextBounds {
						bounds: [1.0; 2].into(),
						fit: TextFit::Wrap,
						anchor_align_x: XAlign::Center,
						anchor_align_y: YAlign::Center,
					})
					.text_align_x(XAlign::Center)
					.text_align_y(YAlign::Center)
					.build(),
			);
		}

		element
	}
}
