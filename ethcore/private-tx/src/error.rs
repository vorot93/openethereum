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

use std::error;
use derive_more::Display;
use ethereum_types::Address;
use rlp::DecoderError;
use ethtrie::TrieError;
use types::{
	errors::{EthcoreError, ExecutionError},
	transaction::Error as TransactionError,
};
use crypto::publickey::Error as CryptoError;
use txpool::VerifiedTransaction;
use private_transactions::VerifiedPrivateTransaction;
use serde_json::{Error as SerdeError};

type TxPoolError = txpool::Error<<VerifiedPrivateTransaction as VerifiedTransaction>::Hash>;

#[derive(Debug, Display)]
pub enum Error {
	/// Error concerning the Rust standard library's IO subsystem.
	#[display(fmt = "Io Error: {}", _0)]
	Io(::std::io::Error),
	/// RLP decoding error.
	#[display(fmt = "Decoder Error: {}", _0)]
	Decoder(DecoderError),
	/// Error concerning TrieDBs.
	#[display(fmt = "Trie Error: {}", _0)]
	Trie(TrieError),
	/// Transaction pool error.
	#[display(fmt = "Transaction Pool Error: {}", _0)]
	TxPool(TxPoolError),
	/// Crypto error.
	#[display(fmt = "Crypto Error {}", _0)]
	Crypto(CryptoError),
	/// Serialization error.
	#[display(fmt = "Serialization Error {}", _0)]
	Json(SerdeError),
	/// Encryption error.
	#[display(fmt = "Encryption error. ({})", _0)]
	Encrypt(String),
	/// Decryption error.
	#[display(fmt = "Decryption error. ({})", _0)]
	Decrypt(String),
	/// Address not authorized.
	#[display(fmt = "Private transaction execution is not authorised for {}", _0)]
	NotAuthorised(Address),
	/// Transaction creates more than one contract.
	#[display(fmt = "Private transaction created too many contracts")]
	TooManyContracts,
	/// Contract call error.
	#[display(fmt = "Contract call error. ({})", _0)]
	Call(String),
	/// State is not available.
	#[display(fmt = "State is not available")]
	StatePruned,
	/// State is incorrect.
	#[display(fmt = "State is incorrect")]
	StateIncorrect,
	/// Wrong private transaction type.
	#[display(fmt = "Wrong private transaction type")]
	BadTransactionType,
	/// Contract does not exist or was not created.
	#[display(fmt = "Contract does not exist or was not created")]
	ContractDoesNotExist,
	/// Reference to the client is corrupted.
	#[display(fmt = "Reference to the client is corrupted")]
	ClientIsMalformed,
	/// Queue of private transactions for verification is full.
	#[display(fmt = "Queue of private transactions for verification is full")]
	QueueIsFull,
	/// The transaction already exists in queue of private transactions.
	#[display(fmt = "The transaction already exists in queue of private transactions.")]
	PrivateTransactionAlreadyImported,
	/// The information about private transaction is not found in the store.
	#[display(fmt = "The information about private transaction is not found in the store.")]
	PrivateTransactionNotFound,
	/// Account for signing public transactions not set.
	#[display(fmt = "Account for signing public transactions not set.")]
	SignerAccountNotSet,
	/// Account for validating private transactions not set.
	#[display(fmt = "Account for validating private transactions not set.")]
	ValidatorAccountNotSet,
	/// Account for signing requests to key server not set.
	#[display(fmt = "Account for signing requests to key server not set.")]
	KeyServerAccountNotSet,
	/// Private state for the contract was not found in the local storage.
	#[display(fmt = "Private state for the contract was not found in the local storage.")]
	PrivateStateNotFound,
	/// Cannot write state to the local database.
	#[display(fmt = "Cannot write state to the local database.")]
	DatabaseWriteError,
	/// Encryption key is not found on key server.
	#[display(fmt = "Encryption key is not found on key server for {}", _0)]
	EncryptionKeyNotFound(Address),
	/// Key server URL is not set.
	#[display(fmt = "Key server URL is not set.")]
	KeyServerNotSet,
	/// Transaction not found in logs.
	#[display(fmt = "Private transaction not found in logs.")]
	TxNotFoundInLog,
	/// Path for logging not set.
	#[display(fmt = "Path for logging not set.")]
	LoggingPathNotSet,
	/// Timestamp overflow error.
	#[display(fmt = "Timestamp overflow error.")]
	TimestampOverflow,
	/// VM execution error.
	#[display(fmt = "VM execution error {}", _0)]
	Execution(ExecutionError),
	/// Error of transactions processing.
	#[display(fmt = "Error of transactions processing {}", _0)]
	Transaction(TransactionError),
	/// General ethcore error.
	#[display(fmt = "General ethcore error {}", _0)]
	Ethcore(EthcoreError),
	/// A convenient variant for String.
	#[display(fmt = "{}", _0)]
	Msg(String),
}

impl error::Error for Error {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match self {
			Self::Io(e) => Some(e),
			Self::Decoder(e) => Some(e),
			Self::Trie(e) => Some(e),
			Self::TxPool(e) => Some(e),
			Self::Json(e) => Some(e),
			Self::Crypto(e) => Some(e),
			Self::Execution(e) => Some(e),
			Self::Transaction(e) => Some(e),
			Self::Ethcore(e) => Some(e),
			_ => None,
		}
	}
}

impl From<String> for Error {
	fn from(s: String) -> Self {
		Self::Msg(s)
	}
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Self::Io(err)
	}
}

impl From<CryptoError> for Error {
	fn from(err: CryptoError) -> Self {
		Self::Crypto(err)
	}
}

impl From<DecoderError> for Error {
	fn from(err: DecoderError) -> Self {
		Self::Decoder(err)
	}
}

impl From<ExecutionError> for Error {
	fn from(err: ExecutionError) -> Self {
		Self::Execution(err)
	}
}

impl From<TransactionError> for Error {
	fn from(err: TransactionError) -> Self {
		Self::Transaction(err)
	}
}

impl From<TrieError> for Error {
	fn from(err: TrieError) -> Self {
		Self::Trie(err)
	}
}

impl From<TxPoolError> for Error {
	fn from(err: TxPoolError) -> Self {
		Self::TxPool(err)
	}
}

impl From<SerdeError> for Error {
	fn from(err: SerdeError) -> Self {
		Self::Json(err)
	}
}

impl From<EthcoreError> for Error {
	fn from(err: EthcoreError) -> Self {
		Self::Ethcore(err)
	}
}

impl<E> From<Box<E>> for Error where Error: From<E> {
	fn from(err: Box<E>) -> Self {
		Self::from(*err)
	}
}
