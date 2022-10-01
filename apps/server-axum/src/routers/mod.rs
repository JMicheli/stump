use axum::Router;

mod api;
mod opds;
mod spa;

pub(crate) fn mount() -> Router {
	Router::new()
		.merge(spa::mount())
		.nest("/api", api::mount())
		.nest("/opds/v1.2", opds::mount())
}