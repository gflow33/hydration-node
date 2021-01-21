// This file is part of hack.HydraDX-node.

// Copyright (C) 2021 Intergalactic Ltd.
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

//! Autogenerated weights for exchange
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0
//! DATE: 2021-01-18, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/hack-hydra-dx
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=exchange
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=weights.rs
// --template=.maintain/pallet-weight-template.hbs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for exchange.
pub trait WeightInfo {
	fn known_overhead_for_on_finalize() -> Weight;
	fn sell_intention() -> Weight;
	fn buy_intention() -> Weight;
	fn on_finalize(t: u32, ) -> Weight;
	fn on_finalize_buys_no_matches(t: u32, ) -> Weight;
	fn on_finalize_sells_no_matches(t: u32, ) -> Weight;
	fn sell_extrinsic() -> Weight;
	fn on_finalize_for_one_sell_extrinsic() -> Weight;
	fn buy_extrinsic() -> Weight;
	fn on_finalize_for_one_buy_extrinsic() -> Weight;
}

/// Weights for exchange using the hack.hydraDX node and recommended hardware.
pub struct HackHydraWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for HackHydraWeight<T> {
	fn known_overhead_for_on_finalize() -> Weight {
		(13_726_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn sell_intention() -> Weight {
		(85_164_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn buy_intention() -> Weight {
		(85_048_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn on_finalize(t: u32, ) -> Weight {
		(21_475_000 as Weight)
			// Standard Error: 36_000
			.saturating_add((156_991_000 as Weight).saturating_mul(t as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(t as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(t as Weight)))
	}
	fn on_finalize_buys_no_matches(t: u32, ) -> Weight {
		(60_923_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((174_428_000 as Weight).saturating_mul(t as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(t as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(t as Weight)))
	}
	fn on_finalize_sells_no_matches(t: u32, ) -> Weight {
		(58_150_000 as Weight)
			// Standard Error: 20_000
			.saturating_add((151_583_000 as Weight).saturating_mul(t as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(t as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(t as Weight)))
	}
	fn sell_extrinsic() -> Weight {
		(165_292_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn on_finalize_for_one_sell_extrinsic() -> Weight {
		(217_154_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn buy_extrinsic() -> Weight {
		(164_250_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn on_finalize_for_one_buy_extrinsic() -> Weight {
		(239_520_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn known_overhead_for_on_finalize() -> Weight {
		(13_726_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn sell_intention() -> Weight {
		(85_164_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn buy_intention() -> Weight {
		(85_048_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn on_finalize(t: u32, ) -> Weight {
		(21_475_000 as Weight)
			// Standard Error: 36_000
			.saturating_add((156_991_000 as Weight).saturating_mul(t as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(t as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(t as Weight)))
	}
	fn on_finalize_buys_no_matches(t: u32, ) -> Weight {
		(60_923_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((174_428_000 as Weight).saturating_mul(t as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(t as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(t as Weight)))
	}
	fn on_finalize_sells_no_matches(t: u32, ) -> Weight {
		(58_150_000 as Weight)
			// Standard Error: 20_000
			.saturating_add((151_583_000 as Weight).saturating_mul(t as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(t as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(t as Weight)))
	}
	fn sell_extrinsic() -> Weight {
		(165_292_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn on_finalize_for_one_sell_extrinsic() -> Weight {
		(217_154_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn buy_extrinsic() -> Weight {
		(164_250_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn on_finalize_for_one_buy_extrinsic() -> Weight {
		(239_520_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
}
