use asteroids::{CustomElement, ValidState};
use protostar::application::Application;
use stardust_xr_fusion::spatial::SpatialRef;

#[derive(Debug)]
pub struct AppLauncher(Application);

impl<State: ValidState> CustomElement<State> for AppLauncher {
	type Inner = SpatialRef;
	type Resource = ();
	type Error = String;

	fn create_inner(
		&self,
		_asteroids_context: &asteroids::Context,
		info: asteroids::CreateInnerInfo,
		_resource: &mut Self::Resource,
	) -> Result<Self::Inner, Self::Error> {
		self.0.launch(info.parent_space);
		Ok(info.parent_space.clone())
	}

	fn update(
		&self,
		_old_decl: &Self,
		_state: &mut State,
		_inner: &mut Self::Inner,
		_resource: &mut Self::Resource,
	) {
	}

	fn spatial_aspect(&self, inner: &Self::Inner) -> SpatialRef {
		inner.clone()
	}
}
