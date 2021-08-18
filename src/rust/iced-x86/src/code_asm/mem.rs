// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::code_asm::op_state::{CodeAsmOpState, MemoryOperandSize};
use crate::code_asm::{AsmRegister16, AsmRegister32, AsmRegister64, AsmRegisterXmm, AsmRegisterYmm, AsmRegisterZmm, CodeLabel};
use crate::{MemoryOperand, Register};
use core::ops::{Add, Mul, Sub};

/// A memory operand passed to [`CodeAssembler`] methods.
///
/// [`CodeAssembler`]: struct.CodeAssembler.html
///
/// The struct name is not part of the public API. Its methods, however, are part of the public API.
///
/// You can create a memory operand in many different ways:
///
/// ```
/// use iced_x86::code_asm::*;
///
/// // <???> ptr [rax+rdx*4]
/// let _ = rax + rdx * 4;
/// // byte ptr [rax]
/// let _ = byte_ptr(rax);
/// // <???> ptr gs:[rcx*4+123]
/// let _ = mem(rcx * 4 + 123).gs();
/// // qword bcst [rdx+xmm0*8+123]
/// let _ = qword_bcst(rdx + xmm0 * 8 + 123);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsmMemoryOperand {
	base: Register,
	index: Register,
	scale: u8,
	displ: i64,
	state: CodeAsmOpState,
}

