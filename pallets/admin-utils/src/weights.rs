
//! Autogenerated weights for `pallet_admin_utils`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Samuels-MacBook-Pro`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("local")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/node-subtensor
// benchmark
// pallet
// --chain=local
// --wasm-execution=compiled
// --pallet=pallet_admin_utils
// --extrinsic=*
// --steps
// 50
// --repeat
// 20
// --output=pallets/admin-utils/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_admin_utils`.
pub trait WeightInfo {
	fn swap_authorities(a: u32, ) -> Weight;
	fn sudo_set_default_take() -> Weight;
	fn sudo_set_serving_rate_limit() -> Weight;
	fn sudo_set_max_difficulty() -> Weight;
	fn sudo_set_min_difficulty() -> Weight;
	fn sudo_set_weights_set_rate_limit() -> Weight;
	fn sudo_set_weights_version_key() -> Weight;
	fn sudo_set_bonds_moving_average() -> Weight;
	fn sudo_set_max_allowed_validators() -> Weight;
	fn sudo_set_difficulty() -> Weight;
	fn sudo_set_adjustment_interval() -> Weight;
	fn sudo_set_target_registrations_per_interval() -> Weight;
	fn sudo_set_activity_cutoff() -> Weight;
	fn sudo_set_rho() -> Weight;
	fn sudo_set_kappa() -> Weight;
	fn sudo_set_max_allowed_uids() -> Weight;
	fn sudo_set_min_allowed_weights() -> Weight;
	fn sudo_set_immunity_period() -> Weight;
	fn sudo_set_max_weight_limit() -> Weight;
	fn sudo_set_max_registrations_per_block() -> Weight;
	fn sudo_set_max_burn() -> Weight;
	fn sudo_set_min_burn() -> Weight;
	fn sudo_set_network_registration_allowed() -> Weight;
	fn sudo_set_tempo() -> Weight;
	fn sudo_set_commit_reveal_weights_interval() -> Weight;
	fn sudo_set_commit_reveal_weights_enabled() -> Weight;
	fn sudo_set_hotkey_emission_tempo() -> Weight;
	fn sudo_set_network_max_stake() -> Weight;
}

