//! Subcommands for cast
//!
//! All subcommands should respect the `foundry_config::Config`.
//! If a subcommand accepts values that are supported by the `Config`, then the subcommand should
//! implement `figment::Provider` which allows the subcommand to override the config's defaults, see
//! [`foundry_config::Config`].

pub mod estimate;
pub mod find_block;
pub mod rpc;
pub mod run;
pub mod wallet;
