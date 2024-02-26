# roscal: Open Security Controls Assessment Language Toolbox

## Introduction

This project aims to build a collection of toolings and libraries for
Open Security Controls Assessment Language(OSCAL) developed by NIST in
the Rust programming language ecosystem. One main focus of this project
is to extend the capabilities of OSCAL model building via automated
data gathering and seemless interactions that target a variety of systems,
tasks such as gathering data from telemetry points, CI/CD pipelines, cloud
infrastructures, version control systems etc. System implementations that rely
on the security and control specification from OSCAL models should form a
tight feedback loop where evolving and relevant data also guides the process
of producing and updating the models themselves in an automated fashion, thus
allowing continuous security enforcement and better visibility into one's
security posture.

One major motivation for this project is to build toolings that are intended
to be used in environments where certain security mandates must be adhere to.
There are unique advantages in using Rust not only for its memory and type safety
characteristics, but also minimal system dependencies and high agility when
it comes to efforts related to automation and integration in these environments.
Most of the time this can be done by simply embedding and running a single binary
or a lean containerised workload throughout.

The goals and focuses of subsequent development efforts can be found here:

https://github.com/gborough/roscal/blob/main/ROADMAP.md

## Components

The first iteration of this project contains two major components:

- roscal_lib: A library with referential implementation of the OSCAL models, which
aims to always track the latest OSCAL standard and primarily support roscal_cli
and the building of OSCAL modela programmatically in Rust.

- roscal_cli: A command line tool for OSCAL model file manipulation, including
but not limited to model file dissection, inspection, editing, merging and
validation etc.

## Downloads

Latest CLI Tool Releases:

## CLI User Manual

### roscal-cli

Run: ```roscal --help``` to see available options

```
Cli Tool For Manipulating OSCAL Model File

Usage: roscal <COMMAND>

Commands:
  dissect       Dissect OSCAL model file and generate a workspace for viewing and editing
                    Full Example:
                    roscal dissect --file /dir/catalog.json
                    --model Catalog
                    --blocks controls,groups
                    --output-dir /home/workspace
                    --parse-markup
  merge         Merge existing worspace and generate new OSCAL model file
                    Full Example:
                    roscal merge --dir /dir/existing_workspace
                    --output-dir /dir/merged
                    --output-format yaml
                    --update-uuid v4
  validate      Validate a specific type of OSCAL model file
                    Full Example:
                    roscal validate --file /dir/catalog.json
                    --model Catalog
  show-dissect  Show available models and blocks for dissect operation
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### Dissect Option

The dissect option takes an OSCAL model file and generates a workspace where
a user could operate on the dissected model file into specified blocks inside
a folder named ```modifiable```, this allows the blocks to be easily inspected
and edited so that it could be later merged to produce a new file. Everything is
in YAML format for easy reading and editing.

Highly recommend to pass in ```--blocks all``` option for the easiest and most
versatile way of editing the entire model file.

Take care not to modified anything outside the modifiable folder as it might alter
the integrity of the workspace.

```
Dissect OSCAL model file and generate a workspace for viewing and editing
Full Example:
roscal dissect --file /dir/catalog.json
--model Catalog
--blocks controls,groups
--output-dir /home/workspace
--parse-markup

Usage: roscal dissect [OPTIONS] --file <FILE> --model <MODEL>

Options:
      --file <FILE>              Location of the OSCAL model file
      --model <MODEL>            Specifiy which OSCAL model to be processed
                                 Run `roscal show-dissect` for available models
      --blocks <BLOCKS>...       Specifiy which blocks to be dissected
                                 Duplicate blocks will be combined
                                 Run `roscal show-dissect` for available blocks
      --output-dir <OUTPUT_DIR>  Specify where dissect workspace should be created
                                 Optional. Will use current directory if unspecified
      --parse-markup             Whether to parse markup lines
                                 Currently experimental feature
                                 see https://pages.nist.gov/metaschema/specification/datatypes/#markup-data-types
  -h, --help                     Print help
