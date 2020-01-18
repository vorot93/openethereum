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

extern crate fetch;
extern crate hyper;
extern crate futures;

use hyper::{StatusCode, Body};
use futures::{future, future::FutureResult};
use fetch::{Fetch, Url, Request};

#[derive(Clone, Default)]
pub struct FakeFetch<T> where T: Clone + Send + Sync {
	val: Option<T>,
}

impl<T> FakeFetch<T> where T: Clone + Send + Sync {
	pub fn new(t: Option<T>) -> Self {
		Self { val : t }
	}
}

impl<T: 'static> Fetch for FakeFetch<T> where T: Clone + Send+ Sync {
	type Result = FutureResult<fetch::Response, fetch::Error>;

	fn fetch(&self, request: Request, abort: fetch::Abort) -> Self::Result {
		let u = request.url().clone();
		future::ok(if self.val.is_some() {
			let r = hyper::Response::new("Some content".into());
			fetch::client::Response::new(u, r, abort)
		} else {
			let r = hyper::Response::builder()
				.status(StatusCode::NOT_FOUND)
				.body(Body::empty()).expect("Nothing to parse, can not fail; qed");
			fetch::client::Response::new(u, r, abort)
		})
	}

	fn get(&self, url: &str, abort: fetch::Abort) -> Self::Result {
		let url: Url = match url.parse() {
			Ok(u) => u,
			Err(e) => return future::err(e.into())
		};
		self.fetch(Request::get(url), abort)
	}

	fn post(&self, url: &str, abort: fetch::Abort) -> Self::Result {
		let url: Url = match url.parse() {
			Ok(u) => u,
			Err(e) => return future::err(e.into())
		};
		self.fetch(Request::post(url), abort)
	}
}
