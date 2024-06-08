use axum::Router;

use crate::config::state::AppState;

pub(crate) mod v1;

pub(crate) fn mount(app_state: AppState) -> Router<AppState> {
	Router::new().nest("/api", Router::new().nest("/v1", v1::mount(app_state)))
}

#[allow(unused_imports)]
mod tests {
	use std::{
		fs::File,
		io::{Read, Write},
		path::PathBuf,
	};

	use specta::{
		export,
		ts::{BigIntExportBehavior, ExportConfiguration, TsExportError},
		NamedType,
	};

	use super::v1::{
		auth::*, book_club::*, emailer::*, epub::*, job::*, library::*, media::*,
		metadata::*, series::*, smart_list::*, user::*, ClaimResponse, StumpVersion,
		UpdateCheck,
	};

	#[test]
	#[ignore]
	fn codegen() -> Result<(), Box<dyn std::error::Error>> {
		let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
			.join("../../packages/types")
			.join("generated.ts")
			.to_string_lossy()
			.to_string();

		println!(
			"Please ensure to only generate types using `cargo run --package codegen`"
		);

		// Write all specta `Type` structs to generated_server_types.ts
		export::ts_with_cfg(
			&path,
			&ExportConfiguration::default().bigint(BigIntExportBehavior::BigInt),
		)
		.unwrap();

		// Do postprocessing to apply changes to specific lines
		codegen_postprocessing(&path)?;

		Ok(())
	}

	fn codegen_postprocessing(path: &str) -> Result<(), Box<dyn std::error::Error>> {
		// Read the file content
		let mut file = File::open(path)?;
		let mut file_content = String::new();
		file.read_to_string(&mut file_content)?;

		// Run each replacement function
		file_content = replace_core_job_output_line(file_content);
		file_content = replace_book_club_member_role_spec(file_content);
		file_content = wrap_codegen_with_header(file_content);

		// Reopen the file in write mode to overwrite the previous content
		let mut file = File::create(path)?;
		file.write_all(file_content.as_bytes())?;

		Ok(())
	}

	fn replace_core_job_output_line(file_content: String) -> String {
		// Replace the line to remove `any`
		file_content.replace(
        "export type CoreJobOutput = LibraryScanOutput | SeriesScanOutput | ThumbnailGenerationOutput | any",
        "export type CoreJobOutput = LibraryScanOutput | SeriesScanOutput | ThumbnailGenerationOutput"
    )
	}

	fn replace_book_club_member_role_spec(file_content: String) -> String {
		file_content.replace(
			"export type BookClubMemberRoleSpec = { [key: BookClubMemberRole]: string }",
			"export type BookClubMemberRoleSpec = Record<BookClubMemberRole, string>",
		)
	}

	fn wrap_codegen_with_header(file_content: String) -> String {
		let header = "/* eslint-disable @typescript-eslint/ban-types */";
		format!("{}\n{}", header, file_content)
	}
}
