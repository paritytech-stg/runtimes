// Copyright (C) Parity Technologies and the various Polkadot contributors, see Contributions.md
// for a list of specific contributors.
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

//! Autogenerated weights for `pallet_bridge_relayers`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-07-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `arc-runners-beefy-stg-qbztc-runner-n2qct`, CPU: `AMD EPYC 7B13`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// frame-omni-bencher
// v1
// benchmark
// pallet
// --extrinsic=*
// --runtime=target/release/wbuild/bridge-hub-polkadot-runtime/bridge_hub_polkadot_runtime.wasm
// --pallet=pallet_bridge_relayers
// --header=/home/runner/_work/runtimes/runtimes/.github/scripts/cmd/file_header.txt
// --output=./system-parachains/bridge-hubs/bridge-hub-polkadot/src/weights/pallet_bridge_relayers.rs
// --wasm-execution=compiled
// --steps=50
// --repeat=20
// --heap-pages=4096

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bridge_relayers`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_relayers::WeightInfo for WeightInfo<T> {
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn claim_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `130`
		//  Estimated: `3593`
		// Minimum execution time: 66_020_000 picoseconds.
		Weight::from_parts(67_900_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BridgeRelayers::RegisteredRelayers` (r:1 w:1)
	/// Proof: `BridgeRelayers::RegisteredRelayers` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x1e8445dc201eeb8560e5579a5dd54655` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x1e8445dc201eeb8560e5579a5dd54655` (r:1 w:0)
	/// Storage: `Balances::Reserves` (r:1 w:1)
	/// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(1249), added: 3724, mode: `MaxEncodedLen`)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `4714`
		// Minimum execution time: 32_940_000 picoseconds.
		Weight::from_parts(34_990_000, 0)
			.saturating_add(Weight::from_parts(0, 4714))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BridgeRelayers::RegisteredRelayers` (r:1 w:1)
	/// Proof: `BridgeRelayers::RegisteredRelayers` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Reserves` (r:1 w:1)
	/// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(1249), added: 3724, mode: `MaxEncodedLen`)
	fn deregister() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `135`
		//  Estimated: `4714`
		// Minimum execution time: 34_960_000 picoseconds.
		Weight::from_parts(36_330_000, 0)
			.saturating_add(Weight::from_parts(0, 4714))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BridgeRelayers::RegisteredRelayers` (r:1 w:1)
	/// Proof: `BridgeRelayers::RegisteredRelayers` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Reserves` (r:1 w:1)
	/// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(1249), added: 3724, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn slash_and_deregister() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `238`
		//  Estimated: `4714`
		// Minimum execution time: 38_620_000 picoseconds.
		Weight::from_parts(40_160_000, 0)
			.saturating_add(Weight::from_parts(0, 4714))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn register_relayer_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3538`
		// Minimum execution time: 6_390_000 picoseconds.
		Weight::from_parts(6_810_000, 0)
			.saturating_add(Weight::from_parts(0, 3538))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
