// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

//! Updater for Parity executables

#![warn(
	clippy::all,
	clippy::pedantic,
	clippy::nursery,
)]
#![allow(
	clippy::blacklisted_name,
	clippy::cast_lossless,
	clippy::cast_possible_truncation,
	clippy::cast_possible_wrap,
	clippy::cast_precision_loss,
	clippy::cast_ptr_alignment,
	clippy::cast_sign_loss,
	clippy::cognitive_complexity,
	clippy::default_trait_access,
	clippy::enum_glob_use,
	clippy::eval_order_dependence,
	clippy::fallible_impl_from,
	clippy::float_cmp,
	clippy::identity_op,
	clippy::if_not_else,
	clippy::indexing_slicing,
	clippy::inline_always,
	clippy::items_after_statements,
	clippy::large_enum_variant,
	clippy::many_single_char_names,
	clippy::match_same_arms,
	clippy::missing_errors_doc,
	clippy::missing_safety_doc,
	clippy::module_inception,
	clippy::module_name_repetitions,
	clippy::must_use_candidate,
	clippy::needless_pass_by_value,
	clippy::needless_update,
	clippy::non_ascii_literal,
	clippy::option_option,
	clippy::pub_enum_variant_names,
	clippy::same_functions_in_if_condition,
	clippy::shadow_unrelated,
	clippy::similar_names,
	clippy::single_component_path_imports,
	clippy::too_many_arguments,
	clippy::too_many_lines,
	clippy::type_complexity,
	clippy::unused_self,
	clippy::used_underscore_binding,
)]

extern crate client_traits;
extern crate common_types;
extern crate ethabi;
extern crate ethabi_derive;
extern crate ethcore;
extern crate ethcore_sync as sync;
extern crate ethereum_types;
extern crate keccak_hash as hash;
extern crate parity_bytes as bytes;
extern crate parity_hash_fetch as hash_fetch;
extern crate parity_path;
extern crate parity_version as version;
extern crate parking_lot;
extern crate rand;
extern crate semver;
extern crate target_info;

#[macro_use]
extern crate ethabi_contract;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

#[cfg(test)]
extern crate tempdir;

#[cfg(test)]
#[macro_use]
extern crate matches;

mod updater;
mod types;
mod service;

pub use service::Service;
pub use types::{ReleaseInfo, OperationsInfo, CapState, VersionInfo, ReleaseTrack};
pub use updater::{Updater, UpdateFilter, UpdatePolicy};
