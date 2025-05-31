use asteroids::elements::{Grabbable, Model, ModelPart, Text};
use asteroids::{Element, ElementTrait, Reify, Transformable};
use glam::{Quat, Vec3};
use mint::{Quaternion, Vector3};
use protostar::xdg::DesktopFile;
use stardust_xr_fusion::{
	drawable::{MaterialParameter, XAlign, YAlign},
	fields::{CylinderShape, Shape},
	spatial::Transform,
};
use stardust_xr_molecules::PointerMode;
use std::f32::consts::PI;

use crate::{ACTIVATION_DISTANCE, APP_SIZE, DEFAULT_HEX_COLOR, MODEL_SCALE};

#[derive(Debug)]
pub struct App {
	pub desktop_entry: DesktopFile,
	pub pos: Vector3<f32>,
	pub rot: Quaternion<f32>,
	pub launching: bool,
}
impl App {
	pub fn new(desktop_entry: DesktopFile) -> Self {
		App {
			desktop_entry,
			pos: [0.0; 3].into(),
			rot: Quat::IDENTITY.into(),
			launching: false,
		}
	}

	// Helper functions for creating app components
	fn create_model(&self) -> Element<Self> {
		// Create model based on icon in desktop entry
		if let Some(Ok(model)) = self.desktop_entry.icon.as_ref().map(Model::direct) {
			model
				.transform(Transform::from_rotation_scale(
					Quat::from_rotation_x(PI / 2.0) * Quat::from_rotation_y(PI),
					[MODEL_SCALE; 3],
				))
				.build()
		} else {
			// Fallback to default hexagon
			Model::namespaced("protostar", "hexagon/hexagon")
				.transform(Transform::from_rotation_scale(
					Quat::from_rotation_x(PI / 2.0) * Quat::from_rotation_y(PI),
					[APP_SIZE / 2.0; 3],
				))
				.part(
					ModelPart::new("Hex")
						.mat_param("color", MaterialParameter::Color(DEFAULT_HEX_COLOR)),
				)
				.build()
		}
	}

	fn create_label(&self) -> Element<Self> {
		Text::default()
			.text(self.desktop_entry.name.as_deref().unwrap_or_default())
			.character_height(APP_SIZE * 0.25)
			.text_align_x(XAlign::Center)
			.text_align_y(YAlign::Center)
			.pos([0.0, 0.0, -(APP_SIZE * 1.5)])
			.rot(Quat::from_rotation_y(PI))
			.build()
	}
}
impl Reify for App {
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
		.with_children([
			// App icon model
			self.create_model(),
			// App label
			self.create_label(),
		])
	}
}
