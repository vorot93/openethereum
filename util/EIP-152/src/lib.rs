// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

#![warn(
	clippy::all,
	clippy::pedantic,
	clippy::nursery,
)]
#![allow(
	clippy::blacklisted_name,
	clippy::cast_lossless,
	clippy::cast_possible_truncation,
	clippy::cast_possible_wrap,
	clippy::cast_precision_loss,
	clippy::cast_ptr_alignment,
	clippy::cast_sign_loss,
	clippy::cognitive_complexity,
	clippy::default_trait_access,
	clippy::enum_glob_use,
	clippy::eval_order_dependence,
	clippy::fallible_impl_from,
	clippy::float_cmp,
	clippy::identity_op,
	clippy::if_not_else,
	clippy::indexing_slicing,
	clippy::inline_always,
	clippy::items_after_statements,
	clippy::large_enum_variant,
	clippy::many_single_char_names,
	clippy::match_same_arms,
	clippy::missing_errors_doc,
	clippy::missing_safety_doc,
	clippy::module_inception,
	clippy::module_name_repetitions,
	clippy::must_use_candidate,
	clippy::needless_pass_by_value,
	clippy::needless_update,
	clippy::non_ascii_literal,
	clippy::option_option,
	clippy::pub_enum_variant_names,
	clippy::same_functions_in_if_condition,
	clippy::shadow_unrelated,
	clippy::similar_names,
	clippy::single_component_path_imports,
	clippy::too_many_arguments,
	clippy::too_many_lines,
	clippy::type_complexity,
	clippy::unused_self,
	clippy::used_underscore_binding,
)]

pub mod portable;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod avx2;

