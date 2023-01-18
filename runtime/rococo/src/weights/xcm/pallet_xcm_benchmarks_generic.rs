// Copyright 2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_xcm_benchmarks::generic`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// target/production/polkadot
// benchmark
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_xcm_benchmarks::generic
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --template=./xcm/pallet-xcm-benchmarks/template.hbs
// --output=./runtime/rococo/src/weights/xcm/pallet_xcm_benchmarks_generic.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_xcm_benchmarks::generic`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo<T> {
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	pub(crate) fn report_holding() -> Weight {
		Weight::from_ref_time(21_822_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	pub(crate) fn buy_execution() -> Weight {
		Weight::from_ref_time(3_109_000 as u64)
	}
	// Storage: XcmPallet Queries (r:1 w:0)
	pub(crate) fn query_response() -> Weight {
		Weight::from_ref_time(12_087_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	pub(crate) fn transact() -> Weight {
		Weight::from_ref_time(12_398_000 as u64)
	}
	pub(crate) fn refund_surplus() -> Weight {
		Weight::from_ref_time(3_247_000 as u64)
	}
	pub(crate) fn set_error_handler() -> Weight {
		Weight::from_ref_time(3_086_000 as u64)
	}
	pub(crate) fn set_appendix() -> Weight {
		Weight::from_ref_time(3_112_000 as u64)
	}
	pub(crate) fn clear_error() -> Weight {
		Weight::from_ref_time(3_118_000 as u64)
	}
	pub(crate) fn descend_origin() -> Weight {
		Weight::from_ref_time(4_054_000 as u64)
	}
	pub(crate) fn clear_origin() -> Weight {
		Weight::from_ref_time(3_111_000 as u64)
	}
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	pub(crate) fn report_error() -> Weight {
		Weight::from_ref_time(18_425_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: XcmPallet AssetTraps (r:1 w:1)
	pub(crate) fn claim_asset() -> Weight {
		Weight::from_ref_time(7_144_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	pub(crate) fn trap() -> Weight {
		Weight::from_ref_time(3_060_000 as u64)
	}
	// Storage: XcmPallet VersionNotifyTargets (r:1 w:1)
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	pub(crate) fn subscribe_version() -> Weight {
		Weight::from_ref_time(21_642_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: XcmPallet VersionNotifyTargets (r:0 w:1)
	pub(crate) fn unsubscribe_version() -> Weight {
		Weight::from_ref_time(4_873_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	pub(crate) fn initiate_reserve_withdraw() -> Weight {
		Weight::from_ref_time(22_809_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	pub(crate) fn burn_asset() -> Weight {
		Weight::from_ref_time(5_259_000 as u64)
	}
	pub(crate) fn expect_asset() -> Weight {
		Weight::from_ref_time(3_745_000 as u64)
	}
	pub(crate) fn expect_origin() -> Weight {
		Weight::from_ref_time(3_847_000 as u64)
	}
	pub(crate) fn expect_error() -> Weight {
		Weight::from_ref_time(3_633_000 as u64)
	}
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	pub(crate) fn query_pallet() -> Weight {
		Weight::from_ref_time(21_645_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	pub(crate) fn expect_pallet() -> Weight {
		Weight::from_ref_time(4_017_000 as u64)
	}
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	pub(crate) fn report_transact_status() -> Weight {
		Weight::from_ref_time(20_465_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	pub(crate) fn clear_transact_status() -> Weight {
		Weight::from_ref_time(3_723_000 as u64)
	}
	pub(crate) fn set_topic() -> Weight {
		Weight::from_ref_time(3_687_000 as u64)
	}
	pub(crate) fn clear_topic() -> Weight {
		Weight::from_ref_time(3_654_000 as u64)
	}
	pub(crate) fn set_fees_mode() -> Weight {
		Weight::from_ref_time(3_721_000 as u64)
	}
	pub(crate) fn unpaid_execution() -> Weight {
		Weight::from_ref_time(3_111_000 as u64)
	}
}
