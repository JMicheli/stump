pub mod context;
pub mod enums;
pub mod errors;
pub mod fs;
pub mod server;

pub type CoreResult<T> = Result<T, CoreError>;

pub use context::*;
pub use enums::*;
pub use errors::*;
pub use fs::*;
pub use server::*;

#[allow(unused_imports)]
mod tests {
	use std::{fs::File, io::Write, path::PathBuf};

	use specta::ts_export;

	use crate::{
		db::models::{
			epub::*, library::*, log::*, media::*, read_progress::*, reading_list::*,
			series::*, tag::*, user::*,
		},
		event::*,
		job::*,
	};

	use super::{enums::*, errors::*, fs::*, inputs::*, server::*};

	#[test]
	#[ignore]
	fn codegen() -> Result<(), Box<dyn std::error::Error>> {
		let mut file = File::create(
			PathBuf::from(env!("CARGO_MANIFEST_DIR"))
				.join("../common/client/src/types")
				.join("core.ts"),
		)?;

		file.write_all(b"// DO NOT MODIFY THIS FILE, IT IS AUTOGENERATED\n\n")?;

		file.write_all(format!("{}\n\n", ts_export::<StumpVersion>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<User>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<UserRole>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<UserPreferences>()?).as_bytes())?;
		file.write_all(
			format!("{}\n\n", ts_export::<UserPreferencesUpdate>()?).as_bytes(),
		)?;
		file.write_all(
			format!("{}\n\n", ts_export::<LoginOrRegisterArgs>()?).as_bytes(),
		)?;
		file.write_all(format!("{}\n\n", ts_export::<ClaimResponse>()?).as_bytes())?;

		file.write_all(format!("{}\n\n", ts_export::<FileStatus>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<Library>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<LibraryPattern>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<LibraryScanMode>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<LibraryOptions>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<CreateLibraryArgs>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<UpdateLibraryArgs>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<LibrariesStats>()?).as_bytes())?;

		file.write_all(format!("{}\n\n", ts_export::<Series>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<Media>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<MediaMetadata>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<ReadProgress>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<Tag>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<LayoutMode>()?).as_bytes())?;

		file.write_all(format!("{}\n\n", ts_export::<Epub>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<EpubContent>()?).as_bytes())?;

		file.write_all(format!("{}\n\n", ts_export::<JobStatus>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<JobUpdate>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<JobReport>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<CoreEvent>()?).as_bytes())?;

		file.write_all(format!("{}\n\n", ts_export::<ReadingList>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<CreateReadingList>()?).as_bytes())?;

		file.write_all(format!("{}\n\n", ts_export::<DirectoryListing>()?).as_bytes())?;
		file.write_all(
			format!("{}\n\n", ts_export::<DirectoryListingFile>()?).as_bytes(),
		)?;
		file.write_all(
			format!("{}\n\n", ts_export::<DirectoryListingInput>()?).as_bytes(),
		)?;

		file.write_all(format!("{}\n\n", ts_export::<Log>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<LogMetadata>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<LogLevel>()?).as_bytes())?;

		file.write_all(format!("{}\n\n", ts_export::<Direction>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<PageParams>()?).as_bytes())?;
		// Note: this will essentially be Partial<PageParams>...
		file.write_all(format!("{}\n\n", ts_export::<PagedRequestParams>()?).as_bytes())?;
		file.write_all(format!("{}\n\n", ts_export::<PageInfo>()?).as_bytes())?;
		// file.write_all(format!("{}\n\n", ts_export::<Pageable>()?).as_bytes())?; // TODO: figure this out

		Ok(())
	}
}