/// The precomputed values for `BLAKE2b` [from the spec](https://tools.ietf.org/html/rfc7693#section-2.7)
/// There are 10 16-byte arrays - one for each round
/// the entries are calculated from the sigma constants.
const SIGMA: [[usize; 16]; 10] = [
	[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
	[14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
	[11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
	[7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
	[9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
	[2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
	[12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
	[13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
	[6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
	[10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
];


/// IV is the initialization vector for `BLAKE2b`. See <https://tools.ietf.org/html/rfc7693#section-2.6>
/// for details.
const IV: [u64; 8] = [
	0x6a09_e667_f3bc_c908, 0xbb67_ae85_84ca_a73b, 0x3c6e_f372_fe94_f82b, 0xa54f_f53a_5f1d_36f1,
	0x510e_527f_ade6_82d1, 0x9b05_688c_2b3e_6c1f, 0x1f83_d9ab_fb41_bd6b, 0x5be0_cd19_137e_2179,
];

/// blake2b compression function
pub fn compress(state: &mut [u64; 8], message: [u64; 16], count: [u64; 2], f: bool, rounds: usize) {
	#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
	{
		if is_x86_feature_detected!("avx2") {
			unsafe {
				avx2::compress(state, message, count, f, rounds)
			}
		} else {
			portable::compress(state, message, count, f, rounds)
		};
	}

	#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
	portable::compress(state, message, count, f, rounds);
}


#[cfg(test)]
mod tests {
	use crate::portable;

	#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
	use crate::avx2;
	use rustc_hex::FromHex;

	#[test]
	fn test_blake2_f() {
		// test from https://github.com/ethereum/EIPs/blob/master/EIPS/eip-152.md#example-usage-in-solidity
		let mut h_in = [
			0x6a09_e667_f2bd_c948_u64, 0xbb67_ae85_84ca_a73b_u64,
			0x3c6e_f372_fe94_f82b_u64, 0xa54f_f53a_5f1d_36f1_u64,
			0x510e_527f_ade6_82d1_u64, 0x9b05_688c_2b3e_6c1f_u64,
			0x1f83_d9ab_fb41_bd6b_u64, 0x5be0_cd19_137e_2179_u64,
		];

		let m = [
			0x0000_0000_0063_6261_u64, 0x0000_0000_0000_0000_u64, 0x0000_0000_0000_0000_u64,
			0x0000_0000_0000_0000_u64, 0x0000_0000_0000_0000_u64, 0x0000_0000_0000_0000_u64,
			0x0000_0000_0000_0000_u64, 0x0000_0000_0000_0000_u64, 0x0000_0000_0000_0000_u64,
			0x0000_0000_0000_0000_u64, 0x0000_0000_0000_0000_u64, 0x0000_0000_0000_0000_u64,
			0x0000_0000_0000_0000_u64, 0x0000_0000_0000_0000_u64, 0x0000_0000_0000_0000_u64,
			0x0000_0000_0000_0000_u64,
		];
		let c = [3, 0];
		let f = true;
		let rounds = 12;
		let h_out: [u64; 8] = [
			0x0D4D_1C98_3FA5_80BA_u64, 0xE9F6_129F_B697_276A_u64, 0xB7C4_5A68_142F_214C_u64,
			0xD1A2_FFDB_6FBB_124B_u64, 0x2D79_AB2A_39C5_877D_u64, 0x95CC_3345_DED5_52C2_u64,
			0x5A92_F1DB_A88A_D318_u64, 0x2399_00D4_ED86_23B9_u64,
		];

		// portable
		portable::compress(&mut h_in, m, c, f, rounds);
		assert_eq!(h_in, h_out);

		let mut h_in = [
			0x6a09_e667_f2bd_c948_u64, 0xbb67_ae85_84ca_a73b_u64,
			0x3c6e_f372_fe94_f82b_u64, 0xa54f_f53a_5f1d_36f1_u64,
			0x510e_527f_ade6_82d1_u64, 0x9b05_688c_2b3e_6c1f_u64,
			0x1f83_d9ab_fb41_bd6b_u64, 0x5be0_cd19_137e_2179_u64,
		];

		// avx
		#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
		{
			if is_x86_feature_detected!("avx2") {
				unsafe {
					avx2::compress(&mut h_in, m, c, f, rounds);
					assert_eq!(h_in, h_out);
				}
			}
		}
	}

	fn to_u64_slice(vec: &[u8], slice: &mut [u64]) {
		vec.chunks(8).enumerate().for_each(|(index, val)| {
			slice[index] = u64::from_le_bytes([val[0], val[1], val[2], val[3], val[4], val[5], val[6], val[7]])
		})
	}


	#[test]
	fn test_vectors_from_eip() {
		let vec = vec![
			(
				// Test vector 4
				"0000000048c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
				"08c9bcf367e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d282e6ad7f520e511f6c3e2b8c68059b9442be0454267ce079217e1319cde05b",
			),
			(   // test vector 5
				"0000000c48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
				"ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923",
			),
			(
				// Test vector 6
				"0000000c48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000",
				"75ab69d3190a562c51aef8d88f1c2775876944407270c42c9844252c26d2875298743e7f6d5ea2f2d3e8d226039cd31b4e426ac4f2d3d666a610c2116fde4735",
			),
			(
				// Test vector 7
				"0000000148c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
				"b63a380cb2897d521994a85234ee2c181b5f844d2c624c002677e9703449d2fba551b3a8333bcdf5f2f7e08993d53923de3d64fcc68c034e717b9293fed7a421",
			),
			// Test vector 8 – u32::MAX rounds – too slow to run
//			(
//				"ffffffff48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b61626300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000001",
//				"fc59093aafa9ab43daae0e914c57635c5402d8e3d2130eb9b3cc181de7f0ecf9b22bf99a7815ce16419e200e01846e6b5df8cc7703041bbceb571de6631d2615",
//			),
		];
		for (hex, output) in vec {
			let hex = hex;
			let bytes: Vec<u8> = hex.from_hex().unwrap();

			assert_eq!(bytes.len(), 213);

			let mut h = [0_u64; 8];
			let mut m = [0_u64; 16];
			let mut t = [0_u64; 2];

			let rounds = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
			let f = match bytes[212] {
				1 => true,
				0 => false,
				_ => unreachable!()
			};

			to_u64_slice(&bytes[4..68], &mut h);
			to_u64_slice(&bytes[68..196], &mut m);
			to_u64_slice(&bytes[196..212], &mut t);
			let output: Vec<u8> = output.from_hex().unwrap();
			let mut out = [0_u64; 8];
			to_u64_slice(&output[..], &mut out);

			#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
			{
				// avx
				if is_x86_feature_detected!("avx2") {
					unsafe {
						avx2::compress(&mut h, m, t, f, rounds as usize);
						assert_eq!(out, h);
					}
				}
			}

			{
				// portable
				to_u64_slice(&bytes[4..68], &mut h);
				portable::compress(&mut h, m, t, f, rounds as usize);
				assert_eq!(out, h);
			}
		}
	}
}
