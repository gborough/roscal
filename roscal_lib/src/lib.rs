//! # OSCAL: Referential implementation of Open Security Controls Assessment Language(OSCAL)
//! This library enables users to programmatically parse and build various OSCAL
//! models using the builder pattern, with bi-directional validation enabled on key
//! fields during serialisation/deserialisation.
//!
//! All textual fields are represented as `String` type such as uuid, datetime etc.
//! This library strives to be unopinionated about which library is being used by
//! the user to build certain data field, therefore it does not re-export some of
//! the libraries used internally for validation. The libraries for building these types
//! are specified Cargo.toml such as `chrono`, `uriparse`, `uuid` etc, although you
//! can follow the OSCAL specification and choose whichever that conforms to the requirement.
//! See <https://pages.nist.gov/metaschema/specification/datatypes/>
//!
//! Currently the latest v1.x is being tracked by this library and primarily supports
//! the roscal_cli tool.
//!  
//!
//! # Example of a building a minimum Catalog model
//!
//! ```toml
//! [dependencies]
//! roscal_lib = "0.1.0"
//! uuid = { version = "1.6.1", features = ["v4"] }
//! chrono = { version = "0.4.31", features = ["serde"] }
//! ```
//!
//! ```
//! use roscal_lib::control::catalog::{self, CatalogBuilder, CatalogClassBuilder};
//!
//! fn builder() {
//!     let mut catalog_builder = CatalogBuilder::default();
//!     let mut catalog_class_builder = CatalogClassBuilder::default();
//!
//!     let uuid = uuid::Uuid::new_v4().to_string();
//!     let last_modified = chrono::DateTime::parse_from_rfc3339("2023-12-31T23:59:59Z")
//!     .unwrap().to_string();
//!     let metadata = catalog::DocumentMetadataBuilder::default()
//!         .version("1")
//!         .title("catalog")
//!         .oscal_version("1.0.0")
//!         .last_modified(last_modified)
//!         .build()
//!         .expect("unable to build metadata");
//!
//!     let catalog_class = catalog_class_builder
//!         .uuid(uuid)
//!         .metadata(metadata)
//!         .build()
//!         .expect("unable to build catalog class");
//!     let catalog = catalog_builder
//!         .catalog(catalog_class)
//!         .build()
//!         .expect("unable to build catalog");
//!
//!     assert_eq!(true, serde_json::to_string(&catalog).is_ok())
//! }
//! ```
#![forbid(unsafe_code)]

pub mod assessment;
mod common_impl;
pub mod control;
pub mod implementation;
mod uuid_impl;
mod validation;

/// Update UUID v4/v5 if model content has changed
pub trait UpdateUuid: Sized {
    fn update_uuid_v4(&mut self, rhs: &Self) -> &mut Self;
    fn update_uuid_v5(&mut self, rhs: &Self) -> &mut Self;
}
