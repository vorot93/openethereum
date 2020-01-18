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

//! Account management (personal) rpc implementation
use std::sync::Arc;
use std::collections::{
	btree_map::{BTreeMap, Entry},
	HashSet,
};

use ethereum_types::{Address, H160, H256, H520};
use ethkey::{Brain, Password};
use crypto::publickey::{Generator, Secret};
use ethstore::KeyFile;
use accounts::AccountProvider;
use jsonrpc_core::Result;
use v1::helpers::deprecated::{self, DeprecationNotice};
use v1::helpers::errors;
use v1::traits::{ParityAccounts, ParityAccountsInfo};
use v1::types::{Derive, DeriveHierarchical, DeriveHash, ExtAccountInfo, AccountInfo};

/// Account management (personal) rpc implementation.
pub struct ParityAccountsClient {
	accounts: Arc<AccountProvider>,
	deprecation_notice: DeprecationNotice,
}

impl ParityAccountsClient {
	/// Creates new `ParityAccountsClient`
	pub fn new(store: &Arc<AccountProvider>) -> Self {
		Self {
			accounts: store.clone(),
			deprecation_notice: DeprecationNotice::default(),
		}
	}
}

impl ParityAccountsClient {
	fn deprecation_notice(&self, method: &'static str) {
		self.deprecation_notice.print(method, deprecated::msgs::ACCOUNTS);
	}
}

impl ParityAccountsInfo for ParityAccountsClient {
	fn accounts_info(&self) -> Result<BTreeMap<H160, AccountInfo>> {
		self.deprecation_notice("parity_accountsInfo");

		let dapp_accounts = self.accounts.accounts()
			.map_err(|e| errors::account("Could not fetch accounts.", e))?
			.into_iter().collect::<HashSet<_>>();

		let info = self.accounts.accounts_info().map_err(|e| errors::account("Could not fetch account info.", e))?;
		let other = self.accounts.addresses_info();

		Ok(info
			.into_iter()
			.chain(other)
			.filter_map(|(a, v)| if dapp_accounts.contains(&a) { Some((a, AccountInfo { name: v.name })) } else { None })
			.collect()
		)
	}

	fn default_account(&self) -> Result<H160> {
		self.deprecation_notice("parity_defaultAccount");

		Ok(self.accounts.default_account()
			.map(Into::into)
			.ok()
			.unwrap_or_default())
	}
}

impl ParityAccounts for ParityAccountsClient {
	fn all_accounts_info(&self) -> Result<BTreeMap<H160, ExtAccountInfo>> {
		let info = self.accounts.accounts_info().map_err(|e| errors::account("Could not fetch account info.", e))?;
		let other = self.accounts.addresses_info();

		let account_iter = info
			.into_iter()
			.chain(other.into_iter())
			.map(|(address, v)| (address, ExtAccountInfo {
				name: v.name,
				meta: v.meta,
				uuid: v.uuid
			}));

		let mut accounts: BTreeMap<H160, ExtAccountInfo> = BTreeMap::new();

		for (address, account) in account_iter {
			match accounts.entry(address) {
				// Insert only if occupied entry isn't already an account with UUID
				Entry::Occupied(mut occupied) if occupied.get().uuid.is_none() => {
					occupied.insert(account);
				},
				Entry::Vacant(vacant) => {
					vacant.insert(account);
				},
				_ => {}
			};
		}

		Ok(accounts)
	}

	fn new_account_from_phrase(&self, phrase: String, pass: Password) -> Result<H160> {
		self.deprecation_notice("parity_newAccountFromPhrase");
		let brain = Brain::new(phrase).generate().unwrap();
		self.accounts.insert_account(brain.secret().clone(), &pass)
			.map(Into::into)
			.map_err(|e| errors::account("Could not create account.", e))
	}

	fn new_account_from_wallet(&self, json: String, pass: Password) -> Result<H160> {
		self.deprecation_notice("parity_newAccountFromWallet");
		self.accounts.import_presale(json.as_bytes(), &pass)
			.or_else(|_| self.accounts.import_wallet(json.as_bytes(), &pass, true))
			.map(Into::into)
			.map_err(|e| errors::account("Could not create account.", e))
	}

	fn new_account_from_secret(&self, secret: H256, pass: Password) -> Result<H160> {
		self.deprecation_notice("parity_newAccountFromSecret");
		let secret = Secret::import_key(&secret.0)
			.map_err(|e| errors::account("Could not create account.", e))?;
		self.accounts.insert_account(secret, &pass)
			.map(Into::into)
			.map_err(|e| errors::account("Could not create account.", e))
	}

	fn test_password(&self, account: Address, password: Password) -> Result<bool> {
		self.deprecation_notice("parity_testPassword");
		self.accounts
			.test_password(&account, &password)
			.map_err(|e| errors::account("Could not fetch account info.", e))
	}

	fn change_password(&self, account: Address, password: Password, new_password: Password) -> Result<bool> {
		self.deprecation_notice("parity_changePassword");
		self.accounts
			.change_password(&account, password, new_password)
			.map(|_| true)
			.map_err(|e| errors::account("Could not fetch account info.", e))
	}

	fn kill_account(&self, account: Address, password: Password) -> Result<bool> {
		self.deprecation_notice("parity_killAccount");
		self.accounts
			.kill_account(&account, &password)
			.map(|_| true)
			.map_err(|e| errors::account("Could not delete account.", e))
	}

	fn remove_address(&self, addr: Address) -> Result<bool> {
		self.deprecation_notice("parity_removeAddresss");

		self.accounts.remove_address(addr);
		Ok(true)
	}

