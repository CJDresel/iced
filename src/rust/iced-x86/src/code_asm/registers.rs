// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

//! This module contains all registers that can be used.
//!
//! All register identifiers (eg. `eax`, `cr8`) are part of the public API but the
//! register *types* are *not*! They're an implementation detail.
//!
//! To use the registers, you must import everything from the module:
//!
//! ```
//! # #![allow(unused_imports)]
//! use iced_x86::code_asm::*;
//! ```
//!
//! or import them from this module:
//!
//! ```
//! # #![allow(unused_imports)]
//! use iced_x86::code_asm::registers::*;
//! ```
//!
//! or only the registers you need:
//!
//! ```
//! # #![allow(unused_imports)]
//! use iced_x86::code_asm::registers::gpr32::*;
//! use iced_x86::code_asm::registers::gpr64::*;
//! use iced_x86::code_asm::registers::xmm::*;
//! ```

#[rustfmt::skip]
pub mod gpr8 {
	//! All 8-bit general purpose registers.
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegister8;
	use crate::Register;
	pub const al: AsmRegister8 = AsmRegister8::new(Register::AL);
	pub const cl: AsmRegister8 = AsmRegister8::new(Register::CL);
	pub const dl: AsmRegister8 = AsmRegister8::new(Register::DL);
	pub const bl: AsmRegister8 = AsmRegister8::new(Register::BL);
	pub const ah: AsmRegister8 = AsmRegister8::new(Register::AH);
	pub const ch: AsmRegister8 = AsmRegister8::new(Register::CH);
	pub const dh: AsmRegister8 = AsmRegister8::new(Register::DH);
	pub const bh: AsmRegister8 = AsmRegister8::new(Register::BH);
	pub const spl: AsmRegister8 = AsmRegister8::new(Register::SPL);
	pub const bpl: AsmRegister8 = AsmRegister8::new(Register::BPL);
	pub const sil: AsmRegister8 = AsmRegister8::new(Register::SIL);
	pub const dil: AsmRegister8 = AsmRegister8::new(Register::DIL);
	pub const r8b: AsmRegister8 = AsmRegister8::new(Register::R8L);
	pub const r9b: AsmRegister8 = AsmRegister8::new(Register::R9L);
	pub const r10b: AsmRegister8 = AsmRegister8::new(Register::R10L);
	pub const r11b: AsmRegister8 = AsmRegister8::new(Register::R11L);
	pub const r12b: AsmRegister8 = AsmRegister8::new(Register::R12L);
	pub const r13b: AsmRegister8 = AsmRegister8::new(Register::R13L);
	pub const r14b: AsmRegister8 = AsmRegister8::new(Register::R14L);
	pub const r15b: AsmRegister8 = AsmRegister8::new(Register::R15L);
	/// Gets a `GPR8` register or `None` if input is invalid.
	pub fn get_gpr8(register: Register) -> Option<AsmRegister8> {
		if register.is_gpr8() {
			Some(AsmRegister8::new(register))
		} else {
			None
		}
	}
}

#[rustfmt::skip]
pub mod gpr16 {
	//! All 16-bit general purpose registers.
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegister16;
	use crate::Register;
	pub const ax: AsmRegister16 = AsmRegister16::new(Register::AX);
	pub const cx: AsmRegister16 = AsmRegister16::new(Register::CX);
	pub const dx: AsmRegister16 = AsmRegister16::new(Register::DX);
	pub const bx: AsmRegister16 = AsmRegister16::new(Register::BX);
	pub const sp: AsmRegister16 = AsmRegister16::new(Register::SP);
	pub const bp: AsmRegister16 = AsmRegister16::new(Register::BP);
	pub const si: AsmRegister16 = AsmRegister16::new(Register::SI);
	pub const di: AsmRegister16 = AsmRegister16::new(Register::DI);
	pub const r8w: AsmRegister16 = AsmRegister16::new(Register::R8W);
	pub const r9w: AsmRegister16 = AsmRegister16::new(Register::R9W);
	pub const r10w: AsmRegister16 = AsmRegister16::new(Register::R10W);
	pub const r11w: AsmRegister16 = AsmRegister16::new(Register::R11W);
	pub const r12w: AsmRegister16 = AsmRegister16::new(Register::R12W);
	pub const r13w: AsmRegister16 = AsmRegister16::new(Register::R13W);
	pub const r14w: AsmRegister16 = AsmRegister16::new(Register::R14W);
	pub const r15w: AsmRegister16 = AsmRegister16::new(Register::R15W);
	/// Gets a `GPR16` register or `None` if input is invalid.
	pub fn get_gpr16(register: Register) -> Option<AsmRegister16> {
		if register.is_gpr16() {
			Some(AsmRegister16::new(register))
		} else {
			None
		}
	}
}

