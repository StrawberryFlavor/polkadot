// Copyright 2017-2022 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `runtime_parachains::disputes::slashing`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-06, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// /home/benchbot/cargo_target_dir/production/polkadot
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --pallet=runtime_parachains::disputes::slashing
// --chain=kusama-dev
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/runtime_parachains_disputes_slashing.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::disputes::slashing`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::disputes::slashing::WeightInfo for WeightInfo<T> {
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: Historical HistoricalSessions (r:1 w:0)
	// Storage: ParaSessionInfo Sessions (r:1 w:0)
	// Storage: ParasSlashing UnappliedSlashes (r:1 w:1)
	// Storage: Offences ReportsByKindIndex (r:1 w:1)
	// Storage: Offences ConcurrentReportsIndex (r:1 w:1)
	// Storage: Offences Reports (r:1 w:1)
	// Storage: Staking SlashRewardFraction (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	// Storage: Staking Invulnerables (r:1 w:0)
	// Storage: Staking ValidatorSlashInEra (r:1 w:0)
	/// The range of component `n` is `[4, 1000]`.
	fn report_dispute_lost(n: u32, ) -> Weight {
		Weight::from_ref_time(115_056_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(128_000 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
}
