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

//! Ethcore library

extern crate account_state;
extern crate ansi_term;
extern crate client_traits;
extern crate common_types as types;
extern crate engine;
extern crate ethcore_blockchain as blockchain;
extern crate ethcore_call_contract as call_contract;
extern crate ethcore_db as db;
extern crate ethcore_io as io;
extern crate ethcore_miner;
extern crate ethereum_types;
extern crate executive_state;
extern crate futures;
extern crate hash_db;
extern crate itertools;
extern crate journaldb;
extern crate keccak_hash as hash;
extern crate kvdb;
extern crate machine;
extern crate memory_cache;
extern crate parity_bytes as bytes;
extern crate parking_lot;
extern crate trie_db as trie;
extern crate patricia_trie_ethereum as ethtrie;
extern crate rand;
extern crate rayon;
extern crate registrar;
extern crate rlp;
extern crate rustc_hex;
extern crate serde;
extern crate snapshot;
extern crate spec;
extern crate state_db;
extern crate trace;
extern crate trie_vm_factories;
extern crate triehash_ethereum as triehash;
extern crate unexpected;
extern crate using_queue;
extern crate verification;
extern crate vm;

#[cfg(test)]
extern crate account_db;
#[cfg(test)]
extern crate ethcore_accounts as accounts;
#[cfg(test)]
extern crate stats;

#[cfg(feature = "stratum")]
extern crate ethcore_stratum;

#[cfg(feature = "stratum")]
extern crate ethash;

#[cfg(any(test, feature = "test-helpers"))]
extern crate parity_crypto;
#[cfg(any(test, feature = "test-helpers"))]
extern crate ethjson;
#[cfg(any(test, feature = "test-helpers"))]
extern crate kvdb_memorydb;
#[cfg(any(test, feature = "kvdb-rocksdb"))]
extern crate kvdb_rocksdb;
#[cfg(feature = "json-tests")]
#[macro_use]
extern crate lazy_static;
#[cfg(any(test, feature = "json-tests"))]
#[macro_use]
extern crate macros;
#[cfg(any(test, feature = "test-helpers"))]
extern crate pod;
#[cfg(any(test, feature = "blooms-db"))]
extern crate blooms_db;
#[cfg(feature = "env_logger")]
extern crate env_logger;
#[cfg(test)]
extern crate serde_json;
#[cfg(any(test, feature = "tempdir"))]
extern crate tempdir;

#[macro_use]
extern crate log;
#[macro_use]
extern crate trace_time;

#[cfg_attr(test, macro_use)]
extern crate evm;

#[cfg(all(test, feature = "price-info"))]
extern crate fetch;

#[cfg(all(test, feature = "price-info"))]
extern crate parity_runtime;

pub mod block;
pub mod client;
pub mod miner;

#[cfg(test)]
mod tests;
#[cfg(feature = "json-tests")]
pub mod json_tests;
#[cfg(any(test, feature = "test-helpers"))]
pub mod test_helpers;
