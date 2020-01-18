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

//! DB Migration module.

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

#[macro_use]
extern crate log;
#[macro_use]
extern crate macros;

extern crate kvdb;
extern crate kvdb_rocksdb;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fs, io, error};

use kvdb::DBTransaction;
use kvdb_rocksdb::{CompactionProfile, Database, DatabaseConfig};

fn other_io_err<E>(e: E) -> io::Error where E: Into<Box<dyn error::Error + Send + Sync>> {
	io::Error::new(io::ErrorKind::Other, e)
}

/// Migration config.
#[derive(Clone)]
pub struct Config {
	/// Defines how many elements should be migrated at once.
	pub batch_size: usize,
	/// Database compaction profile.
	pub compaction_profile: CompactionProfile,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			batch_size: 1024,
			compaction_profile: CompactionProfile::default(),
		}
	}
}

/// A batch of key-value pairs to be written into the database.
pub struct Batch {
	inner: BTreeMap<Vec<u8>, Vec<u8>>,
	batch_size: usize,
	column: u32,
}

impl Batch {
	/// Make a new batch with the given config.
	pub fn new(config: &Config, column: u32) -> Self {
		Self {
			inner: BTreeMap::new(),
			batch_size: config.batch_size,
			column,
		}
	}

	/// Insert a value into the batch, committing if necessary.
	pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>, dest: &mut Database) -> io::Result<()> {
		self.inner.insert(key, value);
		if self.inner.len() == self.batch_size {
			self.commit(dest)?;
		}
		Ok(())
	}

	/// Commit all the items in the batch to the given database.
	pub fn commit(&mut self, dest: &mut Database) -> io::Result<()> {
		if self.inner.is_empty() { return Ok(()) }

		let mut transaction = DBTransaction::new();

		for keypair in &self.inner {
			transaction.put(self.column, keypair.0, keypair.1);
		}

		self.inner.clear();
		dest.write(transaction)
	}
}

/// A generalized migration from the given db to a destination db.
pub trait Migration {
	/// Number of columns in the database before the migration.
	fn pre_columns(&self) -> u32 { self.columns() }
	/// Number of columns in database after the migration.
	fn columns(&self) -> u32;
	/// Whether this migration alters any existing columns.
	/// if not, then column families will simply be added and `migrate` will never be called.
	fn alters_existing(&self) -> bool { true }
	/// Version of the database after the migration.
	fn version(&self) -> u32;
	/// Migrate a source to a destination.
	fn migrate(&mut self, source: Arc<Database>, config: &Config, destination: &mut Database, col: u32) -> io::Result<()>;
}

/// A simple migration over key-value pairs of a single column.
pub trait SimpleMigration {
	/// Number of columns in database after the migration.
	fn columns(&self) -> u32;
	/// Version of database after the migration.
	fn version(&self) -> u32;
	/// Index of column which should be migrated.
	fn migrated_column_index(&self) -> u32;
	/// Should migrate existing object to new database.
	/// Returns `None` if the object does not exist in new version of database.
	fn simple_migrate(&mut self, key: Vec<u8>, value: Vec<u8>) -> Option<(Vec<u8>, Vec<u8>)>;
}

impl<T: SimpleMigration> Migration for T {
	fn columns(&self) -> u32 { SimpleMigration::columns(self) }

	fn alters_existing(&self) -> bool { true }

	fn version(&self) -> u32 { SimpleMigration::version(self) }

	fn migrate(&mut self, source: Arc<Database>, config: &Config, dest: &mut Database, col: u32) -> io::Result<()> {
		let migration_needed = col == SimpleMigration::migrated_column_index(self);
		let mut batch = Batch::new(config, col);

		for (key, value) in source.iter(col) {
			if migration_needed {
				if let Some((key, value)) = self.simple_migrate(key.into_vec(), value.into_vec()) {
					batch.insert(key, value, dest)?;
				}
			} else {
				batch.insert(key.into_vec(), value.into_vec(), dest)?;
			}
		}

		batch.commit(dest)
	}
}

/// An even simpler migration which just changes the number of columns.
pub struct ChangeColumns {
	/// The amount of columns before this migration.
	pub pre_columns: u32,
	/// The amount of columns after this migration.
	pub post_columns: u32,
	/// The version after this migration.
	pub version: u32,
}

impl Migration for ChangeColumns {
	fn pre_columns(&self) -> u32 { self.pre_columns }
	fn columns(&self) -> u32 { self.post_columns }
	fn alters_existing(&self) -> bool { false }
	fn version(&self) -> u32 { self.version }
	fn migrate(&mut self, _: Arc<Database>, _: &Config, _: &mut Database, _: u32) -> io::Result<()> {
		Ok(())
	}
}

