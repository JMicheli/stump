use std::time::Duration;

use stump_core::{
	db::entity::{LibraryPattern, LibraryScanMode},
	prisma::media,
	CoreResult, Ctx,
};

mod utils;
use utils::{init_test, TempLibrary};

#[tokio::test]
async fn digest_zips() -> CoreResult<()> {
	init_test().await;

	let ctx = Ctx::mock().await;

	let _temp_library_guard = TempLibrary::create(
		&ctx.db,
		LibraryPattern::SeriesBased,
		LibraryScanMode::Default,
	)
	.await?;

	let zips = ctx
		.db
		.media()
		.find_many(vec![media::extension::in_vec(vec![
			"zip".to_string(),
			"cbz".to_string(),
		])])
		.exec()
		.await?;

	// Wait for scan job to complete
	tokio::time::sleep(Duration::from_secs(5)).await;

	assert_ne!(zips.len(), 0);
	Ok(())
}