#[rustfmt::skip]
pub mod gpr32 {
	//! All 32-bit general purpose registers.
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegister32;
	use crate::Register;
	pub const eax: AsmRegister32 = AsmRegister32::new(Register::EAX);
	pub const ecx: AsmRegister32 = AsmRegister32::new(Register::ECX);
	pub const edx: AsmRegister32 = AsmRegister32::new(Register::EDX);
	pub const ebx: AsmRegister32 = AsmRegister32::new(Register::EBX);
	pub const esp: AsmRegister32 = AsmRegister32::new(Register::ESP);
	pub const ebp: AsmRegister32 = AsmRegister32::new(Register::EBP);
	pub const esi: AsmRegister32 = AsmRegister32::new(Register::ESI);
	pub const edi: AsmRegister32 = AsmRegister32::new(Register::EDI);
	pub const r8d: AsmRegister32 = AsmRegister32::new(Register::R8D);
	pub const r9d: AsmRegister32 = AsmRegister32::new(Register::R9D);
	pub const r10d: AsmRegister32 = AsmRegister32::new(Register::R10D);
	pub const r11d: AsmRegister32 = AsmRegister32::new(Register::R11D);
	pub const r12d: AsmRegister32 = AsmRegister32::new(Register::R12D);
	pub const r13d: AsmRegister32 = AsmRegister32::new(Register::R13D);
	pub const r14d: AsmRegister32 = AsmRegister32::new(Register::R14D);
	pub const r15d: AsmRegister32 = AsmRegister32::new(Register::R15D);
	/// Gets a `GPR32` register or `None` if input is invalid.
	pub fn get_gpr32(register: Register) -> Option<AsmRegister32> {
		if register.is_gpr32() {
			Some(AsmRegister32::new(register))
		} else {
			None
		}
	}
}

#[rustfmt::skip]
pub mod gpr64 {
	//! All 64-bit general purpose registers.
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegister64;
	use crate::Register;
	pub const rax: AsmRegister64 = AsmRegister64::new(Register::RAX);
	pub const rcx: AsmRegister64 = AsmRegister64::new(Register::RCX);
	pub const rdx: AsmRegister64 = AsmRegister64::new(Register::RDX);
	pub const rbx: AsmRegister64 = AsmRegister64::new(Register::RBX);
	pub const rsp: AsmRegister64 = AsmRegister64::new(Register::RSP);
	pub const rbp: AsmRegister64 = AsmRegister64::new(Register::RBP);
	pub const rsi: AsmRegister64 = AsmRegister64::new(Register::RSI);
	pub const rdi: AsmRegister64 = AsmRegister64::new(Register::RDI);
	pub const r8: AsmRegister64 = AsmRegister64::new(Register::R8);
	pub const r9: AsmRegister64 = AsmRegister64::new(Register::R9);
	pub const r10: AsmRegister64 = AsmRegister64::new(Register::R10);
	pub const r11: AsmRegister64 = AsmRegister64::new(Register::R11);
	pub const r12: AsmRegister64 = AsmRegister64::new(Register::R12);
	pub const r13: AsmRegister64 = AsmRegister64::new(Register::R13);
	pub const r14: AsmRegister64 = AsmRegister64::new(Register::R14);
	pub const r15: AsmRegister64 = AsmRegister64::new(Register::R15);
	/// Gets a `GPR64` register or `None` if input is invalid.
	pub fn get_gpr64(register: Register) -> Option<AsmRegister64> {
		if register.is_gpr64() {
			Some(AsmRegister64::new(register))
		} else {
			None
		}
	}
}

