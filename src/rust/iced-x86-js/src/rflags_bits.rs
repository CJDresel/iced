// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

use wasm_bindgen::prelude::*;

// GENERATOR-BEGIN: Enum
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// `RFLAGS` bits, FPU condition code bits and misc bits (`UIF`) supported by the instruction info code
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum RflagsBits {
	/// No bit is set
	None = 0x0000_0000,
	/// `RFLAGS.OF`
	OF = 0x0000_0001,
	/// `RFLAGS.SF`
	SF = 0x0000_0002,
	/// `RFLAGS.ZF`
	ZF = 0x0000_0004,
	/// `RFLAGS.AF`
	AF = 0x0000_0008,
	/// `RFLAGS.CF`
	CF = 0x0000_0010,
	/// `RFLAGS.PF`
	PF = 0x0000_0020,
	/// `RFLAGS.DF`
	DF = 0x0000_0040,
	/// `RFLAGS.IF`
	IF = 0x0000_0080,
	/// `RFLAGS.AC`
	AC = 0x0000_0100,
	/// `UIF`
	UIF = 0x0000_0200,
	/// FPU status word bit `C0`
	C0 = 0x0000_0400,
	/// FPU status word bit `C1`
	C1 = 0x0000_0800,
	/// FPU status word bit `C2`
	C2 = 0x0000_1000,
	/// FPU status word bit `C3`
	C3 = 0x0000_2000,
}
// GENERATOR-END: Enum