impl AsmMemoryOperand {
	// GENERATOR-BEGIN: AsmMemoryOperandPtrMethods
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn mem(mut self) -> Self {
		self.state.mem(MemoryOperandSize::None);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn byte_ptr(mut self) -> Self {
		self.state.mem(MemoryOperandSize::Byte);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn word_ptr(mut self) -> Self {
		self.state.mem(MemoryOperandSize::Word);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn dword_ptr(mut self) -> Self {
		self.state.mem(MemoryOperandSize::Dword);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn qword_ptr(mut self) -> Self {
		self.state.mem(MemoryOperandSize::Qword);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn mmword_ptr(mut self) -> Self {
		self.state.mem(MemoryOperandSize::Qword);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn tbyte_ptr(mut self) -> Self {
		self.state.mem(MemoryOperandSize::Tbyte);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn tword_ptr(mut self) -> Self {
		self.state.mem(MemoryOperandSize::Tbyte);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn fword_ptr(mut self) -> Self {
		self.state.mem(MemoryOperandSize::Fword);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn oword_ptr(mut self) -> Self {
		self.state.mem(MemoryOperandSize::Xword);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn xmmword_ptr(mut self) -> Self {
		self.state.mem(MemoryOperandSize::Xword);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn ymmword_ptr(mut self) -> Self {
		self.state.mem(MemoryOperandSize::Yword);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn zmmword_ptr(mut self) -> Self {
		self.state.mem(MemoryOperandSize::Zword);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn bcst(mut self) -> Self {
		self.state.bcst(MemoryOperandSize::None);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn word_bcst(mut self) -> Self {
		self.state.bcst(MemoryOperandSize::Word);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn dword_bcst(mut self) -> Self {
		self.state.bcst(MemoryOperandSize::Dword);
		self
	}

	#[must_use]
	#[inline]
	#[rustfmt::skip]
	pub(crate) fn qword_bcst(mut self) -> Self {
		self.state.bcst(MemoryOperandSize::Qword);
		self
	}
	// GENERATOR-END: AsmMemoryOperandPtrMethods

	/// Adds an `ES` segment override
	#[must_use]
	#[inline]
	pub fn es(mut self) -> Self {
		self.state.set_es();
		self
	}

	/// Adds a `CS` segment override
	#[must_use]
	#[inline]
	pub fn cs(mut self) -> Self {
		self.state.set_cs();
		self
	}

	/// Adds an `SS` segment override
	#[must_use]
	#[inline]
	pub fn ss(mut self) -> Self {
		self.state.set_ss();
		self
	}

	/// Adds a `DS` segment override
	#[must_use]
	#[inline]
	pub fn ds(mut self) -> Self {
		self.state.set_ds();
		self
	}

	/// Adds an `FS` segment override
	#[must_use]
	#[inline]
	pub fn fs(mut self) -> Self {
		self.state.set_fs();
		self
	}

	/// Adds a `GS` segment override
	#[must_use]
	#[inline]
	pub fn gs(mut self) -> Self {
		self.state.set_gs();
		self
	}

	/// Adds a `{k1}` opmask register
	#[must_use]
	#[inline]
	pub fn k1(mut self) -> Self {
		self.state.set_k1();
		self
	}

	/// Adds a `{k2}` opmask register
	#[must_use]
	#[inline]
	pub fn k2(mut self) -> Self {
		self.state.set_k2();
		self
	}

	/// Adds a `{k3}` opmask register
	#[must_use]
	#[inline]
	pub fn k3(mut self) -> Self {
		self.state.set_k3();
		self
	}

	/// Adds a `{k4}` opmask register
	#[must_use]
	#[inline]
	pub fn k4(mut self) -> Self {
		self.state.set_k4();
		self
	}

	/// Adds a `{k5}` opmask register
	#[must_use]
	#[inline]
	pub fn k5(mut self) -> Self {
		self.state.set_k5();
		self
	}

	/// Adds a `{k6}` opmask register
	#[must_use]
	#[inline]
	pub fn k6(mut self) -> Self {
		self.state.set_k6();
		self
	}

	/// Adds a `{k7}` opmask register
	#[must_use]
	#[inline]
	pub fn k7(mut self) -> Self {
		self.state.set_k7();
		self
	}

	#[must_use]
	#[inline]
	pub(crate) fn is_displacement_only(&self) -> bool {
		self.base == Register::None && self.index == Register::None
	}

	#[must_use]
	#[inline]
	pub(crate) fn to_memory_operand(self, bitness: u32) -> MemoryOperand {
		MemoryOperand {
			segment_prefix: self.state.segment(),
			base: self.base,
			index: self.index,
			scale: self.scale as u32,
			displacement: self.displ,
			displ_size: if self.is_displacement_only() {
				bitness / 8
			} else if self.displ == 0 {
				0
			} else {
				1
			},
			is_broadcast: self.is_broadcast(),
		}
	}

	#[must_use]
	#[inline]
	pub(crate) fn is_broadcast(&self) -> bool {
		self.state.is_broadcast()
	}

	#[must_use]
	#[inline]
	pub(crate) fn size(&self) -> MemoryOperandSize {
		self.state.size()
	}

	#[must_use]
	#[inline]
	pub(crate) fn index(&self) -> Register {
		self.index
	}

	#[must_use]
	#[inline]
	pub(crate) fn state(&self) -> CodeAsmOpState {
		self.state
	}
}

impl<R: Into<Register>> From<R> for AsmMemoryOperand {
	#[inline]
	fn from(other: R) -> Self {
		AsmMemoryOperand { base: other.into(), index: Register::None, scale: 1, displ: 0, state: CodeAsmOpState::new() }
	}
}

macro_rules! displ_to_mem_op {
	($($displ_ty:ty)+) => {
		$(
			impl From<$displ_ty> for AsmMemoryOperand {
				#[inline]
				#[allow(trivial_numeric_casts)]
				fn from(other: $displ_ty) -> Self {
					AsmMemoryOperand {
						base: Register::None,
						index: Register::None,
						scale: 1,
						displ: other as i64,
						state: CodeAsmOpState::new(),
					}
				}
			}
		)+
	};
}
displ_to_mem_op!(i8 i16 i32 i64 isize u8 u16 u32 u64 usize);

macro_rules! reg_plus_displ {
	($reg_ty:ty, $($displ_ty:ty)+) => {
		$(
			impl Add<$displ_ty> for $reg_ty {
				type Output = AsmMemoryOperand;
				#[inline]
				#[allow(trivial_numeric_casts)]
				fn add(self, rhs: $displ_ty) -> Self::Output {
					AsmMemoryOperand {
						base: self.into(),
						index: Register::None,
						scale: 1,
						displ: rhs as i64,
						state: CodeAsmOpState::new(),
					}
				}
			}
			impl Add<$reg_ty> for $displ_ty {
				type Output = AsmMemoryOperand;
				#[inline]
				#[allow(trivial_numeric_casts)]
				fn add(self, rhs: $reg_ty) -> Self::Output {
					AsmMemoryOperand {
						base: rhs.into(),
						index: Register::None,
						scale: 1,
						displ: self as i64,
						state: CodeAsmOpState::new(),
					}
				}
			}
			impl Sub<$displ_ty> for $reg_ty {
				type Output = AsmMemoryOperand;
				#[inline]
				#[allow(trivial_numeric_casts)]
				fn sub(self, rhs: $displ_ty) -> Self::Output {
					AsmMemoryOperand {
						base: self.into(),
						index: Register::None,
						scale: 1,
						displ: (rhs as i64).wrapping_neg(),
						state: CodeAsmOpState::new(),
					}
				}
			}
		)+
	};
}
reg_plus_displ!(AsmRegister16, i8 i16 i32 i64 isize u8 u16 u32 u64 usize);
reg_plus_displ!(AsmRegister32, i8 i16 i32 i64 isize u8 u16 u32 u64 usize);
reg_plus_displ!(AsmRegister64, i8 i16 i32 i64 isize u8 u16 u32 u64 usize);

macro_rules! reg_mul_scale {
	($reg_ty:ty, $($scale_ty:ty)+) => {
		$(
			impl Mul<$scale_ty> for $reg_ty {
				type Output = AsmMemoryOperand;
				#[inline]
				#[allow(trivial_numeric_casts)]
				fn mul(self, rhs: $scale_ty) -> Self::Output {
					AsmMemoryOperand {
						base: Register::None,
						index: self.into(),
						scale: rhs as u8,
						displ: 0,
						state: CodeAsmOpState::new(),
					}
				}
			}
			impl Mul<$reg_ty> for $scale_ty {
				type Output = AsmMemoryOperand;
				#[inline]
				#[allow(trivial_numeric_casts)]
				fn mul(self, rhs: $reg_ty) -> Self::Output {
					AsmMemoryOperand {
						base: Register::None,
						index: rhs.into(),
						scale: self as u8,
						displ: 0,
						state: CodeAsmOpState::new(),
					}
				}
			}
		)+
	};
}
reg_mul_scale!(AsmRegister32, i8 i16 i32 i64 isize u8 u16 u32 u64 usize);
reg_mul_scale!(AsmRegister64, i8 i16 i32 i64 isize u8 u16 u32 u64 usize);
reg_mul_scale!(AsmRegisterXmm, i8 i16 i32 i64 isize u8 u16 u32 u64 usize);
reg_mul_scale!(AsmRegisterYmm, i8 i16 i32 i64 isize u8 u16 u32 u64 usize);
reg_mul_scale!(AsmRegisterZmm, i8 i16 i32 i64 isize u8 u16 u32 u64 usize);

macro_rules! reg_plus_reg {
	($left_ty:ty, $($right_ty:ty)+) => {
		$(
			impl Add<$right_ty> for $left_ty {
				type Output = AsmMemoryOperand;
				#[inline]
				fn add(self, rhs: $right_ty) -> Self::Output {
					AsmMemoryOperand { base: self.into(), index: rhs.into(), scale: 1, displ: 0, state: CodeAsmOpState::new() }
				}
			}
		)+
	};
}
reg_plus_reg!(AsmRegister16, AsmRegister16);
reg_plus_reg!(AsmRegister32, AsmRegister32 AsmRegisterXmm AsmRegisterYmm AsmRegisterZmm);
reg_plus_reg!(AsmRegister64, AsmRegister64 AsmRegisterXmm AsmRegisterYmm AsmRegisterZmm);

// Special case `vec + reg` and treat `vec` as the index register (it can't be the base)
macro_rules! indexreg_plus_reg {
	($($left_ty:ty)+, $right_ty:ty) => {
		$(
			impl Add<$right_ty> for $left_ty {
				type Output = AsmMemoryOperand;
				#[inline]
				fn add(self, rhs: $right_ty) -> Self::Output {
					AsmMemoryOperand { base: rhs.into(), index: self.into(), scale: 1, displ: 0, state: CodeAsmOpState::new() }
				}
			}
		)+
	};
}
indexreg_plus_reg!(AsmRegisterXmm AsmRegisterYmm AsmRegisterZmm, AsmRegister32);
indexreg_plus_reg!(AsmRegisterXmm AsmRegisterYmm AsmRegisterZmm, AsmRegister64);

macro_rules! reg_plus_mem {
	($($reg_ty:ty)+) => {
		$(
			impl Add<AsmMemoryOperand> for $reg_ty {
				type Output = AsmMemoryOperand;
				#[inline]
				fn add(self, mut rhs: AsmMemoryOperand) -> Self::Output {
					debug_assert_eq!(rhs.base, Register::None);
					rhs.base = self.into();
					rhs
				}
			}

			impl Add<$reg_ty> for AsmMemoryOperand {
				type Output = AsmMemoryOperand;
				#[inline]
				fn add(mut self, rhs: $reg_ty) -> Self::Output {
					debug_assert_eq!(self.base, Register::None);
					self.base = rhs.into();
					self
				}
			}
		)+
	};
}
reg_plus_mem!(AsmRegister16 AsmRegister32 AsmRegister64);

macro_rules! mem_plus_displ {
	($($displ_ty:ty)+) => {
		$(
			impl Add<$displ_ty> for AsmMemoryOperand {
				type Output = AsmMemoryOperand;
				#[inline]
				#[allow(trivial_numeric_casts)]
				fn add(mut self, rhs: $displ_ty) -> Self::Output {
					self.displ = self.displ.wrapping_add(rhs as i64);
					self
				}
			}
			impl Add<AsmMemoryOperand> for $displ_ty {
				type Output = AsmMemoryOperand;
				#[inline]
				#[allow(trivial_numeric_casts)]
				fn add(self, mut rhs: AsmMemoryOperand) -> Self::Output {
					rhs.displ = rhs.displ.wrapping_add(self as i64);
					rhs
				}
			}
			impl Sub<$displ_ty> for AsmMemoryOperand {
				type Output = AsmMemoryOperand;
				#[inline]
				#[allow(trivial_numeric_casts)]
				fn sub(mut self, rhs: $displ_ty) -> Self::Output {
					self.displ = self.displ.wrapping_sub(rhs as i64);
					self
				}
			}
		)+
	};
}
mem_plus_displ!(i8 i16 i32 i64 isize u8 u16 u32 u64 usize);

impl Add<AsmMemoryOperand> for AsmMemoryOperand {
	type Output = AsmMemoryOperand;
	#[inline]
	fn add(mut self, rhs: AsmMemoryOperand) -> Self::Output {
		if self.base == Register::None {
			self.base = rhs.base;
		} else {
			debug_assert_eq!(rhs.base, Register::None);
		}
		if self.index == Register::None {
			self.index = rhs.index;
			self.scale = rhs.scale;
		} else {
			debug_assert_eq!(rhs.index, Register::None);
		}
		self.displ = self.displ.wrapping_add(rhs.displ);
		// The remaining fields aren't copied/updated from rhs, eg. segment register, rc, etc.
		self
	}
}

impl From<CodeLabel> for AsmMemoryOperand {
	#[inline]
	fn from(other: CodeLabel) -> Self {
		AsmMemoryOperand { base: Register::RIP, index: Register::None, scale: 1, displ: other.id as i64, state: CodeAsmOpState::new() }
	}
}

// GENERATOR-BEGIN: GlobalPtrMethods
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Creates a memory operand with no size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = mem(rax);
/// let _ = mem(0x1234_5678).fs();
/// let _ = mem(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn mem<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().mem()
}

/// Creates a memory operand with a `BYTE PTR` size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = byte_ptr(rax);
/// let _ = byte_ptr(0x1234_5678).fs();
/// let _ = byte_ptr(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn byte_ptr<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().byte_ptr()
}

/// Creates a memory operand with a `WORD PTR` size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = word_ptr(rax);
/// let _ = word_ptr(0x1234_5678).fs();
/// let _ = word_ptr(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn word_ptr<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().word_ptr()
}

/// Creates a memory operand with a `DWORD PTR` size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = dword_ptr(rax);
/// let _ = dword_ptr(0x1234_5678).fs();
/// let _ = dword_ptr(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn dword_ptr<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().dword_ptr()
}

/// Creates a memory operand with a `QWORD PTR` size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = qword_ptr(rax);
/// let _ = qword_ptr(0x1234_5678).fs();
/// let _ = qword_ptr(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn qword_ptr<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().qword_ptr()
}

/// Creates a memory operand with an `MMWORD PTR` size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = mmword_ptr(rax);
/// let _ = mmword_ptr(0x1234_5678).fs();
/// let _ = mmword_ptr(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn mmword_ptr<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().mmword_ptr()
}

/// Creates a memory operand with a `TBYTE PTR` size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = tbyte_ptr(rax);
/// let _ = tbyte_ptr(0x1234_5678).fs();
/// let _ = tbyte_ptr(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn tbyte_ptr<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().tbyte_ptr()
}

/// Creates a memory operand with a `TWORD PTR` size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = tword_ptr(rax);
/// let _ = tword_ptr(0x1234_5678).fs();
/// let _ = tword_ptr(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn tword_ptr<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().tword_ptr()
}

/// Creates a memory operand with an `FWORD PTR` size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = fword_ptr(rax);
/// let _ = fword_ptr(0x1234_5678).fs();
/// let _ = fword_ptr(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn fword_ptr<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().fword_ptr()
}

/// Creates a memory operand with an `OWORD PTR` size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = oword_ptr(rax);
/// let _ = oword_ptr(0x1234_5678).fs();
/// let _ = oword_ptr(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn oword_ptr<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().oword_ptr()
}

/// Creates a memory operand with an `XMMWORD PTR` size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = xmmword_ptr(rax);
/// let _ = xmmword_ptr(0x1234_5678).fs();
/// let _ = xmmword_ptr(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn xmmword_ptr<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().xmmword_ptr()
}

/// Creates a memory operand with a `YMMWORD PTR` size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = ymmword_ptr(rax);
/// let _ = ymmword_ptr(0x1234_5678).fs();
/// let _ = ymmword_ptr(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn ymmword_ptr<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().ymmword_ptr()
}

/// Creates a memory operand with a `ZMMWORD PTR` size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = zmmword_ptr(rax);
/// let _ = zmmword_ptr(0x1234_5678).fs();
/// let _ = zmmword_ptr(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn zmmword_ptr<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().zmmword_ptr()
}

/// Creates a broadcasted memory operand with no size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = bcst(rax);
/// let _ = bcst(0x1234_5678).fs();
/// let _ = bcst(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn bcst<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().bcst()
}

/// Creates a broadcasted memory operand with a `WORD BCST` size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = word_bcst(rax);
/// let _ = word_bcst(0x1234_5678).fs();
/// let _ = word_bcst(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn word_bcst<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().word_bcst()
}

/// Creates a broadcasted memory operand with a `DWORD BCST` size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = dword_bcst(rax);
/// let _ = dword_bcst(0x1234_5678).fs();
/// let _ = dword_bcst(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn dword_bcst<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().dword_bcst()
}

/// Creates a broadcasted memory operand with a `QWORD BCST` size hint
///
/// # Examples
///
/// ```
/// use iced_x86::code_asm::*;
///
/// let _ = qword_bcst(rax);
/// let _ = qword_bcst(0x1234_5678).fs();
/// let _ = qword_bcst(rdx * 4 + rcx - 123);
/// ```
#[must_use]
#[inline]
#[rustfmt::skip]
pub fn qword_bcst<M: Into<AsmMemoryOperand>>(mem: M) -> AsmMemoryOperand {
	mem.into().qword_bcst()
}
// GENERATOR-END: GlobalPtrMethods