#[rustfmt::skip]
pub mod segment {
	//! All segment registers.
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegisterSegment;
	use crate::Register;
	pub const es: AsmRegisterSegment = AsmRegisterSegment::new(Register::ES);
	pub const cs: AsmRegisterSegment = AsmRegisterSegment::new(Register::CS);
	pub const ss: AsmRegisterSegment = AsmRegisterSegment::new(Register::SS);
	pub const ds: AsmRegisterSegment = AsmRegisterSegment::new(Register::DS);
	pub const fs: AsmRegisterSegment = AsmRegisterSegment::new(Register::FS);
	pub const gs: AsmRegisterSegment = AsmRegisterSegment::new(Register::GS);
	/// Gets a `SEGMENT` register or `None` if input is invalid.
	pub fn get_segment(register: Register) -> Option<AsmRegisterSegment> {
		if register.is_segment_register() {
			Some(AsmRegisterSegment::new(register))
		} else {
			None
		}
	}
}

#[rustfmt::skip]
pub mod st {
	//! All FPU registers.
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegisterSt;
	use crate::Register;
	pub const st0: AsmRegisterSt = AsmRegisterSt::new(Register::ST0);
	pub const st1: AsmRegisterSt = AsmRegisterSt::new(Register::ST1);
	pub const st2: AsmRegisterSt = AsmRegisterSt::new(Register::ST2);
	pub const st3: AsmRegisterSt = AsmRegisterSt::new(Register::ST3);
	pub const st4: AsmRegisterSt = AsmRegisterSt::new(Register::ST4);
	pub const st5: AsmRegisterSt = AsmRegisterSt::new(Register::ST5);
	pub const st6: AsmRegisterSt = AsmRegisterSt::new(Register::ST6);
	pub const st7: AsmRegisterSt = AsmRegisterSt::new(Register::ST7);
	/// Gets an `ST` register or `None` if input is invalid.
	pub fn get_st(register: Register) -> Option<AsmRegisterSt> {
		if register.is_st() {
			Some(AsmRegisterSt::new(register))
		} else {
			None
		}
	}
}

#[rustfmt::skip]
pub mod cr {
	//! All control registers.
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegisterCr;
	use crate::Register;
	pub const cr0: AsmRegisterCr = AsmRegisterCr::new(Register::CR0);
	pub const cr1: AsmRegisterCr = AsmRegisterCr::new(Register::CR1);
	pub const cr2: AsmRegisterCr = AsmRegisterCr::new(Register::CR2);
	pub const cr3: AsmRegisterCr = AsmRegisterCr::new(Register::CR3);
	pub const cr4: AsmRegisterCr = AsmRegisterCr::new(Register::CR4);
	pub const cr5: AsmRegisterCr = AsmRegisterCr::new(Register::CR5);
	pub const cr6: AsmRegisterCr = AsmRegisterCr::new(Register::CR6);
	pub const cr7: AsmRegisterCr = AsmRegisterCr::new(Register::CR7);
	pub const cr8: AsmRegisterCr = AsmRegisterCr::new(Register::CR8);
	pub const cr9: AsmRegisterCr = AsmRegisterCr::new(Register::CR9);
	pub const cr10: AsmRegisterCr = AsmRegisterCr::new(Register::CR10);
	pub const cr11: AsmRegisterCr = AsmRegisterCr::new(Register::CR11);
	pub const cr12: AsmRegisterCr = AsmRegisterCr::new(Register::CR12);
	pub const cr13: AsmRegisterCr = AsmRegisterCr::new(Register::CR13);
	pub const cr14: AsmRegisterCr = AsmRegisterCr::new(Register::CR14);
	pub const cr15: AsmRegisterCr = AsmRegisterCr::new(Register::CR15);
	/// Gets a `CR` register or `None` if input is invalid.
	pub fn get_cr(register: Register) -> Option<AsmRegisterCr> {
		if register.is_cr() {
			Some(AsmRegisterCr::new(register))
		} else {
			None
		}
	}
}