/// Get the path where all databases reside.
fn database_path(path: &Path) -> PathBuf {
	let mut temp_path = path.to_owned();
	temp_path.pop();
	temp_path
}

enum TempIndex {
	One,
	Two,
}

impl TempIndex {
	fn swap(&mut self) {
		match self {
			Self::One => *self = Self::Two,
			Self::Two => *self = Self::One,
		}
	}

	// given the path to the old database, get the path of this one.
	fn path(&self, db_root: &Path) -> PathBuf {
		let mut buf = db_root.to_owned();

		match self {
			Self::One => buf.push("temp_migration_1"),
			Self::Two => buf.push("temp_migration_2"),
		};

		buf
	}
}

/// Manages database migration.
pub struct Manager {
	config: Config,
	migrations: Vec<Box<dyn Migration>>,
}

impl Manager {
	/// Creates new migration manager with given configuration.
	pub fn new(config: Config) -> Self {
		Self {
			config,
			migrations: Vec::new(),
		}
	}

	/// Adds new migration rules.
	pub fn add_migration<T: 'static>(&mut self, migration: T) -> io::Result<()> where T: Migration {
		let is_new = match self.migrations.last() {
			Some(last) => migration.version() > last.version(),
			None => true,
		};

		if is_new {
			self.migrations.push(Box::new(migration));
		} else {
			return Err(other_io_err("Cannot add migration."));
		}

		Ok(())
	}

	/// Performs migration in order, starting with a source path, migrating between two temporary databases,
	/// and producing a path where the final migration lives.
	pub fn execute(&mut self, old_path: &Path, version: u32) -> io::Result<PathBuf> {
		let config = self.config.clone();
		let migrations = self.migrations_from(version);
		trace!(target: "migration", "Total migrations to execute for version {}: {}", version, migrations.len());
		if migrations.is_empty() {
			return Err(other_io_err("Migration impossible"));
		};

		let columns = migrations.first().expect("checked empty above; qed").pre_columns();
		trace!(target: "migration", "Expecting database to contain {} columns", columns);
		let mut db_config = DatabaseConfig {
			max_open_files: 64,
			compaction: config.compaction_profile,
			columns,
			..DatabaseConfig::default()
		};

		let db_root = database_path(old_path);
		let mut temp_idx = TempIndex::One;
		let mut temp_path = old_path.to_path_buf();

		// start with the old db.
		let old_path_str = old_path.to_str().ok_or_else(|| other_io_err("Migration impossible."))?;
		let mut cur_db = Arc::new(Database::open(&db_config, old_path_str)?);

		for migration in migrations {
			trace!(target: "migration", "starting migration to version {}", migration.version());
			// Change number of columns in new db
			let current_columns = db_config.columns;
			db_config.columns = migration.columns();

			// slow migrations: alter existing data.
			if migration.alters_existing() {
				temp_path = temp_idx.path(&db_root);

				// open the target temporary database.
				let temp_path_str = temp_path.to_str().ok_or_else(|| other_io_err("Migration impossible."))?;
				let mut new_db = Database::open(&db_config, temp_path_str)?;

				for col in 0..current_columns {
					migration.migrate(cur_db.clone(), &config, &mut new_db, col)?
				}

				// next iteration, we will migrate from this db into the other temp.
				cur_db = Arc::new(new_db);
				temp_idx.swap();

				// remove the other temporary migration database.
				let _ = fs::remove_dir_all(temp_idx.path(&db_root));
			} else {
				// migrations which simply add or remove column families.
				// we can do this in-place.
				let goal_columns = migration.columns();
				while cur_db.num_columns() < goal_columns {
					cur_db.add_column().map_err(other_io_err)?;
				}

				while cur_db.num_columns() > goal_columns {
					cur_db.remove_last_column().map_err(other_io_err)?;
				}
			}
		}
		Ok(temp_path)
	}

	/// Returns true if migration is needed.
	pub fn is_needed(&self, version: u32) -> bool {
		match self.migrations.last() {
			Some(last) => version < last.version(),
			None => false,
		}
	}

	/// Find all needed migrations.
	fn migrations_from(&mut self, version: u32) -> Vec<&mut (dyn Migration + 'static)> {
		self.migrations.iter_mut().filter_map(|m| if m.version() > version { Some(&mut **m) } else { None }).collect()
	}
}

/// Prints a dot every `max` ticks
pub struct Progress {
	current: usize,
	max: usize,
}

impl Default for Progress {
	fn default() -> Self {
		Self {
			current: 0,
			max: 100_000,
		}
	}
}

impl Progress {
	/// Tick progress meter.
	pub fn tick(&mut self) {
		self.current += 1;
		if self.current == self.max {
			self.current = 0;
			flush!(".");
		}
	}
}
