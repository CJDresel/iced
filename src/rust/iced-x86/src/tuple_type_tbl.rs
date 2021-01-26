// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

use super::TupleType;

// GENERATOR-BEGIN: TupleTypeTable
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[rustfmt::skip]
static TUPLE_TYPE_TBL: [u8; 28] = [
	// TupleType.N1
	0x01,// N
	0x01,// Nbcst
	// TupleType.N2
	0x02,// N
	0x02,// Nbcst
	// TupleType.N4
	0x04,// N
	0x04,// Nbcst
	// TupleType.N8
	0x08,// N
	0x08,// Nbcst
	// TupleType.N16
	0x10,// N
	0x10,// Nbcst
	// TupleType.N32
	0x20,// N
	0x20,// Nbcst
	// TupleType.N64
	0x40,// N
	0x40,// Nbcst
	// TupleType.N8b4
	0x08,// N
	0x04,// Nbcst
	// TupleType.N16b4
	0x10,// N
	0x04,// Nbcst
	// TupleType.N32b4
	0x20,// N
	0x04,// Nbcst
	// TupleType.N64b4
	0x40,// N
	0x04,// Nbcst
	// TupleType.N16b8
	0x10,// N
	0x08,// Nbcst
	// TupleType.N32b8
	0x20,// N
	0x08,// Nbcst
	// TupleType.N64b8
	0x40,// N
	0x08,// Nbcst
];
// GENERATOR-END: TupleTypeTable

#[must_use]
#[inline(always)]
pub(crate) fn get_disp8n(tuple_type: TupleType, bcst: bool) -> u32 {
	let index = ((tuple_type as usize) << 1) | (if bcst { 1 } else { 0 });
	// Safe, size of table is num(TupleType)*2
	(unsafe { *TUPLE_TYPE_TBL.get_unchecked(index) }) as u32
}
