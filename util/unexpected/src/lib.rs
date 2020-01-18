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

//! Error utils

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

use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Error indicating an expected value was not found.
pub struct Mismatch<T> {
	/// Value expected.
	pub expected: T,
	/// Value found.
	pub found: T,
}

impl<T: fmt::Display> fmt::Display for Mismatch<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_fmt(format_args!("Expected {}, found {}", self.expected, self.found))
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Error indicating value found is outside of a valid range.
pub struct OutOfBounds<T> {
	/// Minimum allowed value.
	pub min: Option<T>,
	/// Maximum allowed value.
	pub max: Option<T>,
	/// Value found.
	pub found: T,
}

#[allow(clippy::use_self)]
impl<T> OutOfBounds<T> {
	pub fn map<F, U>(self, map: F) -> OutOfBounds<U>
		where F: Fn(T) -> U
	{
		OutOfBounds {
			min: self.min.map(&map),
			max: self.max.map(&map),
			found: map(self.found),
		}
	}
}

impl<T: fmt::Display> fmt::Display for OutOfBounds<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let msg = match (self.min.as_ref(), self.max.as_ref()) {
			(Some(min), Some(max)) => format!("Min={}, Max={}", min, max),
			(Some(min), _) => format!("Min={}", min),
			(_, Some(max)) => format!("Max={}", max),
			(None, None) => "".into(),
		};

		f.write_fmt(format_args!("Value {} out of bounds. {}", self.found, msg))
	}
}
