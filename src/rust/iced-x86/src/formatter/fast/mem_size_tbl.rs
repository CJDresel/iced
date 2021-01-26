// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

use super::super::super::iced_constants::IcedConstants;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

// GENERATOR-BEGIN: MemorySizes
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[rustfmt::skip]
static MEM_SIZE_TBL_DATA: [u8; 141] = [
	0x00,
	0x01,
	0x0D,
	0x03,
	0x0B,
	0x0B,
	0x0E,
	0x0F,
	0x10,
	0x01,
	0x0D,
	0x03,
	0x0B,
	0x0E,
	0x0F,
	0x10,
	0x03,
	0x08,
	0x0C,
	0x0D,
	0x03,
	0x0B,
	0x03,
	0x0B,
	0x0B,
	0x09,
	0x08,
	0x08,
	0x0D,
	0x03,
	0x0B,
	0x0C,
	0x0E,
	0x0D,
	0x04,
	0x05,
	0x07,
	0x06,
	0x00,
	0x00,
	0x00,
	0x00,
	0x0C,
	0x10,
	0x00,
	0x0C,
	0x11,
	0x10,
	0x0D,
	0x0D,
	0x03,
	0x03,
	0x03,
	0x03,
	0x03,
	0x0B,
	0x0B,
	0x0B,
	0x0B,
	0x0B,
	0x0B,
	0x0B,
	0x0B,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0E,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x0F,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x10,
	0x02,
	0x02,
	0x02,
	0x02,
	0x02,
	0x0A,
	0x0A,
	0x0A,
	0x02,
	0x0A,
	0x02,
	0x02,
	0x0A,
	0x0A,
	0x0A,
	0x02,
	0x0A,
	0x02,
	0x02,
	0x0A,
	0x0A,
	0x0A,
	0x02,
	0x0A,
	0x02,
	0x02,
	0x02,
	0x0A,
	0x0A,
	0x0A,
	0x0A,
	0x0A,
	0x0A,
	0x02,
	0x02,
	0x02,
];
// GENERATOR-END: MemorySizes

lazy_static! {
	pub(super) static ref MEM_SIZE_TBL: Vec<&'static str> = {
		let mut v = Vec::with_capacity(IcedConstants::MEMORY_SIZE_ENUM_COUNT);
		for &mem_keywords in MEM_SIZE_TBL_DATA.iter() {
			let keywords: &'static str = match mem_keywords {
				// GENERATOR-BEGIN: Match
				// ⚠️This was generated by GENERATOR!🦹‍♂️
				0 => "",
				1 => "byte ptr ",
				2 => "dword bcst ",
				3 => "dword ptr ",
				4 => "fpuenv14 ptr ",
				5 => "fpuenv28 ptr ",
				6 => "fpustate108 ptr ",
				7 => "fpustate94 ptr ",
				8 => "fword ptr ",
				9 => "oword ptr ",
				10 => "qword bcst ",
				11 => "qword ptr ",
				12 => "tbyte ptr ",
				13 => "word ptr ",
				14 => "xmmword ptr ",
				15 => "ymmword ptr ",
				16 => "zmmword ptr ",
				17 => "mem384 ptr ",
				// GENERATOR-END: Match
				_ => unreachable!(),
			};
			v.push(keywords);
		}
		v
	};
}
