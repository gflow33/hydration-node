// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


//! Autogenerated weights for `orml_vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-05-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// ./target/release/hydradx
// benchmark
// pallet
// --wasm-execution=compiled
// --pallet
// *
// --extrinsic
// *
// --heap-pages
// 4096
// --steps
// 50
// --repeat
// 20
// --template=scripts/pallet-weight-template.hbs
// --output
// weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `orml_vesting`.
pub struct WeightInfo<T>(PhantomData<T>);

/// Weights for `orml_vesting` using the HydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> orml_vesting::WeightInfo for HydraWeight<T> {
	/// Storage: `Vesting::VestingSchedules` (r:1 w:1)
	/// Proof: `Vesting::VestingSchedules` (`max_values`: None, `max_size`: Some(2850), added: 5325, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn vested_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1888`
		//  Estimated: `6315`
		// Minimum execution time: 93_625_000 picoseconds.
		Weight::from_parts(94_750_000, 6315)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Vesting::VestingSchedules` (r:1 w:1)
	/// Proof: `Vesting::VestingSchedules` (`max_values`: None, `max_size`: Some(2850), added: 5325, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	fn claim(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2007 + i * (18 ±0)`
		//  Estimated: `6315`
		// Minimum execution time: 53_425_000 picoseconds.
		Weight::from_parts(55_058_378, 6315)
			// Standard Error: 551
			.saturating_add(Weight::from_parts(98_382, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `Vesting::VestingSchedules` (r:0 w:1)
	/// Proof: `Vesting::VestingSchedules` (`max_values`: None, `max_size`: Some(2850), added: 5325, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	fn update_vesting_schedules(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1589`
		//  Estimated: `4764`
		// Minimum execution time: 45_123_000 picoseconds.
		Weight::from_parts(46_153_805, 4764)
			// Standard Error: 356
			.saturating_add(Weight::from_parts(95_410, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}