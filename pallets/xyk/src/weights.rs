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


//! Autogenerated weights for `pallet_xyk`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-20, STEPS: `10`, REPEAT: `30`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=30
// --wasm-execution=compiled
// --heap-pages=4096
// --template=scripts/pallet-weight-template.hbs
// --pallet=pallet-xyk
// --output=weights/xyk.rs
// --extrinsic=*

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_xyk.
pub trait WeightInfo {
	fn create_pool() -> Weight;
	fn add_liquidity() -> Weight;
	fn remove_liquidity() -> Weight;
	fn sell() -> Weight;
	fn buy() -> Weight;
	fn router_execution_sell(c: u32, e: u32) -> Weight;
	fn router_execution_buy(c: u32, e: u32) -> Weight;
	fn calculate_spot_price_with_fee() -> Weight;
}

/// Weights for amm using the hydraDX node and recommended hardware.
impl WeightInfo for () {
	/// Storage: `AssetRegistry::Assets` (r:3 w:1)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `XYK::ShareToken` (r:1 w:1)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:7)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::AssetIds` (r:1 w:1)
	/// Proof: `AssetRegistry::AssetIds` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::NextAssetId` (r:1 w:1)
	/// Proof: `AssetRegistry::NextAssetId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:1)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:4 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:3 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencyPrice` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencyPrice` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:3 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `XYK::TotalLiquidity` (r:0 w:1)
	/// Proof: `XYK::TotalLiquidity` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `XYK::PoolAssets` (r:0 w:1)
	/// Proof: `XYK::PoolAssets` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3655`
		//  Estimated: `19071`
		// Minimum execution time: 478_829_000 picoseconds.
		Weight::from_parts(482_189_000, 19071)
			.saturating_add(RocksDbWeight::get().reads(33_u64))
			.saturating_add(RocksDbWeight::get().writes(20_u64))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:7)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `XYK::TotalLiquidity` (r:1 w:1)
	/// Proof: `XYK::TotalLiquidity` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:4 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:4 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencyPrice` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencyPrice` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::WhitelistedAssets` (r:1 w:0)
	/// Proof: `EmaOracle::WhitelistedAssets` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4291`
		//  Estimated: `19071`
		// Minimum execution time: 295_216_000 picoseconds.
		Weight::from_parts(297_013_000, 19071)
			.saturating_add(RocksDbWeight::get().reads(27_u64))
			.saturating_add(RocksDbWeight::get().writes(13_u64))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:1)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `XYK::TotalLiquidity` (r:1 w:1)
	/// Proof: `XYK::TotalLiquidity` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:7)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:1)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:3 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:4 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:2 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencyPrice` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencyPrice` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::WhitelistedAssets` (r:1 w:0)
	/// Proof: `EmaOracle::WhitelistedAssets` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	/// Storage: `XYK::PoolAssets` (r:0 w:1)
	/// Proof: `XYK::PoolAssets` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4341`
		//  Estimated: `19071`
		// Minimum execution time: 432_017_000 picoseconds.
		Weight::from_parts(435_171_000, 19071)
			.saturating_add(RocksDbWeight::get().reads(29_u64))
			.saturating_add(RocksDbWeight::get().writes(17_u64))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:6 w:6)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:3 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:3 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencyPrice` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencyPrice` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::WhitelistedAssets` (r:1 w:0)
	/// Proof: `EmaOracle::WhitelistedAssets` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	fn sell() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4190`
		//  Estimated: `16488`
		// Minimum execution time: 259_720_000 picoseconds.
		Weight::from_parts(262_022_000, 16488)
			.saturating_add(RocksDbWeight::get().reads(24_u64))
			.saturating_add(RocksDbWeight::get().writes(10_u64))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:6 w:6)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:3 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:3 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencyPrice` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencyPrice` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::WhitelistedAssets` (r:1 w:0)
	/// Proof: `EmaOracle::WhitelistedAssets` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	fn buy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4190`
		//  Estimated: `16488`
		// Minimum execution time: 258_233_000 picoseconds.
		Weight::from_parts(260_534_000, 16488)
			.saturating_add(RocksDbWeight::get().reads(24_u64))
			.saturating_add(RocksDbWeight::get().writes(10_u64))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:6 w:6)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:3 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:3 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencyPrice` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencyPrice` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::WhitelistedAssets` (r:1 w:0)
	/// Proof: `EmaOracle::WhitelistedAssets` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_sell(c: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1221 + e * (2969 ±0)`
		//  Estimated: `6156 + e * (10332 ±118_379_753_737_997_488)`
		// Minimum execution time: 24_323_000 picoseconds.
		Weight::from_parts(10_607_591, 6156)
			// Standard Error: 167_239
			.saturating_add(Weight::from_parts(7_205_082, 0).saturating_mul(c.into()))
			// Standard Error: 167_239
			.saturating_add(Weight::from_parts(243_290_509, 0).saturating_mul(e.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().reads((21_u64).saturating_mul(e.into())))
			.saturating_add(RocksDbWeight::get().writes((10_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 10332).saturating_mul(e.into()))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:6 w:6)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:3 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:3 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencyPrice` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencyPrice` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::WhitelistedAssets` (r:1 w:0)
	/// Proof: `EmaOracle::WhitelistedAssets` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 3]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_buy(c: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1221 + e * (2969 ±0)`
		//  Estimated: `6156 + e * (10332 ±0)`
		// Minimum execution time: 31_926_000 picoseconds.
		Weight::from_parts(8_222_348, 6156)
			// Standard Error: 100_411
			.saturating_add(Weight::from_parts(8_150_950, 0).saturating_mul(c.into()))
			// Standard Error: 170_358
			.saturating_add(Weight::from_parts(243_286_593, 0).saturating_mul(e.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().reads((21_u64).saturating_mul(e.into())))
			.saturating_add(RocksDbWeight::get().writes((10_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 10332).saturating_mul(e.into()))
	}
	/// Storage: `Tokens::Accounts` (r:2 w:0)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	fn calculate_spot_price_with_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1021`
		//  Estimated: `6156`
		// Minimum execution time: 19_174_000 picoseconds.
		Weight::from_parts(19_702_000, 6156)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
}
