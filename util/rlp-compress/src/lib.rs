// Copyright 2015-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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

extern crate elastic_array;
#[macro_use]
extern crate lazy_static;
extern crate rlp;

mod common;

use std::cmp;
use std::collections::HashMap;
use elastic_array::ElasticArray1024;
use rlp::{Rlp, RlpStream};
use common::{SNAPSHOT_SWAPPER, BLOCKS_SWAPPER};

pub fn snapshot_swapper() -> &'static Swapper<'static> {
	&SNAPSHOT_SWAPPER as &Swapper
}

pub fn blocks_swapper() -> &'static Swapper<'static> {
	&BLOCKS_SWAPPER as &Swapper
}

/// A trait used to compress rlp.
pub trait Compressor {
	/// Get compressed version of given rlp.
	fn compressed(&self, rlp: &[u8]) -> Option<&[u8]>;
}

/// A trait used to convert compressed rlp into it's original version.
pub trait Decompressor {
	/// Get decompressed rlp.
	fn decompressed(&self, compressed: &[u8]) -> Option<&[u8]>;
}

/// Call this function to compress rlp.
pub fn compress(c: &[u8], swapper: &dyn Compressor) -> ElasticArray1024<u8> {
	let rlp = Rlp::new(c);
	if rlp.is_data() {
		ElasticArray1024::from_slice(swapper.compressed(rlp.as_raw()).unwrap_or_else(|| rlp.as_raw()))
	} else {
		map_rlp(&rlp, |r| compress(r.as_raw(), swapper))
	}
}

/// Call this function to decompress rlp.
pub fn decompress(c: &[u8], swapper: &dyn Decompressor) -> ElasticArray1024<u8> {
	let rlp = Rlp::new(c);
	if rlp.is_data() {
		ElasticArray1024::from_slice(swapper.decompressed(rlp.as_raw()).unwrap_or_else(|| rlp.as_raw()))
	} else {
		map_rlp(&rlp, |r| decompress(r.as_raw(), swapper))
	}
}

fn map_rlp<F: Fn(&Rlp) -> ElasticArray1024<u8>>(rlp: &Rlp, f: F) -> ElasticArray1024<u8> {
	let mut stream = RlpStream::new_list(rlp.item_count().unwrap_or_default());
	for subrlp in rlp.iter() {
		stream.append_raw(&f(&subrlp), 1);
	}
	stream.drain().as_slice().into()
}

/// Stores RLPs used for compression
pub struct Swapper<'a> {
	compressed_to_rlp: HashMap<&'a [u8], &'a [u8]>,
	rlp_to_compressed: HashMap<&'a [u8], &'a [u8]>,
}

impl<'a> Swapper<'a> {
	/// Construct a swapper from a list of common RLPs
	pub fn new(rlps_to_swap: &[&'a [u8]], compressed: &[&'a [u8]]) -> Self {
		if rlps_to_swap.len() > 0x7e {
			panic!("Invalid usage, only 127 RLPs can be swappable.");
		}

		let items = cmp::min(rlps_to_swap.len(), compressed.len());
		let mut compressed_to_rlp = HashMap::with_capacity(items);
		let mut rlp_to_compressed = HashMap::with_capacity(items);

		for (&rlp, &compressed) in rlps_to_swap.iter().zip(compressed.iter()) {
			compressed_to_rlp.insert(compressed, rlp);
			rlp_to_compressed.insert(rlp, compressed);
		}

		Self {
			compressed_to_rlp,
			rlp_to_compressed,
		}
	}
}

impl<'a> Decompressor for Swapper<'a> {
	fn decompressed(&self, compressed: &[u8]) -> Option<&[u8]> {
		self.compressed_to_rlp.get(compressed).cloned()
	}
}

impl<'a> Compressor for Swapper<'a> {
	fn compressed(&self, rlp: &[u8]) -> Option<&[u8]> {
		self.rlp_to_compressed.get(rlp).cloned()
	}
}
