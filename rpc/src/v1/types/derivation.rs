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

use std::convert::TryFrom;
use std::fmt;
use serde::{Deserialize, Deserializer};
use serde::de::{Error, Visitor};

use ethereum_types::H256;

/// Type of derivation
pub enum DerivationType {
	/// Soft - allow proof of parent
	Soft,
	/// Hard - does not allow proof of parent
	Hard,
}

/// Derivation request by hash
#[derive(Deserialize)]
pub struct DeriveHash {
	hash: H256,
	#[serde(rename = "type")]
	d_type: DerivationType,
}

/// Node propertoes in hierarchical derivation request
#[derive(Deserialize)]
pub struct DeriveHierarchicalItem {
	index: u64,
	#[serde(rename = "type")]
	d_type: DerivationType,
}

/// Hierarchical (index sequence) request
pub type DeriveHierarchical = Vec<DeriveHierarchicalItem>;

/// Generic derivate request
pub enum Derive {
	/// Hierarchical (index sequence) request
	Hierarchical(DeriveHierarchical),
	/// Hash request
	Hash(DeriveHash),
}

impl From<DeriveHierarchical> for Derive {
	fn from(d: DeriveHierarchical) -> Self {
		Self::Hierarchical(d)
	}
}

impl From<DeriveHash> for Derive {
	fn from(d: DeriveHash) -> Self {
		Self::Hash(d)
	}
}

/// Error converting request data
#[cfg(any(test, feature = "accounts"))]
#[derive(Debug)]
pub enum ConvertError {
	IndexOverlfow(u64),
}

impl Derive {
	/// Convert to account provider struct dealing with possible overflows
	#[cfg(any(test, feature = "accounts"))]
	pub fn into_derivation(self) -> Result<ethstore::Derivation, ConvertError> {
		Ok(match self {
			Self::Hierarchical(drv) => {
				ethstore::Derivation::Hierarchical({
					let mut members = Vec::<ethstore::IndexDerivation>::new();
					for h in drv {
						let index = u32::try_from(h.index).map_err(|_| ConvertError::IndexOverlfow(h.index))?;
						let soft = match h.d_type {
							DerivationType::Soft => true,
							DerivationType::Hard => false,
						};
						members.push(ethstore::IndexDerivation { index, soft });
					}
					members
			   })
			},
			Self::Hash(drv) => {
				match drv.d_type {
					DerivationType::Soft => ethstore::Derivation::SoftHash(drv.hash),
					DerivationType::Hard => ethstore::Derivation::HardHash(drv.hash),
				}
			},
		})
	}
}

impl<'a> Deserialize<'a> for DerivationType {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
		deserializer.deserialize_any(DerivationTypeVisitor)
	}
}

struct DerivationTypeVisitor;

impl<'a> Visitor<'a> for DerivationTypeVisitor {
	type Value = DerivationType;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "'hard' or 'soft'")
	}

	fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: Error {
		match value {
			"soft" => Ok(DerivationType::Soft),
			"hard" => Ok(DerivationType::Hard),
			v => Err(Error::custom(format!("invalid derivation type: {:?}", v))),
		}
	}

	fn visit_string<E>(self, value: String) -> Result<Self::Value, E> where E: Error {
		self.visit_str(value.as_ref())
	}
}
