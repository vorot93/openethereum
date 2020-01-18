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

use trie_db::TrieFactory;
use ethtrie::Layout;
use account_db::Factory as AccountFactory;
use evm::{Factory as EvmFactory};
use vm::{Exec, ActionParams, VersionedSchedule, Schedule};
use wasm::WasmInterpreter;

const WASM_MAGIC_NUMBER: &[u8; 4] = b"\0asm";

/// Virtual machine factory
#[derive(Default, Clone)]
pub struct VmFactory {
	evm: EvmFactory,
}

impl VmFactory {
	pub fn create(&self, params: ActionParams, schedule: &Schedule, depth: usize) -> Option<Box<dyn Exec>> {
		if params.code_version.is_zero() {
			Some(if schedule.wasm.is_some() && schedule.versions.is_empty() && params.code.as_ref().map_or(false, |code| code.len() > 4 && &code[0..4] == WASM_MAGIC_NUMBER) {
				Box::new(WasmInterpreter::new(params))
			} else {
				self.evm.create(params, schedule, depth)
			})
		} else {
			let version_config = schedule.versions.get(&params.code_version);

			match version_config {
				Some(VersionedSchedule::PWasm) => {
					Some(Box::new(WasmInterpreter::new(params)))
				},
				None => None,
			}
		}
	}

	pub fn new(cache_size: usize) -> Self {
		Self { evm: EvmFactory::new(cache_size) }
	}
}

impl From<EvmFactory> for VmFactory {
	fn from(evm: EvmFactory) -> Self {
		Self { evm }
	}
}

/// Collection of factories.
#[derive(Default, Clone)]
pub struct Factories {
	/// factory for evm.
	pub vm: VmFactory,
	/// factory for tries.
	pub trie: TrieFactory<Layout>,
	/// factory for account databases.
	pub accountdb: AccountFactory,
}
