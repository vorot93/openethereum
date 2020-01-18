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

use crate::route::Out;

pub type Result<T> = std::result::Result<T, Error>;

/// IPFS server error
#[derive(Debug)]
pub enum ServerError {
	/// Wrapped `std::io::Error`
	IoError(std::io::Error),
	/// Other `hyper` error
	Other(http::hyper::error::Error),
	/// Invalid --ipfs-api-interface
	InvalidInterface
}

/// Handle IO errors (ports taken when starting the server).
impl From<std::io::Error> for ServerError {
	fn from(err: std::io::Error) -> Self {
		Self::IoError(err)
	}
}

impl From<http::hyper::error::Error> for ServerError {
	fn from(err: http::hyper::error::Error) -> Self {
		Self::Other(err)
	}
}

impl From<ServerError> for String {
	fn from(err: ServerError) -> Self {
		match err {
			ServerError::IoError(err) => err.to_string(),
			ServerError::Other(err) => err.to_string(),
			ServerError::InvalidInterface => "Invalid --ipfs-api-interface parameter".into(),
		}
	}
}

impl std::fmt::Display for ServerError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::IoError(err) => write!(f, "Io Error: {}", err),
			Self::Other(err) => write!(f, "Other error: {}", err),
			Self::InvalidInterface => write!(f, "Invalid interface"),
		}
	}
}

impl std::error::Error for ServerError {}

#[derive(Debug, PartialEq)]
pub enum Error {
	CidParsingFailed,
	UnsupportedHash,
	UnsupportedCid,
	BlockNotFound,
	TransactionNotFound,
	StateRootNotFound,
	ContractNotFound,
}

/// Convert Error into Out, handy when switching from Rust's Result-based
/// error handling to Hyper's request handling.
impl From<Error> for Out {
	fn from(err: Error) -> Self {
		use self::Error::*;

		match err {
			UnsupportedHash => Self::Bad("Hash must be Keccak-256"),
			UnsupportedCid => Self::Bad("CID codec not supported"),
			CidParsingFailed => Self::Bad("CID parsing failed"),
			BlockNotFound => Self::NotFound("Block not found"),
			TransactionNotFound => Self::NotFound("Transaction not found"),
			StateRootNotFound => Self::NotFound("State root not found"),
			ContractNotFound => Self::NotFound("Contract not found"),
		}
	}
}

/// Convert Content ID errors.
impl From<cid::Error> for Error {
	fn from(_: cid::Error) -> Self {
		Self::CidParsingFailed
	}
}

/// Convert multihash errors (multihash being part of CID).
impl From<multihash::Error> for Error {
	fn from(_: multihash::Error) -> Self {
		Self::CidParsingFailed
	}
}
