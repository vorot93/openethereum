// Copyright 2015-2019 Parity Technologies (UK) Ltd.
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

use fern::colors::{Color, ColoredLevelConfig};
use rlog::{LevelFilter};
use std::io;

/// Log levels.
#[derive(Debug, PartialEq, Clone)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    /// Returns a string value of the current log level
    pub fn value(&self) -> String {
        match *self {
            LogLevel::Error => "error".to_string(),
            LogLevel::Warn => "warn".to_string(),
            LogLevel::Info => "info".to_string(),
            LogLevel::Debug => "debug".to_string(),
            LogLevel::Trace => "trace".to_string(),
            _ => "error".to_string(),
        }
    }
}

fn setup_logger(verbosity: &String, logging_to_file: bool) -> Result<(), fern::InitError> {
	let colors_line = ColoredLevelConfig::new()
		.error(Color::Red)
		.warn(Color::Yellow)
		.info(Color::BrightBlack)
		.debug(Color::BrightBlack)
		.trace(Color::BrightBlack);

	let colors_level = colors_line.clone().info(Color::Green);

	let mut base_config = fern::Dispatch::new();

	base_config = match verbosity {
		LogLevel::Error.value() => {
			base_config
			.level(LevelFilter::Error)
			.level_for("pretty_colored", LevelFilter::Error)
		}
		LogLevel::Warn.value() => {
			base_config
			.level(LevelFilter::Warn)
			.level_for("pretty_colored", LevelFilter::Warn)
		},
		LogLevel::Info.value() => {
			base_config
			.level(LevelFilter::Info)
			.level_for("pretty_colored", LevelFilter::Info)
		},
		LogLevel::Debug.value() => {
			base_config
			.level(LevelFilter::Debug)
			.level_for("pretty_colored", LevelFilter::Debug)
		},
		LogLevel::Trace.value() => {
			base_config
			.level(LevelFilter::Trace)
			.level_for("pretty_colored", LevelFilter::Trace)
		},
		_ => {
			base_config
			.level(LevelFilter::Error)
			.level_for("pretty_colored", LevelFilter::Error)
		},
	};

	let format_config = fern::Dispatch::new()
		.format(move |out, message, record| {
			out.finish(format_args!(
				"{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
				color_line = format_args!("\x1B[{}m", colors_line.get_color(&record.level()).to_fg_str()),
				date = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
				level = colors_level.color(record.level()),
				target = record.target(),
				message = message
			))
		})
		.chain(io::stdout());

	if logging_to_file {
		let file_config = fern::Dispatch::new()
			.format(|out, message, record| {
				out.finish(format_args!(
				"{}[{}][{}] {}",
				chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
				record.level(),
				record.target(),
				message
				))
			})
			.chain(fern::log_file("output.log")?);

		base_config.chain(file_config).chain(format_config).apply()?;
	} else {
		base_config.chain(format_config).apply()?;
	}

  Ok(())
}

/// Initialisation of the [Log Crate](https://crates.io/crates/log) and [Fern Crate](https://docs.rs/fern/0.5.5/fern/)
///
/// - Choice of logging level verbosity from CLI: error, warn, info, debug, or trace.
/// - Fallback to default logging level that is defined.
/// - Use of logging level macros from highest priority to lowest: `error!`, `warn!`, `info!`, `debug!` and `trace!`.
/// - [Compile time filters](https://docs.rs/log/0.4.1/log/#compile-time-filters) that override the CLI logging levels
/// are configured in Cargo.toml. In production the max logging level may differ from in development.
/// - Output to output.log when logging_to_file is true.
pub fn init_logger(verbosity: &String, logging_to_file: bool) -> () {

	match setup_logger(verbosity, logging_to_file) {
		Ok(_) => {
			println!("Success initializing logger. Verbosity: {:?}. Log to file: {}", verbosity, &logging_to_file);
			()
		}
		Err(e) => { println!("Error initializing logger: {}", e); }
	}
}
