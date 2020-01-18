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

//! Ethereum blooms database

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

mod db;
mod file;

use std::io;
use std::path::Path;

use parking_lot::Mutex;

/// Threadsafe API for blooms database.
///
/// # Warning
///
/// This database does not guarantee atomic writes.
pub struct Database {
	database: Mutex<db::Database>,
}

impl Database {
	/// Creates new database handle.
	///
	/// # Arguments
	///
	/// * `path` - database directory
	pub fn open<P>(path: P) -> io::Result<Self> where P: AsRef<Path> {
		let result = Self {
			database: Mutex::new(db::Database::open(path)?),
		};

		Ok(result)
	}

	/// Closes the inner database
	pub fn close(&self) -> io::Result<()> {
		self.database.lock().close()
	}

	/// Reopens database at the same location.
	pub fn reopen(&self) -> io::Result<()> {
		self.database.lock().reopen()
	}

	/// Inserts one or more blooms into database.
	///
	/// # Arguments
	///
	/// * `from` - index of the first bloom that needs to be inserted
	/// * `blooms` - iterator over blooms
	pub fn insert_blooms<'a, I, B>(&self, from: u64, blooms: I) -> io::Result<()>
	where ethbloom::BloomRef<'a>: From<B>, I: Iterator<Item = B> {
		self.database.lock().insert_blooms(from, blooms)
	}

	/// Returns indexes of all headers matching given bloom in a specified range.
	///
	/// # Arguments
	///
	/// * `from` - index of the first bloom that needs to be checked
	/// * `to` - index of the last bloom that needs to be checked (inclusive range)
	/// * `blooms` - searched pattern
	pub fn filter<'a, B, I, II>(&self, from: u64, to: u64, blooms: II) -> io::Result<Vec<u64>>
	where ethbloom::BloomRef<'a>: From<B>, II: IntoIterator<Item = B, IntoIter = I> + Copy, I: Iterator<Item = B> {
		self.database.lock()
			.iterate_matching(from, to, blooms)?
			.collect::<Result<Vec<u64>, _>>()
	}
}
