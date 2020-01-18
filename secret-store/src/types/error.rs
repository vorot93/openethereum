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

use std::fmt;
use std::net;
use std::io::Error as IoError;

use crypto;

/// Secret store error.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Error {
	/// Invalid node address has been passed.
	InvalidNodeAddress,
	/// Invalid node id has been passed.
	InvalidNodeId,
	/// Session with the given id already exists.
	DuplicateSessionId,
	/// No active session with given id.
	NoActiveSessionWithId,
	/// Invalid threshold value has been passed.
	/// Threshold value must be in [0; n - 1], where n is a number of nodes participating in the encryption.
	NotEnoughNodesForThreshold,
	/// Current state of encryption/decryption session does not allow to proceed request.
	/// Reschedule this request for later processing.
	TooEarlyForRequest,
	/// Current state of encryption/decryption session does not allow to proceed request.
	/// This means that either there is some comm-failure or node is misbehaving/cheating.
	InvalidStateForRequest,
	/// Request cannot be sent/received from this node.
	InvalidNodeForRequest,
	/// Message or some data in the message was recognized as invalid.
	/// This means that node is misbehaving/cheating.
	InvalidMessage,
	/// Message version is not supported.
	InvalidMessageVersion,
	/// Message is invalid because of replay-attack protection.
	ReplayProtection,
	/// Connection to node, required for this session is not established.
	NodeDisconnected,
	/// Server key with this ID is already generated.
	ServerKeyAlreadyGenerated,
	/// Server key with this ID is not yet generated.
	ServerKeyIsNotFound,
	/// Document key with this ID is already stored.
	DocumentKeyAlreadyStored,
	/// Document key with this ID is not yet stored.
	DocumentKeyIsNotFound,
	/// Consensus is temporary unreachable. Means that something is currently blocking us from either forming
	/// consensus group (like disconnecting from too many nodes, which are AGREE to participate in consensus)
	/// or from rejecting request (disconnecting from AccessDenied-nodes).
	ConsensusTemporaryUnreachable,
	/// Consensus is unreachable. It doesn't mean that it will ALWAYS remain unreachable, but right NOW we have
	/// enough nodes confirmed that they do not want to be a part of consensus. Example: we're connected to 10
	/// of 100 nodes. Key threshold is 6 (i.e. 7 nodes are required for consensus). 4 nodes are responding with
	/// reject => consensus is considered unreachable, even though another 90 nodes still can respond with OK.
	ConsensusUnreachable,
	/// Acl storage error.
	AccessDenied,
	/// Can't start session, because exclusive session is active.
	ExclusiveSessionActive,
	/// Can't start exclusive session, because there are other active sessions.
	HasActiveSessions,
	/// Insufficient requester data.
	InsufficientRequesterData(String),
	/// Cryptographic error.
	EthKey(String),
	/// I/O error has occurred.
	Io(String),
	/// Deserialization error has occurred.
	Serde(String),
	/// Hyper error.
	Hyper(String),
	/// Database-related error.
	Database(String),
	/// Internal error.
	Internal(String),
}