	fn set_account_name(&self, addr: Address, name: String) -> Result<bool> {
		self.deprecation_notice("parity_setAccountName");

		self.accounts.set_account_name(addr.clone(), name.clone())
			.unwrap_or_else(|_| self.accounts.set_address_name(addr, name));
		Ok(true)
	}

	fn set_account_meta(&self, addr: Address, meta: String) -> Result<bool> {
		self.deprecation_notice("parity_setAccountMeta");

		self.accounts.set_account_meta(addr.clone(), meta.clone())
			.unwrap_or_else(|_| self.accounts.set_address_meta(addr, meta));
		Ok(true)
	}

	fn import_geth_accounts(&self, addresses: Vec<Address>) -> Result<Vec<H160>> {
		self.deprecation_notice("parity_importGethAccounts");
		self.accounts
			.import_geth_accounts(into_vec(addresses), false)
			.map(into_vec)
			.map_err(|e| errors::account("Couldn't import Geth accounts", e))
	}

	fn geth_accounts(&self) -> Result<Vec<Address>> {
		self.deprecation_notice("parity_listGethAccounts");
		Ok(into_vec(self.accounts.list_geth_accounts(false)))
	}

	fn create_vault(&self, name: String, password: Password) -> Result<bool> {
		self.deprecation_notice("parity_newVault");

		self.accounts
			.create_vault(&name, &password)
			.map_err(|e| errors::account("Could not create vault.", e))
			.map(|_| true)
	}

	fn open_vault(&self, name: String, password: Password) -> Result<bool> {
		self.deprecation_notice("parity_openVault");

		self.accounts
			.open_vault(&name, &password)
			.map_err(|e| errors::account("Could not open vault.", e))
			.map(|_| true)
	}

	fn close_vault(&self, name: String) -> Result<bool> {
		self.deprecation_notice("parity_closeVault");

		self.accounts
			.close_vault(&name)
			.map_err(|e| errors::account("Could not close vault.", e))
			.map(|_| true)
	}

	fn list_vaults(&self) -> Result<Vec<String>> {
		self.deprecation_notice("parity_listVaults");

		self.accounts
			.list_vaults()
			.map_err(|e| errors::account("Could not list vaults.", e))
	}

	fn list_opened_vaults(&self) -> Result<Vec<String>> {
		self.deprecation_notice("parity_listOpenedVaults");

		self.accounts
			.list_opened_vaults()
			.map_err(|e| errors::account("Could not list vaults.", e))
	}

	fn change_vault_password(&self, name: String, new_password: Password) -> Result<bool> {
		self.deprecation_notice("parity_changeVaultPassword");

		self.accounts
			.change_vault_password(&name, &new_password)
			.map_err(|e| errors::account("Could not change vault password.", e))
			.map(|_| true)
	}

	fn change_vault(&self, address: H160, new_vault: String) -> Result<bool> {
		self.deprecation_notice("parity_changeVault");
		self.accounts
			.change_vault(address, &new_vault)
			.map_err(|e| errors::account("Could not change vault.", e))
			.map(|_| true)
	}

	fn get_vault_meta(&self, name: String) -> Result<String> {
		self.deprecation_notice("parity_getVaultMeta");

		self.accounts
			.get_vault_meta(&name)
			.map_err(|e| errors::account("Could not get vault metadata.", e))
	}

	fn set_vault_meta(&self, name: String, meta: String) -> Result<bool> {
		self.deprecation_notice("parity_setVaultMeta");

		self.accounts
			.set_vault_meta(&name, &meta)
			.map_err(|e| errors::account("Could not update vault metadata.", e))
			.map(|_| true)
	}

	fn derive_key_index(&self, addr: H160, password: Password, derivation: DeriveHierarchical, save_as_account: bool) -> Result<H160> {
		self.deprecation_notice("parity_deriveAddressIndex");
		self.accounts
			.derive_account(
				&addr,
				Some(password),
				Derive::from(derivation).into_derivation()
					.map_err(|c| errors::account("Could not parse derivation request: {:?}", c))?,
				save_as_account)
			.map(Into::into)
			.map_err(|e| errors::account("Could not derive account.", e))
	}

	fn derive_key_hash(&self, addr: H160, password: Password, derivation: DeriveHash, save_as_account: bool) -> Result<H160> {
		self.deprecation_notice("parity_deriveAddressHash");
		self.accounts
			.derive_account(
				&addr,
				Some(password),
				Derive::from(derivation).into_derivation()
					.map_err(|c| errors::account("Could not parse derivation request: {:?}", c))?,
				save_as_account)
			.map(Into::into)
			.map_err(|e| errors::account("Could not derive account.", e))
	}

	fn export_account(&self, addr: H160, password: Password) -> Result<KeyFile> {
		self.deprecation_notice("parity_exportAccount");
		self.accounts
			.export_account(
				&addr,
				password,
			)
			.map(Into::into)
			.map_err(|e| errors::account("Could not export account.", e))
	}

	fn sign_message(&self, addr: H160, password: Password, message: H256) -> Result<H520> {
		self.deprecation_notice("parity_signMessage");
		self.accounts
			.sign(
				addr,
				Some(password),
				message
			)
			.map(Into::into)
			.map_err(|e| errors::account("Could not sign message.", e))
	}
}

fn into_vec<A, B>(a: Vec<A>) -> Vec<B> where
	A: Into<B>
{
	a.into_iter().map(Into::into).collect()
}
