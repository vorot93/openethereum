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

//! Types used in the public API
//!
//! This crate stores Parity Etherem specific types that are
//! COMMONLY used across different separate modules of the codebase.
//! It should only focus on data structures, not any logic that relates to them.
//!
//! The interaction between modules should be possible by
//! implementing a required trait that potentially uses some of the data
//! structures from that crate.
//!
//! NOTE If you can specify your data type in the same crate as your trait, please do that.
//! Don't treat this crate as a bag for any types that we use in Parity Ethereum.
//! This one is reserved for types that are shared heavily (like transactions),
//! historically this contains types extracted from `ethcore` crate, if possible
//! we should try to dissolve that crate in favour of more fine-grained crates,
//! by moving the types closer to where they are actually required.

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

extern crate ethbloom;
extern crate ethereum_types;
extern crate ethjson;
extern crate parity_crypto;
#[macro_use]
extern crate derive_more;
extern crate keccak_hash as hash;
extern crate parity_bytes as bytes;
extern crate patricia_trie_ethereum as ethtrie;
extern crate parity_snappy;
extern crate rlp;
extern crate unexpected;

#[macro_use]
extern crate rlp_derive;
extern crate parity_util_mem;
extern crate parity_util_mem as malloc_size_of;

#[cfg(test)]
extern crate rustc_hex;

#[macro_use]
pub mod views;

pub mod account_diff;
pub mod ancestry_action;
pub mod basic_account;
pub mod block;
pub mod block_status;
pub mod blockchain_info;
pub mod call_analytics;
pub mod chain_notify;
pub mod client_types;
pub mod encoded;
pub mod engines;
pub mod errors;
pub mod filter;
pub mod header;
pub mod ids;
pub mod io_message;
pub mod import_route;
pub mod log_entry;
pub mod pruning_info;
pub mod receipt;
pub mod security_level;
pub mod snapshot;
pub mod state_diff;
pub mod trace_filter;
pub mod transaction;
pub mod tree_route;
pub mod verification;
pub mod data_format;

/// Type for block number.
pub type BlockNumber = u64;