/// Weights for `pallet_admin_utils` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `System::Digest` (r:1 w:1)
	/// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Aura::Authorities` (r:0 w:1)
	/// Proof: `Aura::Authorities` (`max_values`: Some(1), `max_size`: Some(1025), added: 1520, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 32]`.
	fn swap_authorities(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `1485`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(5_590_882, 1485)
			// Standard Error: 2_759
			.saturating_add(Weight::from_parts(106_257, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `SubtensorModule::MaxTake` (r:0 w:1)
	/// Proof: `SubtensorModule::MaxTake` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn sudo_set_default_take() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_000_000 picoseconds.
		Weight::from_parts(5_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::ServingRateLimit` (r:0 w:1)
	/// Proof: `SubtensorModule::ServingRateLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_serving_rate_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MaxDifficulty` (r:0 w:1)
	/// Proof: `SubtensorModule::MaxDifficulty` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_max_difficulty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MinDifficulty` (r:0 w:1)
	/// Proof: `SubtensorModule::MinDifficulty` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_min_difficulty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::WeightsSetRateLimit` (r:0 w:1)
	/// Proof: `SubtensorModule::WeightsSetRateLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_weights_set_rate_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::WeightsVersionKey` (r:0 w:1)
	/// Proof: `SubtensorModule::WeightsVersionKey` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_weights_version_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(14_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::BondsMovingAverage` (r:0 w:1)
	/// Proof: `SubtensorModule::BondsMovingAverage` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_bonds_moving_average() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MaxAllowedUids` (r:1 w:0)
	/// Proof: `SubtensorModule::MaxAllowedUids` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MaxAllowedValidators` (r:0 w:1)
	/// Proof: `SubtensorModule::MaxAllowedValidators` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_max_allowed_validators() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `499`
		//  Estimated: `3964`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(17_000_000, 3964)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::Difficulty` (r:0 w:1)
	/// Proof: `SubtensorModule::Difficulty` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_difficulty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(14_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::AdjustmentInterval` (r:0 w:1)
	/// Proof: `SubtensorModule::AdjustmentInterval` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_adjustment_interval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::TargetRegistrationsPerInterval` (r:0 w:1)
	/// Proof: `SubtensorModule::TargetRegistrationsPerInterval` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_target_registrations_per_interval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::ActivityCutoff` (r:0 w:1)
	/// Proof: `SubtensorModule::ActivityCutoff` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_activity_cutoff() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::Rho` (r:0 w:1)
	/// Proof: `SubtensorModule::Rho` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_rho() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(11_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::Kappa` (r:0 w:1)
	/// Proof: `SubtensorModule::Kappa` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_kappa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::SubnetworkN` (r:1 w:0)
	/// Proof: `SubtensorModule::SubnetworkN` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MaxAllowedUids` (r:0 w:1)
	/// Proof: `SubtensorModule::MaxAllowedUids` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_max_allowed_uids() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `462`
		//  Estimated: `3927`
		// Minimum execution time: 15_000_000 picoseconds.
		Weight::from_parts(16_000_000, 3927)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MinAllowedWeights` (r:0 w:1)
	/// Proof: `SubtensorModule::MinAllowedWeights` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_min_allowed_weights() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::ImmunityPeriod` (r:0 w:1)
	/// Proof: `SubtensorModule::ImmunityPeriod` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_immunity_period() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(14_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MaxWeightsLimit` (r:0 w:1)
	/// Proof: `SubtensorModule::MaxWeightsLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_max_weight_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MaxRegistrationsPerBlock` (r:0 w:1)
	/// Proof: `SubtensorModule::MaxRegistrationsPerBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_max_registrations_per_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MaxBurn` (r:0 w:1)
	/// Proof: `SubtensorModule::MaxBurn` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_max_burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MinBurn` (r:0 w:1)
	/// Proof: `SubtensorModule::MinBurn` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_min_burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworkRegistrationAllowed` (r:0 w:1)
	/// Proof: `SubtensorModule::NetworkRegistrationAllowed` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_network_registration_allowed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(7_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::Tempo` (r:0 w:1)
	/// Proof: `SubtensorModule::Tempo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_tempo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::WeightCommitRevealInterval` (r:0 w:1)
	/// Proof: `SubtensorModule::WeightCommitRevealInterval` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_commit_reveal_weights_interval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(11_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::CommitRevealWeightsEnabled` (r:0 w:1)
	/// Proof: `SubtensorModule::CommitRevealWeightsEnabled` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_commit_reveal_weights_enabled() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(12_000_000, 3921)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::HotkeyEmissionTempo` (r:0 w:1)
	/// Proof: `SubtensorModule::HotkeyEmissionTempo` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn sudo_set_hotkey_emission_tempo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_000_000 picoseconds.
		Weight::from_parts(6_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworkMaxStake` (r:0 w:1)
	/// Proof: `SubtensorModule::NetworkMaxStake` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_network_max_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(7_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `System::Digest` (r:1 w:1)
	/// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Aura::Authorities` (r:0 w:1)
	/// Proof: `Aura::Authorities` (`max_values`: Some(1), `max_size`: Some(1025), added: 1520, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 32]`.
	fn swap_authorities(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `1485`
		// Minimum execution time: 4_000_000 picoseconds.
		Weight::from_parts(5_590_882, 1485)
			// Standard Error: 2_759
			.saturating_add(Weight::from_parts(106_257, 0).saturating_mul(a.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `SubtensorModule::MaxTake` (r:0 w:1)
	/// Proof: `SubtensorModule::MaxTake` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn sudo_set_default_take() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_000_000 picoseconds.
		Weight::from_parts(5_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::ServingRateLimit` (r:0 w:1)
	/// Proof: `SubtensorModule::ServingRateLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_serving_rate_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MaxDifficulty` (r:0 w:1)
	/// Proof: `SubtensorModule::MaxDifficulty` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_max_difficulty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MinDifficulty` (r:0 w:1)
	/// Proof: `SubtensorModule::MinDifficulty` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_min_difficulty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::WeightsSetRateLimit` (r:0 w:1)
	/// Proof: `SubtensorModule::WeightsSetRateLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_weights_set_rate_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::WeightsVersionKey` (r:0 w:1)
	/// Proof: `SubtensorModule::WeightsVersionKey` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_weights_version_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(14_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::BondsMovingAverage` (r:0 w:1)
	/// Proof: `SubtensorModule::BondsMovingAverage` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_bonds_moving_average() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MaxAllowedUids` (r:1 w:0)
	/// Proof: `SubtensorModule::MaxAllowedUids` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MaxAllowedValidators` (r:0 w:1)
	/// Proof: `SubtensorModule::MaxAllowedValidators` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_max_allowed_validators() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `499`
		//  Estimated: `3964`
		// Minimum execution time: 17_000_000 picoseconds.
		Weight::from_parts(17_000_000, 3964)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::Difficulty` (r:0 w:1)
	/// Proof: `SubtensorModule::Difficulty` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_difficulty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(14_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::AdjustmentInterval` (r:0 w:1)
	/// Proof: `SubtensorModule::AdjustmentInterval` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_adjustment_interval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::TargetRegistrationsPerInterval` (r:0 w:1)
	/// Proof: `SubtensorModule::TargetRegistrationsPerInterval` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_target_registrations_per_interval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::ActivityCutoff` (r:0 w:1)
	/// Proof: `SubtensorModule::ActivityCutoff` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_activity_cutoff() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::Rho` (r:0 w:1)
	/// Proof: `SubtensorModule::Rho` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_rho() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(11_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::Kappa` (r:0 w:1)
	/// Proof: `SubtensorModule::Kappa` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_kappa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::SubnetworkN` (r:1 w:0)
	/// Proof: `SubtensorModule::SubnetworkN` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MaxAllowedUids` (r:0 w:1)
	/// Proof: `SubtensorModule::MaxAllowedUids` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_max_allowed_uids() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `462`
		//  Estimated: `3927`
		// Minimum execution time: 15_000_000 picoseconds.
		Weight::from_parts(16_000_000, 3927)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MinAllowedWeights` (r:0 w:1)
	/// Proof: `SubtensorModule::MinAllowedWeights` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_min_allowed_weights() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::ImmunityPeriod` (r:0 w:1)
	/// Proof: `SubtensorModule::ImmunityPeriod` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_immunity_period() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(14_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MaxWeightsLimit` (r:0 w:1)
	/// Proof: `SubtensorModule::MaxWeightsLimit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_max_weight_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MaxRegistrationsPerBlock` (r:0 w:1)
	/// Proof: `SubtensorModule::MaxRegistrationsPerBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_max_registrations_per_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MaxBurn` (r:0 w:1)
	/// Proof: `SubtensorModule::MaxBurn` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_max_burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::MinBurn` (r:0 w:1)
	/// Proof: `SubtensorModule::MinBurn` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_min_burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworkRegistrationAllowed` (r:0 w:1)
	/// Proof: `SubtensorModule::NetworkRegistrationAllowed` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_network_registration_allowed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(7_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::Tempo` (r:0 w:1)
	/// Proof: `SubtensorModule::Tempo` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_tempo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::WeightCommitRevealInterval` (r:0 w:1)
	/// Proof: `SubtensorModule::WeightCommitRevealInterval` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_commit_reveal_weights_interval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(11_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworksAdded` (r:1 w:0)
	/// Proof: `SubtensorModule::NetworksAdded` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SubtensorModule::CommitRevealWeightsEnabled` (r:0 w:1)
	/// Proof: `SubtensorModule::CommitRevealWeightsEnabled` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_commit_reveal_weights_enabled() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(12_000_000, 3921)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::HotkeyEmissionTempo` (r:0 w:1)
	/// Proof: `SubtensorModule::HotkeyEmissionTempo` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn sudo_set_hotkey_emission_tempo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_000_000 picoseconds.
		Weight::from_parts(6_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `SubtensorModule::NetworkMaxStake` (r:0 w:1)
	/// Proof: `SubtensorModule::NetworkMaxStake` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sudo_set_network_max_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(7_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}