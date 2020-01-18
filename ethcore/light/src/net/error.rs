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

//! Defines error types and levels of punishment to use upon
//! encountering.

use std::fmt;
use {rlp, network};

/// Levels of punishment.
///
/// Currently just encompasses two different kinds of disconnect and
/// no punishment, but this is where reputation systems might come into play.
// In ascending order
#[derive(Debug, PartialEq, Eq)]
pub enum Punishment {
	/// Perform no punishment.
	None,
	/// Disconnect the peer, but don't prevent them from reconnecting.
	Disconnect,
	/// Disconnect the peer and prevent them from reconnecting.
	Disable,
}

/// Kinds of errors which can be encountered in the course of LES.
#[derive(Debug)]
pub enum Error {
	/// An RLP decoding error.
	Rlp(rlp::DecoderError),
	/// A network error.
	Network(network::Error),
	/// Out of credits.
	NoCredits,
	/// Unrecognized packet code.
	UnrecognizedPacket(u8),
	/// Unexpected handshake.
	UnexpectedHandshake,
	/// Peer on wrong network (wrong NetworkId or genesis hash)
	WrongNetwork,
	/// Unknown peer.
	UnknownPeer,
	/// Unsolicited response.
	UnsolicitedResponse,
	/// Bad back-reference in request.
	BadBackReference,
	/// Not a server.
	NotServer,
	/// Unsupported protocol version.
	UnsupportedProtocolVersion(u8),
	/// Bad protocol version.
	BadProtocolVersion,
	/// Peer is overburdened.
	Overburdened,
	/// No handler kept the peer.
	RejectedByHandlers,
}

impl Error {
	/// What level of punishment does this error warrant?
	pub fn punishment(&self) -> Punishment {
		match self {
			Self::Rlp(_) => Punishment::Disable,
			Self::Network(_) => Punishment::None,
			Self::NoCredits => Punishment::Disable,
			Self::UnrecognizedPacket(_) => Punishment::Disconnect,
			Self::UnexpectedHandshake => Punishment::Disconnect,
			Self::WrongNetwork => Punishment::Disable,
			Self::UnknownPeer => Punishment::Disconnect,
			Self::UnsolicitedResponse => Punishment::Disable,
			Self::BadBackReference => Punishment::Disable,
			Self::NotServer => Punishment::Disable,
			Self::UnsupportedProtocolVersion(_) => Punishment::Disable,
			Self::BadProtocolVersion => Punishment::Disable,
			Self::Overburdened => Punishment::None,
			Self::RejectedByHandlers => Punishment::Disconnect,
		}
	}
}

impl From<rlp::DecoderError> for Error {
	fn from(err: rlp::DecoderError) -> Self {
		Self::Rlp(err)
	}
}

impl From<network::Error> for Error {
	fn from(err: network::Error) -> Self {
		Self::Network(err)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Rlp(err) => err.fmt(f),
			Self::Network(err) => err.fmt(f),
			Self::NoCredits => write!(f, "Out of request credits"),
			Self::UnrecognizedPacket(code) => write!(f, "Unrecognized packet: 0x{:x}", code),
			Self::UnexpectedHandshake => write!(f, "Unexpected handshake"),
			Self::WrongNetwork => write!(f, "Wrong network"),
			Self::UnknownPeer => write!(f, "Unknown peer"),
			Self::UnsolicitedResponse => write!(f, "Peer provided unsolicited data"),
			Self::BadBackReference => write!(f, "Bad back-reference in request."),
			Self::NotServer => write!(f, "Peer not a server."),
			Self::UnsupportedProtocolVersion(pv) => write!(f, "Unsupported protocol version: {}", pv),
			Self::BadProtocolVersion => write!(f, "Bad protocol version in handshake"),
			Self::Overburdened => write!(f, "Peer overburdened"),
			Self::RejectedByHandlers => write!(f, "No handler kept this peer"),
		}
	}
}