#[rustfmt::skip]
pub mod dr {
	//! All debug registers.
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegisterDr;
	use crate::Register;
	pub const dr0: AsmRegisterDr = AsmRegisterDr::new(Register::DR0);
	pub const dr1: AsmRegisterDr = AsmRegisterDr::new(Register::DR1);
	pub const dr2: AsmRegisterDr = AsmRegisterDr::new(Register::DR2);
	pub const dr3: AsmRegisterDr = AsmRegisterDr::new(Register::DR3);
	pub const dr4: AsmRegisterDr = AsmRegisterDr::new(Register::DR4);
	pub const dr5: AsmRegisterDr = AsmRegisterDr::new(Register::DR5);
	pub const dr6: AsmRegisterDr = AsmRegisterDr::new(Register::DR6);
	pub const dr7: AsmRegisterDr = AsmRegisterDr::new(Register::DR7);
	pub const dr8: AsmRegisterDr = AsmRegisterDr::new(Register::DR8);
	pub const dr9: AsmRegisterDr = AsmRegisterDr::new(Register::DR9);
	pub const dr10: AsmRegisterDr = AsmRegisterDr::new(Register::DR10);
	pub const dr11: AsmRegisterDr = AsmRegisterDr::new(Register::DR11);
	pub const dr12: AsmRegisterDr = AsmRegisterDr::new(Register::DR12);
	pub const dr13: AsmRegisterDr = AsmRegisterDr::new(Register::DR13);
	pub const dr14: AsmRegisterDr = AsmRegisterDr::new(Register::DR14);
	pub const dr15: AsmRegisterDr = AsmRegisterDr::new(Register::DR15);
	/// Gets a `DR` register or `None` if input is invalid.
	pub fn get_dr(register: Register) -> Option<AsmRegisterDr> {
		if register.is_dr() {
			Some(AsmRegisterDr::new(register))
		} else {
			None
		}
	}
}

#[rustfmt::skip]
pub mod tr {
	//! All test registers.
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegisterTr;
	use crate::Register;
	pub const tr0: AsmRegisterTr = AsmRegisterTr::new(Register::TR0);
	pub const tr1: AsmRegisterTr = AsmRegisterTr::new(Register::TR1);
	pub const tr2: AsmRegisterTr = AsmRegisterTr::new(Register::TR2);
	pub const tr3: AsmRegisterTr = AsmRegisterTr::new(Register::TR3);
	pub const tr4: AsmRegisterTr = AsmRegisterTr::new(Register::TR4);
	pub const tr5: AsmRegisterTr = AsmRegisterTr::new(Register::TR5);
	pub const tr6: AsmRegisterTr = AsmRegisterTr::new(Register::TR6);
	pub const tr7: AsmRegisterTr = AsmRegisterTr::new(Register::TR7);
	/// Gets a `TR` register or `None` if input is invalid.
	pub fn get_tr(register: Register) -> Option<AsmRegisterTr> {
		if register.is_tr() {
			Some(AsmRegisterTr::new(register))
		} else {
			None
		}
	}
}

#[rustfmt::skip]
pub mod bnd {
	//! All bound registers.
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegisterBnd;
	use crate::Register;
	pub const bnd0: AsmRegisterBnd = AsmRegisterBnd::new(Register::BND0);
	pub const bnd1: AsmRegisterBnd = AsmRegisterBnd::new(Register::BND1);
	pub const bnd2: AsmRegisterBnd = AsmRegisterBnd::new(Register::BND2);
	pub const bnd3: AsmRegisterBnd = AsmRegisterBnd::new(Register::BND3);
	/// Gets a `BND` register or `None` if input is invalid.
	pub fn get_bnd(register: Register) -> Option<AsmRegisterBnd> {
		if register.is_bnd() {
			Some(AsmRegisterBnd::new(register))
		} else {
			None
		}
	}
}

