//! Defines an interface for writing OPDS 2.0 complaint JSON, based on the specification defined at
//! https://drafts.opds.io/opds-2.0.html.

mod metadata;

use chrono::{self, DateTime, Utc};
use serde::{Deserialize, Serialize};

use metadata::Metadata;

/// An OPDS 2.0 Catalog Feed as standardized at https://drafts.opds.io/opds-2.0.html#1-overview
///
/// See also, the JSON schema at:
/// https://github.com/opds-community/drafts/blob/main/schema/feed.schema.json
pub struct CatalogFeed {
	/// Contains feed-level metadata such as title or number of items
	metadata: Metadata,
	/// Feed-level links such as search or pagination
	links: Vec<Link>,
	/// A list of publications that can be acquired
	publications: Vec<Publication>,
	/// Navigation for the catalog using links
	navigation: Vec<Navigation>,
	/// Facets are meant to re-order or obtain a subset for the current list of publications
	facets: Vec<Facet>,
	/// Groups provide a curated experience, grouping publications or navigation links together
	groups: Vec<Group>,
}

pub struct Link {}

pub struct Publication {}

pub struct Navigation {}

pub struct Facet {}

pub struct Group {}
