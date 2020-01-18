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

//! Custom panic hook with bug report link

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

extern crate backtrace;

use std::panic::{self, PanicInfo};
use std::thread;
use std::process;
use backtrace::Backtrace;

/// Set the panic hook to write to stderr and abort the process when a panic happens.
pub fn set_abort() {
	set_with(|msg| {
		eprintln!("{}", msg);
		process::abort()
	});
}

/// Set the panic hook with a closure to be called. The closure receives the panic message.
///
/// Depending on how Parity was compiled, after the closure has been executed, either the process
/// aborts or unwinding starts.
///
/// If you panic within the closure, a double panic happens and the process will stop.
pub fn set_with<F>(f: F)
where F: Fn(&str) + Send + Sync + 'static
{
	panic::set_hook(Box::new(move |info| {
		let msg = gen_panic_msg(info);
		f(&msg);
	}));
}

static ABOUT_PANIC: &str = "
This is a bug. Please report it at:

    https://github.com/paritytech/parity-ethereum/issues/new
";

fn gen_panic_msg(info: &PanicInfo) -> String {
	let location = info.location();
	let file = location.as_ref().map_or("<unknown>", |l| l.file());
	let line = location.as_ref().map_or(0, |l| l.line());

	let msg = if let Some(s) = info.payload().downcast_ref::<&'static str>() {
		*s
	} else if let Some(s) = info.payload().downcast_ref::<String>() {
		&s[..]
	} else {
		"Box<Any>"
	};

	let thread = thread::current();
	let name = thread.name().unwrap_or("<unnamed>");

	let backtrace = Backtrace::new();

	format!(r#"

====================

{backtrace:?}

Thread '{name}' panicked at '{msg}', {file}:{line}
{about}
"#, backtrace = backtrace, name = name, msg = msg, file = file, line = line, about = ABOUT_PANIC)
}
