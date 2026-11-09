//! Decals (`pulsar.decals`): the plugin's root crate, the one place the
//! plugin registers everything it contributes.
//!
//! The plugin's other crates hold the code: `helio-pass-decal` (the feature:
//! the decal drawer after the G-buffer's and the join that places decals in
//! their owners' boxes) and `decals_component` (the SceneDB component, its
//! row, its image binding and its script method). Neither registers a
//! contribution; this crate imports them and submits their registrations, so
//! this file and the module below are the whole list of what the plugin
//! brings:
//!
//! - [`runtime`]: what a game links. The component class, the Helio feature,
//!   the scene rows and the texture binding.
//! - The editor (with the `editor` feature) registers nothing: the inspector
//!   edits a decal through its reflected properties.
//!
//! Every registration names this crate as its origin
//! (`pulsar_plugins::origin!()`), so the plugin gate skips all of them while
//! a project disables the plugin, and the plugin catalog lists them under
//! `pulsar.decals`. Other plugins never depend on this crate; hosts reach it
//! through the installed set or a generated game, a game without the
//! `editor` feature.

// The plugin's descriptor (`pulsar-plugin.toml`): linking this crate installs
// the plugin; a project enables it or not (`pulsar_plugins`).
pulsar_plugins::installed!("pulsar-plugin.toml");

pub use decals_component;

pub mod runtime;
