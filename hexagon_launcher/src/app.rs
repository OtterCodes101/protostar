use crate::{ACTIVATION_DISTANCE, APP_SIZE, DEFAULT_HEX_COLOR, MODEL_SCALE};
use asteroids::elements::{Grabbable, Model, ModelPart, Text};
use asteroids::{CustomElement, Element, Reify, Transformable};
use glam::{Quat, Vec3};
use mint::{Quaternion, Vector3};
use protostar::application::Application;
use protostar::xdg::{DesktopFile, Icon, IconType};
use stardust_xr_fusion::values::ResourceID;
use stardust_xr_fusion::{
	drawable::{MaterialParameter, XAlign, YAlign},
	fields::{CylinderShape, Shape},
	spatial::Transform,
};
use stardust_xr_molecules::PointerMode;
use std::f32::consts::PI;

#[derive(Debug)]
pub struct App {
	pub app: Application,
	pub icon: Option<Icon>,
	pub pos: Vector3<f32>,
	pub rot: Quaternion<f32>,
	pub launching: bool,
}
impl App {
	pub fn new(desktop_entry: DesktopFile) -> Self {
		let app = Application::create(desktop_entry).unwrap();
		App {
			icon: app.icon(64, true).and_then(|i| i.cached_process(64).ok()),
			app,
			pos: [0.0; 3].into(),
			rot: Quat::IDENTITY.into(),
			launching: false,
		}
	}

	// Helper functions for creating app components
	#[tracing::instrument]
	fn create_model(&self) -> Element<Self> {
		match self.icon.as_ref().map(|i| (i.icon_type.clone(), i)) {
			Some((IconType::Gltf, icon)) => Model::direct(icon.path.clone())
				.unwrap()
				.transform(Transform::from_rotation_scale(
					Quat::from_rotation_x(PI / 2.0) * Quat::from_rotation_y(PI),
					[MODEL_SCALE; 3],
				))
				.build(),
			other => {
				let model = Model::namespaced("protostar", "hexagon/hexagon")
					.transform(Transform::from_rotation_scale(
						Quat::from_rotation_x(PI / 2.0) * Quat::from_rotation_y(PI),
						[APP_SIZE / 2.0; 3],
					))
					.part(
						ModelPart::new("Hex")
							.mat_param("color", MaterialParameter::Color(DEFAULT_HEX_COLOR)),
					);

				match other {
					Some((IconType::Png, icon)) => model.part(ModelPart::new("Icon").mat_param(
						"icon",
						MaterialParameter::Texture(ResourceID::Direct(icon.path.clone())),
					)),
					_ => model,
				}
				.build()
			}
		}
	}
}
impl Reify for App {
	#[tracing::instrument]
	fn reify(&self) -> Element<Self> {
		// The field shape for the grabbable
		let field_shape = Shape::Cylinder(CylinderShape {
			radius: APP_SIZE / 2.0,
			length: 0.01,
		});

		Grabbable::new(
			field_shape,
			self.pos,
			self.rot,
			move |state: &mut Self, pos, rot| {
				state.pos = pos;
				state.rot = rot;
			},
		)
		.grab_stop({
			move |state: &mut Self| {
				let pos_vec = Vec3::from(state.pos);
				if pos_vec.length_squared() > ACTIVATION_DISTANCE {
					state.launching = true;
				}
			}
		})
		.pointer_mode(PointerMode::Move)
		.max_distance(0.05)
		.build()
		.child(self.create_model())
		.child({
			Text::default()
				.text(self.app.name().unwrap_or_default())
				.character_height(APP_SIZE * 0.25)
				.text_align_x(XAlign::Center)
				.text_align_y(YAlign::Center)
				.pos([0.0, 0.0, -(APP_SIZE * 1.5)])
				.rot(Quat::from_rotation_y(PI))
				.build()
		})
	}
}
