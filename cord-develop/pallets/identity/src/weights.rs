// This file is part of CORD – https://cord.network

// Copyright (C) Dhiway Networks Pvt. Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// CORD is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// CORD is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with CORD. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-03-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `smohan-dev-host`, CPU: `AMD EPYC 7B12`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/production/cord
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/identity/src/weights.rs
// --header=./HEADER-GPL3
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_identity`.
pub trait WeightInfo {
	fn add_registrar(r: u32, ) -> Weight;
	fn remove_registrar(r: u32, ) -> Weight;
	fn set_identity(r: u32, ) -> Weight;
	fn set_subs_new(s: u32, ) -> Weight;
	fn set_subs_old(p: u32, ) -> Weight;
	fn clear_identity(r: u32, s: u32, ) -> Weight;
	fn request_judgement(r: u32, ) -> Weight;
	fn cancel_request(r: u32, ) -> Weight;
	fn set_account_id(r: u32, ) -> Weight;
	fn set_fields(r: u32, ) -> Weight;
	fn provide_judgement(r: u32, ) -> Weight;
	fn kill_identity(r: u32, s: u32, ) -> Weight;
	fn add_sub(s: u32, ) -> Weight;
	fn rename_sub(s: u32, ) -> Weight;
	fn remove_sub(s: u32, ) -> Weight;
	fn quit_sub(s: u32, ) -> Weight;
	fn add_username_authority() -> Weight;
	fn remove_username_authority() -> Weight;
	fn set_username_for() -> Weight;
	fn accept_username() -> Weight;
	fn remove_expired_approval() -> Weight;
	fn set_primary_username() -> Weight;
	fn remove_dangling_username() -> Weight;
}

/// Weights for `pallet_identity` using the CORD node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(821), added: 1316, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + r * (41 ±0)`
		//  Estimated: `2306`
		// Minimum execution time: 9_230_000 picoseconds.
		Weight::from_parts(9_722_479, 2306)
			// Standard Error: 1_084
			.saturating_add(Weight::from_parts(45_270, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(821), added: 1316, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn remove_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + r * (41 ±0)`
		//  Estimated: `2306`
		// Minimum execution time: 9_250_000 picoseconds.
		Weight::from_parts(9_768_440, 2306)
			// Standard Error: 1_172
			.saturating_add(Weight::from_parts(46_936, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn set_identity(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1594 + r * (33 ±0)`
		//  Estimated: `5731`
		// Minimum execution time: 29_340_000 picoseconds.
		Weight::from_parts(30_073_045, 5731)
			// Standard Error: 7_649
			.saturating_add(Weight::from_parts(155_115, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3242), added: 5717, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:100 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `6707 + s * (2589 ±0)`
		// Minimum execution time: 8_790_000 picoseconds.
		Weight::from_parts(10_400_138, 6707)
			// Standard Error: 3_098
			.saturating_add(Weight::from_parts(3_406_095, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
			.saturating_add(Weight::from_parts(0, 2589).saturating_mul(s.into()))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3242), added: 5717, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `178 + p * (32 ±0)`
		//  Estimated: `6707`
		// Minimum execution time: 8_900_000 picoseconds.
		Weight::from_parts(11_448_084, 6707)
			// Standard Error: 1_641
			.saturating_add(Weight::from_parts(1_395_402, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3242), added: 5717, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1670 + r * (33 ±0) + s * (32 ±0)`
		//  Estimated: `6707`
		// Minimum execution time: 20_240_000 picoseconds.
		Weight::from_parts(20_746_858, 6707)
			// Standard Error: 11_792
			.saturating_add(Weight::from_parts(149_924, 0).saturating_mul(r.into()))
			// Standard Error: 2_300
			.saturating_add(Weight::from_parts(1_407_069, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(821), added: 1316, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn request_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1584 + r * (41 ±0)`
		//  Estimated: `5731`
		// Minimum execution time: 25_529_000 picoseconds.
		Weight::from_parts(26_560_568, 5731)
			// Standard Error: 4_027
			.saturating_add(Weight::from_parts(53_879, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn cancel_request(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1627`
		//  Estimated: `5731`
		// Minimum execution time: 24_080_000 picoseconds.
		Weight::from_parts(24_797_940, 5731)
			// Standard Error: 1_535
			.saturating_add(Weight::from_parts(14_275, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(821), added: 1316, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `73 + r * (41 ±0)`
		//  Estimated: `2306`
		// Minimum execution time: 6_860_000 picoseconds.
		Weight::from_parts(7_131_088, 2306)
			// Standard Error: 843
			.saturating_add(Weight::from_parts(54_623, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(821), added: 1316, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `73 + r * (41 ±0)`
		//  Estimated: `2306`
		// Minimum execution time: 6_960_000 picoseconds.
		Weight::from_parts(7_199_778, 2306)
			// Standard Error: 817
			.saturating_add(Weight::from_parts(43_496, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(821), added: 1316, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn provide_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1658 + r * (41 ±0)`
		//  Estimated: `5731`
		// Minimum execution time: 33_160_000 picoseconds.
		Weight::from_parts(33_858_642, 5731)
			// Standard Error: 1_889
			.saturating_add(Weight::from_parts(76_393, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3242), added: 5717, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1670 + r * (33 ±0) + s * (32 ±0)`
		//  Estimated: `6707`
		// Minimum execution time: 20_320_000 picoseconds.
		Weight::from_parts(18_932_891, 6707)
			// Standard Error: 13_487
			.saturating_add(Weight::from_parts(208_451, 0).saturating_mul(r.into()))
			// Standard Error: 2_631
			.saturating_add(Weight::from_parts(1_420_224, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3242), added: 5717, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `459 + s * (36 ±0)`
		//  Estimated: `6707`
		// Minimum execution time: 14_690_000 picoseconds.
		Weight::from_parts(19_714_349, 6707)
			// Standard Error: 1_388
			.saturating_add(Weight::from_parts(67_819, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `591 + s * (3 ±0)`
		//  Estimated: `5731`
		// Minimum execution time: 13_250_000 picoseconds.
		Weight::from_parts(15_302_248, 5731)
			// Standard Error: 486
			.saturating_add(Weight::from_parts(24_972, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3242), added: 5717, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `622 + s * (35 ±0)`
		//  Estimated: `6707`
		// Minimum execution time: 19_850_000 picoseconds.
		Weight::from_parts(21_628_598, 6707)
			// Standard Error: 621
			.saturating_add(Weight::from_parts(58_410, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3242), added: 5717, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `548 + s * (37 ±0)`
		//  Estimated: `6707`
		// Minimum execution time: 15_760_000 picoseconds.
		Weight::from_parts(17_789_756, 6707)
			// Standard Error: 871
			.saturating_add(Weight::from_parts(57_171, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Identity::UsernameAuthorities` (r:0 w:1)
	/// Proof: `Identity::UsernameAuthorities` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn add_username_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_290_000 picoseconds.
		Weight::from_parts(6_560_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::UsernameAuthorities` (r:1 w:1)
	/// Proof: `Identity::UsernameAuthorities` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn remove_username_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `3517`
		// Minimum execution time: 9_390_000 picoseconds.
		Weight::from_parts(9_670_000, 3517)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::UsernameAuthorities` (r:1 w:1)
	/// Proof: `Identity::UsernameAuthorities` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AccountOfUsername` (r:1 w:1)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Identity::PendingUsernames` (r:1 w:0)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	fn set_username_for() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `5731`
		// Minimum execution time: 71_410_000 picoseconds.
		Weight::from_parts(72_760_000, 5731)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Identity::PendingUsernames` (r:1 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AccountOfUsername` (r:0 w:1)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	fn accept_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115`
		//  Estimated: `5731`
		// Minimum execution time: 20_279_000 picoseconds.
		Weight::from_parts(20_780_000, 5731)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Identity::PendingUsernames` (r:1 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn remove_expired_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115`
		//  Estimated: `3550`
		// Minimum execution time: 25_870_000 picoseconds.
		Weight::from_parts(30_871_000, 3550)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::AccountOfUsername` (r:1 w:0)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	fn set_primary_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `238`
		//  Estimated: `5731`
		// Minimum execution time: 16_600_000 picoseconds.
		Weight::from_parts(17_100_000, 5731)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::AccountOfUsername` (r:1 w:1)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	fn remove_dangling_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `98`
		//  Estimated: `5731`
		// Minimum execution time: 10_911_000 picoseconds.
		Weight::from_parts(11_500_000, 5731)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(821), added: 1316, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + r * (41 ±0)`
		//  Estimated: `2306`
		// Minimum execution time: 9_230_000 picoseconds.
		Weight::from_parts(9_722_479, 2306)
			// Standard Error: 1_084
			.saturating_add(Weight::from_parts(45_270, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(821), added: 1316, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn remove_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + r * (41 ±0)`
		//  Estimated: `2306`
		// Minimum execution time: 9_250_000 picoseconds.
		Weight::from_parts(9_768_440, 2306)
			// Standard Error: 1_172
			.saturating_add(Weight::from_parts(46_936, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn set_identity(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1594 + r * (33 ±0)`
		//  Estimated: `5731`
		// Minimum execution time: 29_340_000 picoseconds.
		Weight::from_parts(30_073_045, 5731)
			// Standard Error: 7_649
			.saturating_add(Weight::from_parts(155_115, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3242), added: 5717, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:100 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `6707 + s * (2589 ±0)`
		// Minimum execution time: 8_790_000 picoseconds.
		Weight::from_parts(10_400_138, 6707)
			// Standard Error: 3_098
			.saturating_add(Weight::from_parts(3_406_095, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(s.into())))
			.saturating_add(Weight::from_parts(0, 2589).saturating_mul(s.into()))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3242), added: 5717, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `178 + p * (32 ±0)`
		//  Estimated: `6707`
		// Minimum execution time: 8_900_000 picoseconds.
		Weight::from_parts(11_448_084, 6707)
			// Standard Error: 1_641
			.saturating_add(Weight::from_parts(1_395_402, 0).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3242), added: 5717, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1670 + r * (33 ±0) + s * (32 ±0)`
		//  Estimated: `6707`
		// Minimum execution time: 20_240_000 picoseconds.
		Weight::from_parts(20_746_858, 6707)
			// Standard Error: 11_792
			.saturating_add(Weight::from_parts(149_924, 0).saturating_mul(r.into()))
			// Standard Error: 2_300
			.saturating_add(Weight::from_parts(1_407_069, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(821), added: 1316, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn request_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1584 + r * (41 ±0)`
		//  Estimated: `5731`
		// Minimum execution time: 25_529_000 picoseconds.
		Weight::from_parts(26_560_568, 5731)
			// Standard Error: 4_027
			.saturating_add(Weight::from_parts(53_879, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn cancel_request(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1627`
		//  Estimated: `5731`
		// Minimum execution time: 24_080_000 picoseconds.
		Weight::from_parts(24_797_940, 5731)
			// Standard Error: 1_535
			.saturating_add(Weight::from_parts(14_275, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(821), added: 1316, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `73 + r * (41 ±0)`
		//  Estimated: `2306`
		// Minimum execution time: 6_860_000 picoseconds.
		Weight::from_parts(7_131_088, 2306)
			// Standard Error: 843
			.saturating_add(Weight::from_parts(54_623, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(821), added: 1316, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `73 + r * (41 ±0)`
		//  Estimated: `2306`
		// Minimum execution time: 6_960_000 picoseconds.
		Weight::from_parts(7_199_778, 2306)
			// Standard Error: 817
			.saturating_add(Weight::from_parts(43_496, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(821), added: 1316, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn provide_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1658 + r * (41 ±0)`
		//  Estimated: `5731`
		// Minimum execution time: 33_160_000 picoseconds.
		Weight::from_parts(33_858_642, 5731)
			// Standard Error: 1_889
			.saturating_add(Weight::from_parts(76_393, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3242), added: 5717, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1670 + r * (33 ±0) + s * (32 ±0)`
		//  Estimated: `6707`
		// Minimum execution time: 20_320_000 picoseconds.
		Weight::from_parts(18_932_891, 6707)
			// Standard Error: 13_487
			.saturating_add(Weight::from_parts(208_451, 0).saturating_mul(r.into()))
			// Standard Error: 2_631
			.saturating_add(Weight::from_parts(1_420_224, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3242), added: 5717, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `459 + s * (36 ±0)`
		//  Estimated: `6707`
		// Minimum execution time: 14_690_000 picoseconds.
		Weight::from_parts(19_714_349, 6707)
			// Standard Error: 1_388
			.saturating_add(Weight::from_parts(67_819, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `591 + s * (3 ±0)`
		//  Estimated: `5731`
		// Minimum execution time: 13_250_000 picoseconds.
		Weight::from_parts(15_302_248, 5731)
			// Standard Error: 486
			.saturating_add(Weight::from_parts(24_972, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3242), added: 5717, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `622 + s * (35 ±0)`
		//  Estimated: `6707`
		// Minimum execution time: 19_850_000 picoseconds.
		Weight::from_parts(21_628_598, 6707)
			// Standard Error: 621
			.saturating_add(Weight::from_parts(58_410, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3242), added: 5717, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `548 + s * (37 ±0)`
		//  Estimated: `6707`
		// Minimum execution time: 15_760_000 picoseconds.
		Weight::from_parts(17_789_756, 6707)
			// Standard Error: 871
			.saturating_add(Weight::from_parts(57_171, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Identity::UsernameAuthorities` (r:0 w:1)
	/// Proof: `Identity::UsernameAuthorities` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn add_username_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_290_000 picoseconds.
		Weight::from_parts(6_560_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::UsernameAuthorities` (r:1 w:1)
	/// Proof: `Identity::UsernameAuthorities` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn remove_username_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `3517`
		// Minimum execution time: 9_390_000 picoseconds.
		Weight::from_parts(9_670_000, 3517)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::UsernameAuthorities` (r:1 w:1)
	/// Proof: `Identity::UsernameAuthorities` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AccountOfUsername` (r:1 w:1)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Identity::PendingUsernames` (r:1 w:0)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	fn set_username_for() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `5731`
		// Minimum execution time: 71_410_000 picoseconds.
		Weight::from_parts(72_760_000, 5731)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Identity::PendingUsernames` (r:1 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AccountOfUsername` (r:0 w:1)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	fn accept_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115`
		//  Estimated: `5731`
		// Minimum execution time: 20_279_000 picoseconds.
		Weight::from_parts(20_780_000, 5731)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Identity::PendingUsernames` (r:1 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn remove_expired_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115`
		//  Estimated: `3550`
		// Minimum execution time: 25_870_000 picoseconds.
		Weight::from_parts(30_871_000, 3550)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::AccountOfUsername` (r:1 w:0)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	fn set_primary_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `238`
		//  Estimated: `5731`
		// Minimum execution time: 16_600_000 picoseconds.
		Weight::from_parts(17_100_000, 5731)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::AccountOfUsername` (r:1 w:1)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(2266), added: 4741, mode: `MaxEncodedLen`)
	fn remove_dangling_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `98`
		//  Estimated: `5731`
		// Minimum execution time: 10_911_000 picoseconds.
		Weight::from_parts(11_500_000, 5731)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}