```

#### Merge Option

The merge options takes an existing workspace generated by the dissect command
and reconstitute its content to produce a new model file with specified output format.
It can also optional update the toplevel uuid should the content differs from the
original one.

Highly recommend to always pass in ```--output-dir``` for clear separation of the
original and generated.

Caveat: Currently ```--update-uuid``` only update UUIDs in core types but not recursively
for all eligible UUID fields. This feature will be made available in future releases.

```
Merge existing worspace and generate new OSCAL model file
Full Example:
roscal merge --dir /dir/existing_workspace
--output-dir /dir/merged
--output-format yaml
--update-uuid v4

Usage: roscal merge [OPTIONS] --output-format <OUTPUT_FORMAT>

Options:
      --dir <DIR>                      Location of existing workspace created by dissect operation
                                       Optional. Can be run directly in existing workspace
      --output-dir <OUTPUT_DIR>        Specify where merged file should be created
                                       Optional. Can be run directly in existing workspace
      --output-format <OUTPUT_FORMAT>  Options: json or yaml
      --update-uuid <UPDATE_UUID>      Options: v4 or v5 (as in uuid version)
                                       Optional. No-op if model unchanged or uuid manually updated
  -h, --help                           Print help
```

#### Validate Option

The validate option takes an OSCAL model file and validate against the specified model

```
Validate a specific type of OSCAL model file
Full Example:
roscal validate --file /dir/catalog.json
--model Catalog

Usage: roscal validate --file <FILE> --model <MODEL>

Options:
      --file <FILE>    Location of OSCAL model file
      --model <MODEL>  Model type of OSCAL model file
                       Run `roscal show-dissect` for available models
  -h, --help           Print help
```

#### Show-Dissect Option

The show-dissect option compliments the dissect option and display all available
models and blocks options to be passed in.

Run ```roscal show-dissect``` if unsure which models/blocks names available to use 

```
Show available models and blocks for dissect operation

Usage: roscal show-dissect

Options:
  -h, --help  Print help
```

### OSCAL Library

The roscal library adopts the builder pattern to programmatically build OSCAL
model files. You can find the official documentation here:



Example of building a minimal Catalog model:

```toml
[dependencies]
roscal = "0.1.0"
uuid = { version = "1.6.1", features = ["v4"] }
chrono = { version = "0.4.31", features = ["serde"] }
```

```rust
use roscal_lib::control::catalog::{self, CatalogBuilder, CatalogClassBuilder};

fn builder() {
    let mut catalog_builder = CatalogBuilder::default();
    let mut catalog_class_builder = CatalogClassBuilder::default();

    let uuid = uuid::Uuid::new_v4().to_string();
    let last_modified = chrono::DateTime::parse_from_rfc3339("2023-12-31T23:59:59Z")
    .unwrap().to_string();
    let metadata = catalog::DocumentMetadataBuilder::default()
        .version("1")
        .title("catalog")
        .oscal_version("1.0.0")
        .last_modified(last_modified)
        .build()
        .expect("unable to build metadata");

    let catalog_class = catalog_class_builder
        .uuid(uuid)
        .metadata(metadata)
        .build()
        .expect("unable to build catalog class");
    let catalog = catalog_builder
        .catalog(catalog_class)
        .build()
        .expect("unable to build catalog");

    println!("{catalog}")
}
```

## Build Instruction

The roscal library relies on ```quicktype``` to generate Rust structs and then
manipulated the Rust abstract syntax tree to derive various properties. To build
project locally, it's highly recommended to install quicktype via ```npm install -g quicktype```
before running ```cargo build```, if you have npm available on your OS.

If you are using nix and have flake enabled, simply run ```nix build```.

Note: Build under Windows is currently broken due to open issue https://github.com/glideapps/quicktype/issues/1113. Workaround is being worked on at the moment.

## License

This project is licensed under dual MIT and Apache 2.0 licenses.

[MIT license]: https://github.com/gborough/roscal/blob/main/LICENSE-MIT
[APACHE license]: https://github.com/gborough/roscal/blob/main/LICENSE-APACHE

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in roscal by you, shall be licensed as MIT or Apache 2.0, without
any additional terms or conditions.