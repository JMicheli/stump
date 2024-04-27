use chrono::{self, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub identifier: Option<String>,

	pub title: String,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub subtitle: Option<String>,

	#[serde(
		with = "chrono::serde::ts_seconds_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub modified: Option<DateTime<Utc>>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub items_per_page: Option<i32>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub current_page: Option<i32>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub number_of_items: Option<i32>,
}
