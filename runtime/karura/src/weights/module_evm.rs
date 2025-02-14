// This file is part of Acala.

// Copyright (C) 2020-2023 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_evm
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `187e78510d7a`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_evm.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_evm::WeightInfo for WeightInfo<T> {
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create() -> Weight {
		// Minimum execution time: 127_439 nanoseconds.
		Weight::from_ref_time(133_403_000)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create2() -> Weight {
		// Minimum execution time: 124_294 nanoseconds.
		Weight::from_ref_time(128_313_000)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: EVM NetworkContractIndex (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create_nft_contract() -> Weight {
		// Minimum execution time: 139_572 nanoseconds.
		Weight::from_ref_time(145_583_000)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(10))
	}
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create_predeploy_contract() -> Weight {
		// Minimum execution time: 139_952 nanoseconds.
		Weight::from_ref_time(145_141_000)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM Accounts (r:2 w:1)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Codes (r:1 w:0)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn call() -> Weight {
		// Minimum execution time: 130_564 nanoseconds.
		Weight::from_ref_time(134_686_000)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: EVM Accounts (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	fn transfer_maintainer() -> Weight {
		// Minimum execution time: 97_219 nanoseconds.
		Weight::from_ref_time(98_344_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM Accounts (r:1 w:1)
	fn publish_contract() -> Weight {
		// Minimum execution time: 114_325 nanoseconds.
		Weight::from_ref_time(116_053_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EVM Accounts (r:1 w:1)
	fn publish_free() -> Weight {
		// Minimum execution time: 22_701 nanoseconds.
		Weight::from_ref_time(23_896_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Balances Reserves (r:1 w:1)
	fn enable_contract_development() -> Weight {
		// Minimum execution time: 100_777 nanoseconds.
		Weight::from_ref_time(101_380_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Balances Reserves (r:1 w:1)
	fn disable_contract_development() -> Weight {
		// Minimum execution time: 101_187 nanoseconds.
		Weight::from_ref_time(102_353_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EVM Accounts (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM CodeInfos (r:2 w:2)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	// Storage: EVM Codes (r:0 w:2)
	/// The range of component `c` is `[0, 61440]`.
	fn set_code(c: u32, ) -> Weight {
		// Minimum execution time: 159_193 nanoseconds.
		Weight::from_ref_time(157_484_863)
			// Standard Error: 23
			.saturating_add(Weight::from_ref_time(5_803).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM Accounts (r:1 w:1)
	// Storage: EvmAccounts Accounts (r:1 w:0)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	// Storage: IdleScheduler NextTaskId (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: IdleScheduler Tasks (r:0 w:1)
	// Storage: EVM Codes (r:0 w:1)
	fn selfdestruct() -> Weight {
		// Minimum execution time: 123_502 nanoseconds.
		Weight::from_ref_time(125_041_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(7))
	}
}
