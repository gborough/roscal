# roscal_lib: Referential implementation of Open Security Controls Assessment Language(OSCAL)

This library enables users to programmatically build, and parse various OSCAL
models using the builder pattern, with bi-directional validation enabled on key
fields during serialisation/deserialisation.

All textual fields are represented as `String` type such as uuid, datetime etc.
This library strives to be unopinionated about which libraries user use to build
certain data fields, therefore it does not re-export some of the libraries used
internally for validation. The libraries for building these types
are specified Cargo.toml such as `chrono`, `uriparse`, `uuid` etc, although you
can follow the OSCAL specification and choose whichever that conforms to the requirements.
See <https://pages.nist.gov/metaschema/specification/datatypes/>

Currently the latest v1.x is being tracked by this library and compatible all the way
back to v1.0.0. It primarily supports the roscal_cli tool.

Note: The generation of Rust structs are done via external tool called quicktype. Future
plan is to migrate away from this and derive structs using a Rust native solution. The
naming of types are subject to change once this is completed.