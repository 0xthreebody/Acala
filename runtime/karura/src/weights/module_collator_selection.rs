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

//! Autogenerated weights for module_collator_selection
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

/// Weight functions for module_collator_selection.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_collator_selection::WeightInfo for WeightInfo<T> {
	// Storage: CollatorSelection Invulnerables (r:0 w:1)
	/// The range of component `b` is `[1, 10]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Minimum execution time: 12_283 nanoseconds.
		Weight::from_ref_time(12_821_505)
			// Standard Error: 2_673
			.saturating_add(Weight::from_ref_time(65_241).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	fn set_desired_candidates() -> Weight {
		// Minimum execution time: 11_755 nanoseconds.
		Weight::from_ref_time(12_218_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	fn set_candidacy_bond() -> Weight {
		// Minimum execution time: 11_968 nanoseconds.
		Weight::from_ref_time(12_564_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection NonCandidates (r:1 w:1)
	// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	/// The range of component `c` is `[5, 50]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 49_049 nanoseconds.
		Weight::from_ref_time(49_583_529)
			// Standard Error: 3_797
			.saturating_add(Weight::from_ref_time(429_728).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:0)
	/// The range of component `c` is `[1, 50]`.
	fn register_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 28_825 nanoseconds.
		Weight::from_ref_time(33_576_929)
			// Standard Error: 3_543
			.saturating_add(Weight::from_ref_time(401_270).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: CollatorSelection NonCandidates (r:0 w:1)
	/// The range of component `c` is `[6, 50]`.
	fn leave_intent(c: u32, ) -> Weight {
		// Minimum execution time: 22_956 nanoseconds.
		Weight::from_ref_time(22_921_114)
			// Standard Error: 2_204
			.saturating_add(Weight::from_ref_time(310_006).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: CollatorSelection NonCandidates (r:1 w:1)
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	fn withdraw_bond() -> Weight {
		// Minimum execution time: 49_570 nanoseconds.
		Weight::from_ref_time(51_808_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: CollatorSelection SessionPoints (r:1 w:0)
	fn note_author() -> Weight {
		// Minimum execution time: 36_155 nanoseconds.
		Weight::from_ref_time(37_734_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: CollatorSelection Candidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	fn new_session() -> Weight {
		// Minimum execution time: 20_795 nanoseconds.
		Weight::from_ref_time(21_529_000)
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: Session Validators (r:1 w:0)
	// Storage: CollatorSelection Candidates (r:1 w:0)
	// Storage: CollatorSelection SessionPoints (r:0 w:50)
	/// The range of component `r` is `[5, 50]`.
	/// The range of component `c` is `[5, 50]`.
	fn start_session(r: u32, c: u32, ) -> Weight {
		// Minimum execution time: 17_279 nanoseconds.
		Weight::from_ref_time(13_246_345)
			// Standard Error: 1_627
			.saturating_add(Weight::from_ref_time(308).saturating_mul(r.into()))
			// Standard Error: 1_627
			.saturating_add(Weight::from_ref_time(1_067_302).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
	// Storage: CollatorSelection SessionPoints (r:51 w:50)
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: CollatorSelection NonCandidates (r:0 w:41)
	/// The range of component `r` is `[5, 50]`.
	/// The range of component `c` is `[5, 50]`.
	fn end_session(_r: u32, c: u32, ) -> Weight {
		// Minimum execution time: 38_375 nanoseconds.
		Weight::from_ref_time(402_118_446)
			// Standard Error: 15_407
			.saturating_add(Weight::from_ref_time(5_690_126).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(48))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
}
