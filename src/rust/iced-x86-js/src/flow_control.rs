// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use wasm_bindgen::prelude::*;

// GENERATOR-BEGIN: Enum
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Control flow
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum FlowControl {
	/// The next instruction that will be executed is the next instruction in the instruction stream
	Next = 0,
	/// It's an unconditional branch instruction: `JMP NEAR`, `JMP FAR`
	UnconditionalBranch = 1,
	/// It's an unconditional indirect branch: `JMP NEAR reg`, `JMP NEAR [mem]`, `JMP FAR [mem]`
	IndirectBranch = 2,
	/// It's a conditional branch instruction: `Jcc SHORT`, `Jcc NEAR`, `LOOP`, `LOOPcc`, `JRCXZ`
	ConditionalBranch = 3,
	/// It's a return instruction: `RET NEAR`, `RET FAR`, `IRET`, `SYSRET`, `SYSEXIT`, `RSM`, `SKINIT`, `RDM`, `UIRET`
	Return = 4,
	/// It's a call instruction: `CALL NEAR`, `CALL FAR`, `SYSCALL`, `SYSENTER`, `VMLAUNCH`, `VMRESUME`, `VMCALL`, `VMMCALL`, `VMGEXIT`, `VMRUN`, `TDCALL`, `SEAMCALL`, `SEAMRET`
	Call = 5,
	/// It's an indirect call instruction: `CALL NEAR reg`, `CALL NEAR [mem]`, `CALL FAR [mem]`
	IndirectCall = 6,
	/// It's an interrupt instruction: `INT n`, `INT3`, `INT1`, `INTO`, `SMINT`, `DMINT`
	Interrupt = 7,
	/// It's `XBEGIN`
	XbeginXabortXend = 8,
	/// It's an invalid instruction, eg. [`Code.INVALID`], `UD0`, `UD1`, `UD2`
	///
	/// [`Code.INVALID`]: enum.Code.html#variant.INVALID
	Exception = 9,
}
// GENERATOR-END: Enum

#[allow(dead_code)]
pub(crate) fn iced_to_flow_control(value: iced_x86_rust::FlowControl) -> FlowControl {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}