#[rustfmt::skip]
pub mod k {
	//! All opmask registers.
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegisterK;
	use crate::Register;
	pub const k0: AsmRegisterK = AsmRegisterK::new(Register::K0);
	pub const k1: AsmRegisterK = AsmRegisterK::new(Register::K1);
	pub const k2: AsmRegisterK = AsmRegisterK::new(Register::K2);
	pub const k3: AsmRegisterK = AsmRegisterK::new(Register::K3);
	pub const k4: AsmRegisterK = AsmRegisterK::new(Register::K4);
	pub const k5: AsmRegisterK = AsmRegisterK::new(Register::K5);
	pub const k6: AsmRegisterK = AsmRegisterK::new(Register::K6);
	pub const k7: AsmRegisterK = AsmRegisterK::new(Register::K7);
	/// Gets a `K` register or `None` if input is invalid.
	pub fn get_k(register: Register) -> Option<AsmRegisterK> {
		if register.is_k() {
			Some(AsmRegisterK::new(register))
		} else {
			None
		}
	}
}

#[rustfmt::skip]
pub mod mm {
	//! All MMX registers.
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegisterMm;
	use crate::Register;
	pub const mm0: AsmRegisterMm = AsmRegisterMm::new(Register::MM0);
	pub const mm1: AsmRegisterMm = AsmRegisterMm::new(Register::MM1);
	pub const mm2: AsmRegisterMm = AsmRegisterMm::new(Register::MM2);
	pub const mm3: AsmRegisterMm = AsmRegisterMm::new(Register::MM3);
	pub const mm4: AsmRegisterMm = AsmRegisterMm::new(Register::MM4);
	pub const mm5: AsmRegisterMm = AsmRegisterMm::new(Register::MM5);
	pub const mm6: AsmRegisterMm = AsmRegisterMm::new(Register::MM6);
	pub const mm7: AsmRegisterMm = AsmRegisterMm::new(Register::MM7);
	/// Gets an `MM` register or `None` if input is invalid.
	pub fn get_mm(register: Register) -> Option<AsmRegisterMm> {
		if register.is_mm() {
			Some(AsmRegisterMm::new(register))
		} else {
			None
		}
	}
}

#[rustfmt::skip]
pub mod xmm {
	//! All 128-bit vector registers (XMM).
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegisterXmm;
	use crate::Register;
	pub const xmm0: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM0);
	pub const xmm1: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM1);
	pub const xmm2: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM2);
	pub const xmm3: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM3);
	pub const xmm4: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM4);
	pub const xmm5: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM5);
	pub const xmm6: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM6);
	pub const xmm7: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM7);
	pub const xmm8: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM8);
	pub const xmm9: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM9);
	pub const xmm10: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM10);
	pub const xmm11: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM11);
	pub const xmm12: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM12);
	pub const xmm13: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM13);
	pub const xmm14: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM14);
	pub const xmm15: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM15);
	pub const xmm16: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM16);
	pub const xmm17: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM17);
	pub const xmm18: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM18);
	pub const xmm19: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM19);
	pub const xmm20: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM20);
	pub const xmm21: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM21);
	pub const xmm22: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM22);
	pub const xmm23: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM23);
	pub const xmm24: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM24);
	pub const xmm25: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM25);
	pub const xmm26: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM26);
	pub const xmm27: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM27);
	pub const xmm28: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM28);
	pub const xmm29: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM29);
	pub const xmm30: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM30);
	pub const xmm31: AsmRegisterXmm = AsmRegisterXmm::new(Register::XMM31);
	/// Gets an `XMM` register or `None` if input is invalid.
	pub fn get_xmm(register: Register) -> Option<AsmRegisterXmm> {
		if register.is_xmm() {
			Some(AsmRegisterXmm::new(register))
		} else {
			None
		}
	}
}

#[rustfmt::skip]
pub mod ymm {
	//! All 256-bit vector registers (YMM).
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegisterYmm;
	use crate::Register;
	pub const ymm0: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM0);
	pub const ymm1: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM1);
	pub const ymm2: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM2);
	pub const ymm3: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM3);
	pub const ymm4: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM4);
	pub const ymm5: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM5);
	pub const ymm6: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM6);
	pub const ymm7: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM7);
	pub const ymm8: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM8);
	pub const ymm9: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM9);
	pub const ymm10: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM10);
	pub const ymm11: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM11);
	pub const ymm12: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM12);
	pub const ymm13: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM13);
	pub const ymm14: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM14);
	pub const ymm15: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM15);
	pub const ymm16: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM16);
	pub const ymm17: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM17);
	pub const ymm18: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM18);
	pub const ymm19: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM19);
	pub const ymm20: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM20);
	pub const ymm21: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM21);
	pub const ymm22: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM22);
	pub const ymm23: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM23);
	pub const ymm24: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM24);
	pub const ymm25: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM25);
	pub const ymm26: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM26);
	pub const ymm27: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM27);
	pub const ymm28: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM28);
	pub const ymm29: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM29);
	pub const ymm30: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM30);
	pub const ymm31: AsmRegisterYmm = AsmRegisterYmm::new(Register::YMM31);
	/// Gets a `YMM` register or `None` if input is invalid.
	pub fn get_ymm(register: Register) -> Option<AsmRegisterYmm> {
		if register.is_ymm() {
			Some(AsmRegisterYmm::new(register))
		} else {
			None
		}
	}
}

