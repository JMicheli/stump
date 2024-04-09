use std::{fs, path::PathBuf};

use stump_core::config::{
	defaults::{
		DEFAULT_PASSWORD_HASH_COST, DEFAULT_SCANNER_CHUNK_SIZE,
		DEFAULT_SESSION_EXPIRY_CLEANUP_INTERVAL, DEFAULT_SESSION_TTL,
	},
	StumpConfig,
};

mod utils;
use utils::get_mock_config_file;

#[test]
fn test_getting_config_from_toml() {
	// Create temporary directory and place a copy of our mock Stump.toml in it
	let tempdir = tempfile::tempdir().expect("Failed to create temporary directory");
	let temp_config_file_path = tempdir.path().join("Stump.toml");
	fs::write(temp_config_file_path, get_mock_config_file())
		.expect("Failed to write temporary Stump.toml");

	// Now we can create a StumpConfig rooted at the temporary directory and load the values
	let config_dir = tempdir.path().to_string_lossy().to_string();
	let config = StumpConfig::new(config_dir).with_config_file().unwrap();

	// Check that values are as expected
	assert_eq!(
		config,
		StumpConfig {
			profile: "release".to_string(),
			port: 1337,
			verbosity: 3,
			pretty_logs: true,
			db_path: Some("not_a_real_path".to_string()),
			client_dir: "not_a_real_dir".to_string(),
			config_dir: "also_not_a_real_dir".to_string(),
			allowed_origins: vec!["origin1".to_string(), "origin2".to_string()],
			pdfium_path: Some("not_a_path_to_pdfium".to_string()),
			disable_swagger: false,
			password_hash_cost: DEFAULT_PASSWORD_HASH_COST,
			session_ttl: DEFAULT_SESSION_TTL,
			expired_session_cleanup_interval: DEFAULT_SESSION_EXPIRY_CLEANUP_INTERVAL,
			scanner_chunk_size: DEFAULT_SCANNER_CHUNK_SIZE,
		}
	);

	// Ensure that the temporary directory is deleted
	tempdir
		.close()
		.expect("Failed to delete temporary directory");
}
