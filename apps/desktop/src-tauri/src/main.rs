#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

// TODO: https://github.com/tauri-apps/tauri-plugin-store
// TODO: https://tauri.app/v1/guides/features/menu

mod commands;
mod utils;

use std::sync::Mutex;

use utils::discord::StumpDiscordPresence;

use commands::{set_discord_presence, set_use_discord_connection};

// TODO: https://github.com/tauri-apps/tauri/issues/2663

fn main() {
	tauri::Builder::default()
		.manage(Mutex::new(StumpDiscordPresence::new()))
		.invoke_handler(tauri::generate_handler![
			set_use_discord_connection,
			set_discord_presence
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}