#[rustfmt::skip]
pub mod zmm {
	//! All 512-bit vector registers (ZMM).
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegisterZmm;
	use crate::Register;
	pub const zmm0: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM0);
	pub const zmm1: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM1);
	pub const zmm2: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM2);
	pub const zmm3: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM3);
	pub const zmm4: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM4);
	pub const zmm5: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM5);
	pub const zmm6: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM6);
	pub const zmm7: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM7);
	pub const zmm8: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM8);
	pub const zmm9: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM9);
	pub const zmm10: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM10);
	pub const zmm11: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM11);
	pub const zmm12: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM12);
	pub const zmm13: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM13);
	pub const zmm14: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM14);
	pub const zmm15: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM15);
	pub const zmm16: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM16);
	pub const zmm17: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM17);
	pub const zmm18: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM18);
	pub const zmm19: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM19);
	pub const zmm20: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM20);
	pub const zmm21: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM21);
	pub const zmm22: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM22);
	pub const zmm23: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM23);
	pub const zmm24: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM24);
	pub const zmm25: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM25);
	pub const zmm26: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM26);
	pub const zmm27: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM27);
	pub const zmm28: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM28);
	pub const zmm29: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM29);
	pub const zmm30: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM30);
	pub const zmm31: AsmRegisterZmm = AsmRegisterZmm::new(Register::ZMM31);
	/// Gets a `ZMM` register or `None` if input is invalid.
	pub fn get_zmm(register: Register) -> Option<AsmRegisterZmm> {
		if register.is_zmm() {
			Some(AsmRegisterZmm::new(register))
		} else {
			None
		}
	}
}

#[rustfmt::skip]
pub mod tmm {
	//! All tile registers.
	#![allow(non_upper_case_globals)]
	#![allow(missing_docs)]
	use crate::code_asm::reg::AsmRegisterTmm;
	use crate::Register;
	pub const tmm0: AsmRegisterTmm = AsmRegisterTmm::new(Register::TMM0);
	pub const tmm1: AsmRegisterTmm = AsmRegisterTmm::new(Register::TMM1);
	pub const tmm2: AsmRegisterTmm = AsmRegisterTmm::new(Register::TMM2);
	pub const tmm3: AsmRegisterTmm = AsmRegisterTmm::new(Register::TMM3);
	pub const tmm4: AsmRegisterTmm = AsmRegisterTmm::new(Register::TMM4);
	pub const tmm5: AsmRegisterTmm = AsmRegisterTmm::new(Register::TMM5);
	pub const tmm6: AsmRegisterTmm = AsmRegisterTmm::new(Register::TMM6);
	pub const tmm7: AsmRegisterTmm = AsmRegisterTmm::new(Register::TMM7);
	/// Gets a `TMM` register or `None` if input is invalid.
	pub fn get_tmm(register: Register) -> Option<AsmRegisterTmm> {
		if register.is_tmm() {
			Some(AsmRegisterTmm::new(register))
		} else {
			None
		}
	}
}

pub use self::bnd::*;
pub use self::cr::*;
pub use self::dr::*;
pub use self::gpr16::*;
pub use self::gpr32::*;
pub use self::gpr64::*;
pub use self::gpr8::*;
pub use self::k::*;
pub use self::mm::*;
pub use self::segment::*;
pub use self::st::*;
pub use self::tmm::*;
pub use self::tr::*;
pub use self::xmm::*;
pub use self::ymm::*;
pub use self::zmm::*;
