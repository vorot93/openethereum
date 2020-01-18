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

//! This crate allows automatic caching of `T.len()` with an api that 
//! allows drop in replacement for `parking_lot`
//! [`Mutex`](../lock_api/struct.Mutex.html)
//! and [`RwLock`](../lock_api/struct.RwLock.html) for most common use-cases.
//!
//! This crate implements `Len` for the following types: 
//! `std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap}`
//!
//! ## Example
//!
//! ```rust
//! extern crate len_caching_lock;
//! use len_caching_lock::LenCachingMutex;
//!
//! let vec: Vec<i32> = Vec::new();
//! let len_caching_mutex = LenCachingMutex::new(vec);
//! assert_eq!(len_caching_mutex.lock().len(), len_caching_mutex.load_len());
//! len_caching_mutex.lock().push(0);
//! assert_eq!(1, len_caching_mutex.load_len());
//! ```

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

extern crate parking_lot;
use std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap};
use std::hash::Hash;

pub mod mutex;
pub mod rwlock;

pub use mutex::LenCachingMutex;
pub use rwlock::LenCachingRwLock;

/// Implement to allow a type with a len() method to be used
/// with [`LenCachingMutex`](mutex/struct.LenCachingMutex.html)
/// or  [`LenCachingRwLock`](rwlock/struct.LenCachingRwLock.html)
pub trait Len {
	fn len(&self) -> usize;
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

impl<T> Len for Vec<T> {
	fn len(&self) -> usize { Self::len(self) }
}

impl<T> Len for VecDeque<T> {
	fn len(&self) -> usize { Self::len(self) }
}

impl<T> Len for LinkedList<T> {
	fn len(&self) -> usize { Self::len(self) }
}

impl<K: Eq + Hash, V, S> Len for HashMap<K, V, S> {
	fn len(&self) -> usize { Self::len(self) }
}

impl<K, V> Len for BTreeMap<K, V> {
	fn len(&self) -> usize { Self::len(self) }
}

impl<T: Eq + Hash, S> Len for HashSet<T, S> {
	fn len(&self) -> usize { Self::len(self) }
}

impl<T> Len for BTreeSet<T> {
	fn len(&self) -> usize { Self::len(self) }
}

impl<T: Ord> Len for BinaryHeap<T> {
	fn len(&self) -> usize { Self::len(self) }
}
