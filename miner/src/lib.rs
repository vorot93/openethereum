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

//! Miner module
//! Keeps track of transactions and mined block.

extern crate ansi_term;
extern crate common_types as types;
extern crate ethabi;
extern crate ethcore_call_contract as call_contract;
extern crate ethereum_types;
extern crate futures;

extern crate parity_util_mem;
extern crate keccak_hash as hash;
extern crate linked_hash_map;
extern crate parity_runtime;
extern crate parking_lot;
#[cfg(feature = "price-info")]
extern crate price_info;
extern crate registrar;
extern crate rlp;
extern crate transaction_pool as txpool;
extern crate serde;

#[macro_use]
extern crate ethabi_contract;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate trace_time;

#[cfg(test)]
extern crate rustc_hex;
#[cfg(test)]
extern crate parity_crypto;
#[cfg(test)]
extern crate env_logger;

pub mod external;
#[cfg(feature = "price-info")]
pub mod gas_price_calibrator;
pub mod gas_pricer;
pub mod local_accounts;
pub mod pool;
pub mod service_transaction_checker;
#[cfg(feature = "work-notify")]
pub mod work_notify;
