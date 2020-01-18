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

extern crate rustc_version;
extern crate toml;
extern crate vergen;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use vergen::{ConstantsFlags, generate_cargo_keys};

const ERROR_MSG: &str = "Failed to generate metadata files";

fn main() {
	let vergen_flags = ConstantsFlags::COMMIT_DATE |
		ConstantsFlags::SHA |
		ConstantsFlags::SHA_SHORT |
		ConstantsFlags::TARGET_TRIPLE |
		ConstantsFlags::REBUILD_ON_HEAD_CHANGE;
	generate_cargo_keys(vergen_flags).expect(ERROR_MSG);

	let version = rustc_version::version().expect(ERROR_MSG);

	let cargo: toml::Value = toml::from_str(include_str!("./Cargo.toml")).expect(ERROR_MSG);
	let track = cargo["package"]["metadata"]["track"].as_str().expect("'track' has to be a string!");

	create_file("meta.rs", format!("
			/// This versions track.
			#[allow(unused)]
			pub const TRACK: &str = {track:?};

			/// Returns compiler version.
			pub const fn rustc_version() -> &'static str {{
				\"{version}\"
			}}
		",
		track = track,
		version = version,
	));
}

fn create_file(filename: &str, data: String) {
	let out_dir = env::var("OUT_DIR").expect(ERROR_MSG);
	let dest_path = Path::new(&out_dir).join(filename);
	let mut f = File::create(&dest_path).expect(ERROR_MSG);
	f.write_all(data.as_bytes()).expect(ERROR_MSG);
}