impl Error {
	/// Is this a fatal error? Non-fatal means that it is possible to replay the same request with a non-zero
	/// chance to success. I.e. the error is not about request itself (or current environment factors that
	/// are affecting request processing), but about current `SecretStore` state.
	pub fn is_non_fatal(&self) -> bool {
		match self {
			// non-fatal errors:

			// session start errors => restarting session is a solution
			Self::DuplicateSessionId | Self::NoActiveSessionWithId |
			// unexpected message errors => restarting session/excluding node is a solution
			Self::TooEarlyForRequest | Self::InvalidStateForRequest | Self::InvalidNodeForRequest |
			// invalid message errors => restarting/updating/excluding node is a solution
			Self::InvalidMessage | Self::InvalidMessageVersion | Self::ReplayProtection |
			// connectivity problems => waiting for reconnect && restarting session is a solution
			Self::NodeDisconnected |
			// temporary (?) consensus problems, related to other non-fatal errors => restarting is probably (!) a solution
			Self::ConsensusTemporaryUnreachable |
			// exclusive session errors => waiting && restarting is a solution
			Self::ExclusiveSessionActive | Self::HasActiveSessions => true,

			// fatal errors:

			// config-related errors
			Self::InvalidNodeAddress | Self::InvalidNodeId |
			// wrong session input params errors
			Self::NotEnoughNodesForThreshold | Self::ServerKeyAlreadyGenerated | Self::ServerKeyIsNotFound |
				Self::DocumentKeyAlreadyStored | Self::DocumentKeyIsNotFound | Self::InsufficientRequesterData(_) |
			// access denied/consensus error
			Self::AccessDenied | Self::ConsensusUnreachable |
			// indeterminate internal errors, which could be either fatal (db failure, invalid request), or not (network error),
			// but we still consider these errors as fatal
			Self::EthKey(_) | Self::Serde(_) | Self::Hyper(_) | Self::Database(_) | Self::Internal(_) | Self::Io(_) => false,
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match self {
			Self::InvalidNodeAddress => write!(f, "invalid node address has been passed"),
			Self::InvalidNodeId => write!(f, "invalid node id has been passed"),
			Self::DuplicateSessionId => write!(f, "session with the same id is already registered"),
			Self::NoActiveSessionWithId => write!(f, "no active session with given id"),
			Self::NotEnoughNodesForThreshold => write!(f, "not enough nodes for passed threshold"),
			Self::TooEarlyForRequest => write!(f, "session is not yet ready to process this request"),
			Self::InvalidStateForRequest => write!(f, "session is in invalid state for processing this request"),
			Self::InvalidNodeForRequest => write!(f, "invalid node for this request"),
			Self::InvalidMessage => write!(f, "invalid message is received"),
			Self::InvalidMessageVersion => write!(f, "unsupported message is received"),
			Self::ReplayProtection => write!(f, "replay message is received"),
			Self::NodeDisconnected => write!(f, "node required for this operation is currently disconnected"),
			Self::ServerKeyAlreadyGenerated => write!(f, "Server key with this ID is already generated"),
			Self::ServerKeyIsNotFound => write!(f, "Server key with this ID is not found"),
			Self::DocumentKeyAlreadyStored => write!(f, "Document key with this ID is already stored"),
			Self::DocumentKeyIsNotFound => write!(f, "Document key with this ID is not found"),
			Self::ConsensusUnreachable => write!(f, "Consensus unreachable"),
			Self::ConsensusTemporaryUnreachable => write!(f, "Consensus temporary unreachable"),
			Self::AccessDenied => write!(f, "Access denied"),
			Self::ExclusiveSessionActive => write!(f, "Exclusive session active"),
			Self::HasActiveSessions => write!(f, "Unable to start exclusive session"),
			Self::InsufficientRequesterData(e) => write!(f, "Insufficient requester data: {}", e),
			Self::EthKey(e) => write!(f, "cryptographic error {}", e),
			Self::Hyper(msg) => write!(f, "Hyper error: {}", msg),
			Self::Serde(msg) => write!(f, "Serialization error: {}", msg),
			Self::Database(msg) => write!(f, "Database error: {}", msg),
			Self::Internal(msg) => write!(f, "Internal error: {}", msg),
			Self::Io(msg) => write!(f, "IO error: {}", msg),
		}
	}
}

impl From<crypto::publickey::Error> for Error {
	fn from(err: crypto::publickey::Error) -> Self {
		Self::EthKey(err.into())
	}
}

impl From<crypto::Error> for Error {
	fn from(err: crypto::Error) -> Self {
		Self::EthKey(err.to_string())
	}
}

impl From<IoError> for Error {
	fn from(err: IoError) -> Self {
		Self::Io(err.to_string())
	}
}

impl Into<String> for Error {
	fn into(self) -> String {
		format!("{}", self)
	}
}

impl From<net::AddrParseError> for Error {
	fn from(err: net::AddrParseError) -> Self {
		Self::Internal(err.to_string())
	}
}
