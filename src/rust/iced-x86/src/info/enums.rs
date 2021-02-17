// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use super::super::*;
use core::fmt;

// GENERATOR-BEGIN: OpAccesses
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[rustfmt::skip]
pub(super) static OP_ACCESS_1: [OpAccess; 7] = [
	OpAccess::None,
	OpAccess::CondRead,
	OpAccess::NoMemAccess,
	OpAccess::Read,
	OpAccess::Read,
	OpAccess::ReadWrite,
	OpAccess::Write,
];
#[rustfmt::skip]
pub(super) static OP_ACCESS_2: [OpAccess; 3] = [
	OpAccess::None,
	OpAccess::Read,
	OpAccess::ReadWrite,
];
// GENERATOR-END: OpAccesses

// GENERATOR-BEGIN: InstrInfoConstants
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct InstrInfoConstants;
#[allow(dead_code)]
impl InstrInfoConstants {
	pub(crate) const OP_INFO0_COUNT: usize = 13;
	pub(crate) const OP_INFO1_COUNT: usize = 7;
	pub(crate) const OP_INFO2_COUNT: usize = 3;
	pub(crate) const OP_INFO3_COUNT: usize = 2;
	pub(crate) const OP_INFO4_COUNT: usize = 2;
	pub(crate) const RFLAGS_INFO_COUNT: usize = 79;
	pub(crate) const DEFAULT_USED_REGISTER_COLL_CAPACITY: usize = 10;
	pub(crate) const DEFAULT_USED_MEMORY_COLL_CAPACITY: usize = 8;
}
// GENERATOR-END: InstrInfoConstants

// GENERATOR-BEGIN: InfoFlags1
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[cfg(feature = "instr_info")]
pub(crate) struct InfoFlags1;
#[cfg(feature = "instr_info")]
#[allow(dead_code)]
impl InfoFlags1 {
	pub(crate) const OP_INFO0_SHIFT: u32 = 0x0000_0000;
	pub(crate) const OP_INFO0_MASK: u32 = 0x0000_000F;
	pub(crate) const OP_INFO1_SHIFT: u32 = 0x0000_0004;
	pub(crate) const OP_INFO1_MASK: u32 = 0x0000_0007;
	pub(crate) const OP_INFO2_SHIFT: u32 = 0x0000_0007;
	pub(crate) const OP_INFO2_MASK: u32 = 0x0000_0003;
	pub(crate) const OP_INFO3_SHIFT: u32 = 0x0000_0009;
	pub(crate) const OP_INFO3_MASK: u32 = 0x0000_0001;
	pub(crate) const OP_INFO4_SHIFT: u32 = 0x0000_000A;
	pub(crate) const OP_INFO4_MASK: u32 = 0x0000_0001;
	pub(crate) const RFLAGS_INFO_SHIFT: u32 = 0x0000_000D;
	pub(crate) const RFLAGS_INFO_MASK: u32 = 0x0000_007F;
	pub(crate) const IMPLIED_ACCESS_SHIFT: u32 = 0x0000_0014;
	pub(crate) const IMPLIED_ACCESS_MASK: u32 = 0x0000_00FF;
	pub(crate) const IGNORES_INDEX_VA: u32 = 0x2000_0000;
	pub(crate) const OP_MASK_READ_WRITE: u32 = 0x4000_0000;
	pub(crate) const IGNORES_SEGMENT: u32 = 0x8000_0000;
}
// GENERATOR-END: InfoFlags1

// GENERATOR-BEGIN: InfoFlags2
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[cfg(feature = "instr_info")]
pub(crate) struct InfoFlags2;
#[cfg(feature = "instr_info")]
#[allow(dead_code)]
impl InfoFlags2 {
	pub(crate) const ENCODING_SHIFT: u32 = 0x0000_0000;
	pub(crate) const ENCODING_MASK: u32 = 0x0000_0007;
	pub(crate) const SAVE_RESTORE: u32 = 0x0002_0000;
	pub(crate) const STACK_INSTRUCTION: u32 = 0x0004_0000;
	pub(crate) const PRIVILEGED: u32 = 0x0008_0000;
	pub(crate) const FLOW_CONTROL_SHIFT: u32 = 0x0000_0014;
	pub(crate) const FLOW_CONTROL_MASK: u32 = 0x0000_000F;
	pub(crate) const CPUID_FEATURE_INTERNAL_SHIFT: u32 = 0x0000_0018;
	pub(crate) const CPUID_FEATURE_INTERNAL_MASK: u32 = 0x0000_00FF;
}
// GENERATOR-END: InfoFlags2

// GENERATOR-BEGIN: OpInfo0
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum OpInfo0 {
	None,
	CondWrite,
	CondWrite32_ReadWrite64,
	NoMemAccess,
	Read,
	ReadCondWrite,
	ReadWrite,
	Write,
	WriteVmm,
	ReadWriteVmm,
	WriteForce,
	WriteMem_ReadWriteReg,
	WriteForceP1,
}
#[cfg(feature = "instr_info")]
#[rustfmt::skip]
static GEN_DEBUG_OP_INFO0: [&str; 13] = [
	"None",
	"CondWrite",
	"CondWrite32_ReadWrite64",
	"NoMemAccess",
	"Read",
	"ReadCondWrite",
	"ReadWrite",
	"Write",
	"WriteVmm",
	"ReadWriteVmm",
	"WriteForce",
	"WriteMem_ReadWriteReg",
	"WriteForceP1",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for OpInfo0 {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_OP_INFO0[*self as usize])
	}
}
#[cfg(feature = "instr_info")]
impl Default for OpInfo0 {
	#[must_use]
	#[inline]
	fn default() -> Self {
		OpInfo0::None
	}
}
// GENERATOR-END: OpInfo0

// GENERATOR-BEGIN: OpInfo1
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum OpInfo1 {
	None,
	CondRead,
	NoMemAccess,
	Read,
	ReadP3,
	ReadWrite,
	Write,
}
#[cfg(feature = "instr_info")]
#[rustfmt::skip]
static GEN_DEBUG_OP_INFO1: [&str; 7] = [
	"None",
	"CondRead",
	"NoMemAccess",
	"Read",
	"ReadP3",
	"ReadWrite",
	"Write",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for OpInfo1 {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_OP_INFO1[*self as usize])
	}
}
#[cfg(feature = "instr_info")]
impl Default for OpInfo1 {
	#[must_use]
	#[inline]
	fn default() -> Self {
		OpInfo1::None
	}
}
// GENERATOR-END: OpInfo1

// GENERATOR-BEGIN: OpInfo2
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum OpInfo2 {
	None,
	Read,
	ReadWrite,
}
#[cfg(feature = "instr_info")]
#[rustfmt::skip]
static GEN_DEBUG_OP_INFO2: [&str; 3] = [
	"None",
	"Read",
	"ReadWrite",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for OpInfo2 {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_OP_INFO2[*self as usize])
	}
}
#[cfg(feature = "instr_info")]
impl Default for OpInfo2 {
	#[must_use]
	#[inline]
	fn default() -> Self {
		OpInfo2::None
	}
}
// GENERATOR-END: OpInfo2

// GENERATOR-BEGIN: OpInfo3
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum OpInfo3 {
	None,
	Read,
}
#[cfg(feature = "instr_info")]
#[rustfmt::skip]
static GEN_DEBUG_OP_INFO3: [&str; 2] = [
	"None",
	"Read",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for OpInfo3 {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_OP_INFO3[*self as usize])
	}
}
#[cfg(feature = "instr_info")]
impl Default for OpInfo3 {
	#[must_use]
	#[inline]
	fn default() -> Self {
		OpInfo3::None
	}
}
// GENERATOR-END: OpInfo3

// GENERATOR-BEGIN: OpInfo4
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum OpInfo4 {
	None,
	Read,
}
#[cfg(feature = "instr_info")]
#[rustfmt::skip]
static GEN_DEBUG_OP_INFO4: [&str; 2] = [
	"None",
	"Read",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for OpInfo4 {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_OP_INFO4[*self as usize])
	}
}
#[cfg(feature = "instr_info")]
impl Default for OpInfo4 {
	#[must_use]
	#[inline]
	fn default() -> Self {
		OpInfo4::None
	}
}
// GENERATOR-END: OpInfo4

// GENERATOR-BEGIN: ImpliedAccess
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum ImpliedAccess {
	None,
	Shift_Ib_MASK1FMOD9,
	Shift_Ib_MASK1FMOD11,
	Shift_Ib_MASK1F,
	Shift_Ib_MASK3F,
	Clear_rflags,
	t_push1x2,
	t_push1x4,
	t_pop1x2,
	t_pop1x4,
	t_RWal,
	t_RWax,
	t_push1x8,
	t_pop1x8,
	t_pusha2,
	t_pusha4,
	t_popa2,
	t_popa4,
	t_arpl,
	t_ins,
	t_outs,
	t_lea,
	t_gpr16,
	t_poprm2,
	t_poprm4,
	t_poprm8,
	t_Ral_Wah,
	t_Rax_Weax,
	t_RWeax,
	t_Rax_Wdx,
	t_Reax_Wedx,
	t_Rrax_Wrdx,
	t_push2x2,
	t_push2x4,
	t_Rah,
	t_Wah,
	t_movs,
	t_cmps,
	t_stos,
	t_lods,
	t_scas,
	t_Wes,
	t_Wds,
	t_CWeax,
	t_enter2,
	t_enter4,
	t_enter8,
	t_leave2,
	t_leave4,
	t_leave8,
	t_pop2x2,
	t_pop2x4,
	t_pop2x8,
	b64_t_Wss_pop5x2_f_pop3x2,
	b64_t_Wss_pop5x4_f_pop3x4,
	t_Wss_pop5x8,
	t_Ral_Wax,
	t_Wal,
	t_RWst0,
	t_Rst0,
	t_Rst0_RWst1,
	t_RCWst0,
	t_Rst1_RWst0,
	t_Rst0_Rst1,
	t_Wst0TOst7_Wmm0TOmm7,
	t_Rst0TOst7_Rmm0TOmm7,
	t_RWcx,
	t_RWecx,
	t_RWrcx,
	t_Rcx,
	t_Recx,
	t_Rrcx,
	t_Wdx_RWax,
	t_Wedx_RWeax,
	t_Wrdx_RWrax,
	t_RWax_RWdx,
	t_RWeax_RWedx,
	t_RWrax_RWrdx,
	t_push2x8,
	t_Rcr0,
	t_RWcr0,
	t_gpr16_RWcr0,
	t_RCWeax_b64_t_CRrcx_CRrdx_CRrbx_CWrcx_CWrdx_CWrbx_f_CRecx_CRedx_CRebx_CRds_CWecx_CWedx_CWebx,
	t_RWeax_b64_t_CRrcx_CRrdx_CRrbx_f_CRecx_CRedx_CRebx_CRds,
	t_Rax_Recx_Redx_Rseg,
	t_Reax_Recx_Redx_Rseg,
	t_Recx_Redx_Rrax_Rseg,
	t_Reax_Recx,
	t_Recx_Weax_Wedx,
	t_Reax_Recx_Redx,
	t_Rax,
	t_Reax,
	t_Rrax,
	t_Rax_Wfs_Wgs,
	t_Reax_Wfs_Wgs,
	t_Rrax_Wfs_Wgs,
	t_Rax_Rfs_Rgs,
	t_Reax_Rfs_Rgs,
	t_Rrax_Rfs_Rgs,
	t_Reax_Wcr0_Wdr6_Wdr7_WesTOgs_Wcr2TOcr4_Wdr0TOdr3_b64_t_WraxTOr15_f_WeaxTOedi,
	t_Rax_Recx,
	t_Recx_Rrax,
	t_Weax_Wecx_Wedx,
	t_Reax_Recx_CRebx,
	t_Rax_Rseg,
	t_Reax_Rseg,
	t_Rrax_Rseg,
	t_Wecx_b64_t_Wr11,
	t_Redi_Res,
	t_Recx_Wcs_Wss_b64_t_Rr11d,
	t_Rr11d_Rrcx_Wcs_Wss,
	t_Weax_Wedx,
	t_Wesp,
	t_Recx_Redx_Wesp_Wcs_Wss,
	t_Rrcx_Rrdx_Wrsp_Wcs_Wss,
	t_zrrm,
	t_zrrrm,
	b64_t_RWxmm0TOxmm15_f_RWxmm0TOxmm7,
	b64_t_Wzmm0TOzmm15_f_Wzmm0TOzmm7,
	t_CRecx_Wecx_Wedx_Webx_RWeax,
	t_CRmem_CRsi_CReax_CRes_CWeax_CWedx_RCWecx,
	t_CRmem_CReax_CResi_CRes_CWeax_CWedx_RCWecx,
	t_CRmem_CReax_CRrsi_CRes_CWeax_CWedx_RCWrcx,
	t_CRmem_CRmem_CWmem_CRsi_CRdi_CRes_CWsi_RCWax_RCWcx,
	t_CRmem_CRmem_CWmem_CResi_CRedi_CRes_CWesi_RCWeax_RCWecx,
	t_CRmem_CRmem_CWmem_CRrsi_CRrdi_CRes_CWrsi_RCWrax_RCWrcx,
	t_Rcl_Rax,
	t_Rcl_Reax,
	t_xstore2,
	t_xstore4,
	t_xstore8,
	t_CRmem_CRmem_CRmem_CWmem_CRdx_CRbx_CRsi_CRdi_CRes_CWsi_CWdi_RCWcx,
	t_CRmem_CRmem_CRmem_CWmem_CRedx_CRebx_CResi_CRedi_CRes_CWesi_CWedi_RCWecx,
	t_CRmem_CRmem_CRmem_CWmem_CRrdx_CRrbx_CRrsi_CRrdi_CRes_CWrsi_CWrdi_RCWrcx,
	t_CRmem_CRmem_CRmem_CRmem_CWmem_CWmem_CRax_CRdx_CRbx_CRsi_CRdi_CRes_CWsi_CWdi_RCWcx,
	t_CRmem_CRmem_CRmem_CRmem_CWmem_CWmem_CReax_CRedx_CRebx_CResi_CRedi_CRes_CWesi_CWedi_RCWecx,
	t_CRmem_CRmem_CRmem_CRmem_CWmem_CWmem_CRrax_CRrdx_CRrbx_CRrsi_CRrdi_CRes_CWrsi_CWrdi_RCWrcx,
	t_RCWal,
	t_RCWax,
	t_RCWeax,
	t_Reax_Redx,
	t_gpr8,
	t_gpr32_Reax_Redx,
	t_Rmem_Rseg,
	t_RCWrax,
	t_Wss,
	t_Wfs,
	t_Wgs,
	t_CRecx_CRebx_RCWeax_RCWedx,
	t_CRrcx_CRrbx_RCWrax_RCWrdx,
	t_Wmem_RarDI_Rseg,
	t_Rxmm0,
	t_Redx,
	t_Rrdx,
	t_Wmem_Res,
	t_Reax_Redx_Wxmm0,
	t_Rrax_Rrdx_Wxmm0,
	t_Reax_Redx_Wecx,
	t_Rrax_Rrdx_Wecx,
	t_Wxmm0,
	t_Wecx,
	t_Rmem_Rds,
	t_Rrcx_Rrdx_RWrax,
	t_Rmem_Rrcx_Rseg_RWrax,
	t_RWrax,
	t_Rax_Recx_Redx_Weax,
	t_Recx_Redx_RWeax,
	t_Recx_Redx_RWrax,
	t_Rax_Recx_Redx,
	t_Recx_Redx_Rrax,
	t_Wtmm0TOtmm7,
	t_Reax_Rebx,
	t_Rebx_Weax,
	t_emmiW,
	t_emmiRW,
	t_emmiR,
	t_CRrcx_CRrdx_CRr8_CRr9_RWrax,
	t_RWxmm0TOxmm7,
	t_Reax_Rxmm0,
	t_Wxmm1_Wxmm2_RWxmm0_Wxmm4TOxmm6,
	t_RWxmm0_RWxmm1_Wxmm2TOxmm6,
	t_pop3x8,
	t_CRmem_CRmem_CWmem_CRbx_CRsi_CRdi_CRes_CWsi_RCWax_RCWcx,
	t_CRmem_CRmem_CWmem_CRebx_CResi_CRedi_CRes_CWesi_RCWeax_RCWecx,
	t_CRmem_CRmem_CWmem_CRrbx_CRrsi_CRrdi_CRes_CWrsi_RCWrax_RCWrcx,
	t_CRmem_CRmem_CWmem_CRax_CRbx_CRsi_CRdi_CRes_CWsi_CWdi_RCWcx,
	t_CRmem_CRmem_CWmem_CReax_CRebx_CResi_CRedi_CRes_CWesi_CWedi_RCWecx,
	t_CRmem_CRmem_CWmem_CRrax_CRrbx_CRrsi_CRrdi_CRes_CWrsi_CWrdi_RCWrcx,
}
#[cfg(feature = "instr_info")]
#[rustfmt::skip]
static GEN_DEBUG_IMPLIED_ACCESS: [&str; 188] = [
	"None",
	"Shift_Ib_MASK1FMOD9",
	"Shift_Ib_MASK1FMOD11",
	"Shift_Ib_MASK1F",
	"Shift_Ib_MASK3F",
	"Clear_rflags",
	"t_push1x2",
	"t_push1x4",
	"t_pop1x2",
	"t_pop1x4",
	"t_RWal",
	"t_RWax",
	"t_push1x8",
	"t_pop1x8",
	"t_pusha2",
	"t_pusha4",
	"t_popa2",
	"t_popa4",
	"t_arpl",
	"t_ins",
	"t_outs",
	"t_lea",
	"t_gpr16",
	"t_poprm2",
	"t_poprm4",
	"t_poprm8",
	"t_Ral_Wah",
	"t_Rax_Weax",
	"t_RWeax",
	"t_Rax_Wdx",
	"t_Reax_Wedx",
	"t_Rrax_Wrdx",
	"t_push2x2",
	"t_push2x4",
	"t_Rah",
	"t_Wah",
	"t_movs",
	"t_cmps",
	"t_stos",
	"t_lods",
	"t_scas",
	"t_Wes",
	"t_Wds",
	"t_CWeax",
	"t_enter2",
	"t_enter4",
	"t_enter8",
	"t_leave2",
	"t_leave4",
	"t_leave8",
	"t_pop2x2",
	"t_pop2x4",
	"t_pop2x8",
	"b64_t_Wss_pop5x2_f_pop3x2",
	"b64_t_Wss_pop5x4_f_pop3x4",
	"t_Wss_pop5x8",
	"t_Ral_Wax",
	"t_Wal",
	"t_RWst0",
	"t_Rst0",
	"t_Rst0_RWst1",
	"t_RCWst0",
	"t_Rst1_RWst0",
	"t_Rst0_Rst1",
	"t_Wst0TOst7_Wmm0TOmm7",
	"t_Rst0TOst7_Rmm0TOmm7",
	"t_RWcx",
	"t_RWecx",
	"t_RWrcx",
	"t_Rcx",
	"t_Recx",
	"t_Rrcx",
	"t_Wdx_RWax",
	"t_Wedx_RWeax",
	"t_Wrdx_RWrax",
	"t_RWax_RWdx",
	"t_RWeax_RWedx",
	"t_RWrax_RWrdx",
	"t_push2x8",
	"t_Rcr0",
	"t_RWcr0",
	"t_gpr16_RWcr0",
	"t_RCWeax_b64_t_CRrcx_CRrdx_CRrbx_CWrcx_CWrdx_CWrbx_f_CRecx_CRedx_CRebx_CRds_CWecx_CWedx_CWebx",
	"t_RWeax_b64_t_CRrcx_CRrdx_CRrbx_f_CRecx_CRedx_CRebx_CRds",
	"t_Rax_Recx_Redx_Rseg",
	"t_Reax_Recx_Redx_Rseg",
	"t_Recx_Redx_Rrax_Rseg",
	"t_Reax_Recx",
	"t_Recx_Weax_Wedx",
	"t_Reax_Recx_Redx",
	"t_Rax",
	"t_Reax",
	"t_Rrax",
	"t_Rax_Wfs_Wgs",
	"t_Reax_Wfs_Wgs",
	"t_Rrax_Wfs_Wgs",
	"t_Rax_Rfs_Rgs",
	"t_Reax_Rfs_Rgs",
	"t_Rrax_Rfs_Rgs",
	"t_Reax_Wcr0_Wdr6_Wdr7_WesTOgs_Wcr2TOcr4_Wdr0TOdr3_b64_t_WraxTOr15_f_WeaxTOedi",
	"t_Rax_Recx",
	"t_Recx_Rrax",
	"t_Weax_Wecx_Wedx",
	"t_Reax_Recx_CRebx",
	"t_Rax_Rseg",
	"t_Reax_Rseg",
	"t_Rrax_Rseg",
	"t_Wecx_b64_t_Wr11",
	"t_Redi_Res",
	"t_Recx_Wcs_Wss_b64_t_Rr11d",
	"t_Rr11d_Rrcx_Wcs_Wss",
	"t_Weax_Wedx",
	"t_Wesp",
	"t_Recx_Redx_Wesp_Wcs_Wss",
	"t_Rrcx_Rrdx_Wrsp_Wcs_Wss",
	"t_zrrm",
	"t_zrrrm",
	"b64_t_RWxmm0TOxmm15_f_RWxmm0TOxmm7",
	"b64_t_Wzmm0TOzmm15_f_Wzmm0TOzmm7",
	"t_CRecx_Wecx_Wedx_Webx_RWeax",
	"t_CRmem_CRsi_CReax_CRes_CWeax_CWedx_RCWecx",
	"t_CRmem_CReax_CResi_CRes_CWeax_CWedx_RCWecx",
	"t_CRmem_CReax_CRrsi_CRes_CWeax_CWedx_RCWrcx",
	"t_CRmem_CRmem_CWmem_CRsi_CRdi_CRes_CWsi_RCWax_RCWcx",
	"t_CRmem_CRmem_CWmem_CResi_CRedi_CRes_CWesi_RCWeax_RCWecx",
	"t_CRmem_CRmem_CWmem_CRrsi_CRrdi_CRes_CWrsi_RCWrax_RCWrcx",
	"t_Rcl_Rax",
	"t_Rcl_Reax",
	"t_xstore2",
	"t_xstore4",
	"t_xstore8",
	"t_CRmem_CRmem_CRmem_CWmem_CRdx_CRbx_CRsi_CRdi_CRes_CWsi_CWdi_RCWcx",
	"t_CRmem_CRmem_CRmem_CWmem_CRedx_CRebx_CResi_CRedi_CRes_CWesi_CWedi_RCWecx",
	"t_CRmem_CRmem_CRmem_CWmem_CRrdx_CRrbx_CRrsi_CRrdi_CRes_CWrsi_CWrdi_RCWrcx",
	"t_CRmem_CRmem_CRmem_CRmem_CWmem_CWmem_CRax_CRdx_CRbx_CRsi_CRdi_CRes_CWsi_CWdi_RCWcx",
	"t_CRmem_CRmem_CRmem_CRmem_CWmem_CWmem_CReax_CRedx_CRebx_CResi_CRedi_CRes_CWesi_CWedi_RCWecx",
	"t_CRmem_CRmem_CRmem_CRmem_CWmem_CWmem_CRrax_CRrdx_CRrbx_CRrsi_CRrdi_CRes_CWrsi_CWrdi_RCWrcx",
	"t_RCWal",
	"t_RCWax",
	"t_RCWeax",
	"t_Reax_Redx",
	"t_gpr8",
	"t_gpr32_Reax_Redx",
	"t_Rmem_Rseg",
	"t_RCWrax",
	"t_Wss",
	"t_Wfs",
	"t_Wgs",
	"t_CRecx_CRebx_RCWeax_RCWedx",
	"t_CRrcx_CRrbx_RCWrax_RCWrdx",
	"t_Wmem_RarDI_Rseg",
	"t_Rxmm0",
	"t_Redx",
	"t_Rrdx",
	"t_Wmem_Res",
	"t_Reax_Redx_Wxmm0",
	"t_Rrax_Rrdx_Wxmm0",
	"t_Reax_Redx_Wecx",
	"t_Rrax_Rrdx_Wecx",
	"t_Wxmm0",
	"t_Wecx",
	"t_Rmem_Rds",
	"t_Rrcx_Rrdx_RWrax",
	"t_Rmem_Rrcx_Rseg_RWrax",
	"t_RWrax",
	"t_Rax_Recx_Redx_Weax",
	"t_Recx_Redx_RWeax",
	"t_Recx_Redx_RWrax",
	"t_Rax_Recx_Redx",
	"t_Recx_Redx_Rrax",
	"t_Wtmm0TOtmm7",
	"t_Reax_Rebx",
	"t_Rebx_Weax",
	"t_emmiW",
	"t_emmiRW",
	"t_emmiR",
	"t_CRrcx_CRrdx_CRr8_CRr9_RWrax",
	"t_RWxmm0TOxmm7",
	"t_Reax_Rxmm0",
	"t_Wxmm1_Wxmm2_RWxmm0_Wxmm4TOxmm6",
	"t_RWxmm0_RWxmm1_Wxmm2TOxmm6",
	"t_pop3x8",
	"t_CRmem_CRmem_CWmem_CRbx_CRsi_CRdi_CRes_CWsi_RCWax_RCWcx",
	"t_CRmem_CRmem_CWmem_CRebx_CResi_CRedi_CRes_CWesi_RCWeax_RCWecx",
	"t_CRmem_CRmem_CWmem_CRrbx_CRrsi_CRrdi_CRes_CWrsi_RCWrax_RCWrcx",
	"t_CRmem_CRmem_CWmem_CRax_CRbx_CRsi_CRdi_CRes_CWsi_CWdi_RCWcx",
	"t_CRmem_CRmem_CWmem_CReax_CRebx_CResi_CRedi_CRes_CWesi_CWedi_RCWecx",
	"t_CRmem_CRmem_CWmem_CRrax_CRrbx_CRrsi_CRrdi_CRes_CWrsi_CWrdi_RCWrcx",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for ImpliedAccess {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_IMPLIED_ACCESS[*self as usize])
	}
}
#[cfg(feature = "instr_info")]
impl Default for ImpliedAccess {
	#[must_use]
	#[inline]
	fn default() -> Self {
		ImpliedAccess::None
	}
}
// GENERATOR-END: ImpliedAccess

// GENERATOR-BEGIN: RflagsInfo
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum RflagsInfo {
	None,
	C_0123,
	C_1_U_023,
	C_A,
	C_acopsz,
	C_acopszidA,
	C_acos_S_pz,
	C_c,
	C_cos_S_pz_U_a,
	C_d,
	C_i,
	C_u,
	R_0123_C_0123,
	R_0123_U_0123,
	R_a_W_ac_U_opsz,
	R_ac_W_acpsz_U_o,
	R_acopszid,
	R_acopszidA,
	R_acopszidA_W_acopszidA,
	R_acpsz,
	R_c,
	R_c_C_1_U_023,
	R_c_W_acopsz,
	R_c_W_c,
	R_c_W_c_U_o,
	R_c_W_co,
	R_cz,
	R_cz_C_1_U_023,
	R_d,
	R_d_W_acopsz,
	R_o,
	R_o_W_o,
	R_os,
	R_osz,
	R_p,
	R_p_C_1_U_023,
	R_s,
	R_u_W_c_C_aopsz,
	R_z,
	R_z_C_1_U_023,
	S_A,
	S_c,
	S_d,
	S_i,
	S_u,
	U_0123,
	U_acopsz,
	W_0123,
	W_023_C_1,
	W_12_U_03,
	W_1_U_023,
	W_acopsz,
	W_acopszdA_S_u,
	W_acopszid,
	W_acopszidA,
	W_acpsz,
	W_aopsz,
	W_c,
	W_c_C_aopsz,
	W_c_U_aops,
	W_c_U_o,
	W_co,
	W_co_U_apsz,
	W_copsz_U_a,
	W_cosz_C_ap,
	W_cpsz_U_ao,
	W_cpz_C_aos,
	W_cpz_C_aos1,
	W_cs_C_oz_U_ap,
	W_csz_C_o_U_ap,
	W_cz_C_aops,
	W_cz_U_aops,
	W_psz_C_co_U_a,
	W_psz_U_aco,
	W_sz_C_co_U_ap,
	W_z,
	W_z_C_acops,
	W_z_C_co_U_aps,
	W_z_U_acops,
}
#[cfg(feature = "instr_info")]
#[rustfmt::skip]
static GEN_DEBUG_RFLAGS_INFO: [&str; 79] = [
	"None",
	"C_0123",
	"C_1_U_023",
	"C_A",
	"C_acopsz",
	"C_acopszidA",
	"C_acos_S_pz",
	"C_c",
	"C_cos_S_pz_U_a",
	"C_d",
	"C_i",
	"C_u",
	"R_0123_C_0123",
	"R_0123_U_0123",
	"R_a_W_ac_U_opsz",
	"R_ac_W_acpsz_U_o",
	"R_acopszid",
	"R_acopszidA",
	"R_acopszidA_W_acopszidA",
	"R_acpsz",
	"R_c",
	"R_c_C_1_U_023",
	"R_c_W_acopsz",
	"R_c_W_c",
	"R_c_W_c_U_o",
	"R_c_W_co",
	"R_cz",
	"R_cz_C_1_U_023",
	"R_d",
	"R_d_W_acopsz",
	"R_o",
	"R_o_W_o",
	"R_os",
	"R_osz",
	"R_p",
	"R_p_C_1_U_023",
	"R_s",
	"R_u_W_c_C_aopsz",
	"R_z",
	"R_z_C_1_U_023",
	"S_A",
	"S_c",
	"S_d",
	"S_i",
	"S_u",
	"U_0123",
	"U_acopsz",
	"W_0123",
	"W_023_C_1",
	"W_12_U_03",
	"W_1_U_023",
	"W_acopsz",
	"W_acopszdA_S_u",
	"W_acopszid",
	"W_acopszidA",
	"W_acpsz",
	"W_aopsz",
	"W_c",
	"W_c_C_aopsz",
	"W_c_U_aops",
	"W_c_U_o",
	"W_co",
	"W_co_U_apsz",
	"W_copsz_U_a",
	"W_cosz_C_ap",
	"W_cpsz_U_ao",
	"W_cpz_C_aos",
	"W_cpz_C_aos1",
	"W_cs_C_oz_U_ap",
	"W_csz_C_o_U_ap",
	"W_cz_C_aops",
	"W_cz_U_aops",
	"W_psz_C_co_U_a",
	"W_psz_U_aco",
	"W_sz_C_co_U_ap",
	"W_z",
	"W_z_C_acops",
	"W_z_C_co_U_aps",
	"W_z_U_acops",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for RflagsInfo {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_RFLAGS_INFO[*self as usize])
	}
}
#[cfg(feature = "instr_info")]
impl Default for RflagsInfo {
	#[must_use]
	#[inline]
	fn default() -> Self {
		RflagsInfo::None
	}
}
// GENERATOR-END: RflagsInfo

// GENERATOR-BEGIN: CpuidFeatureInternal
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum CpuidFeatureInternal {
	INTEL8086,
	INTEL8086_ONLY,
	INTEL186,
	INTEL286,
	INTEL286_ONLY,
	INTEL386,
	INTEL386_ONLY,
	INTEL386_A0_ONLY,
	INTEL486,
	INTEL486_A_ONLY,
	UMOV,
	IA64,
	X64,
	ADX,
	AES,
	AVX,
	AES_and_AVX,
	AVX2,
	AVX512_4FMAPS,
	AVX512_4VNNIW,
	AVX512_BITALG,
	AVX512_IFMA,
	AVX512_VBMI,
	AVX512_VBMI2,
	AVX512_VNNI,
	AVX512_VPOPCNTDQ,
	AVX512BW,
	AVX512CD,
	AVX512DQ,
	AVX512ER,
	AVX512F,
	AVX512F_and_AVX512_BF16,
	AVX512F_and_AVX512_VP2INTERSECT,
	AVX512PF,
	AVX512VL_and_AVX512_BF16,
	AVX512VL_and_AVX512_BITALG,
	AVX512VL_and_AVX512_IFMA,
	AVX512VL_and_AVX512_VBMI,
	AVX512VL_and_AVX512_VBMI2,
	AVX512VL_and_AVX512_VNNI,
	AVX512VL_and_AVX512_VP2INTERSECT,
	AVX512VL_and_AVX512_VPOPCNTDQ,
	AVX512VL_and_AVX512BW,
	AVX512VL_and_AVX512CD,
	AVX512VL_and_AVX512DQ,
	AVX512VL_and_AVX512F,
	BMI1,
	BMI2,
	CET_IBT,
	CET_SS,
	CL1INVMB,
	CLDEMOTE,
	CLFLUSHOPT,
	CLFSH,
	CLWB,
	CLZERO,
	CMOV,
	CMPXCHG16B,
	CPUID,
	CX8,
	D3NOW,
	D3NOWEXT,
	OSS,
	ENQCMD,
	F16C,
	FMA,
	FMA4,
	FPU,
	FPU_and_CMOV,
	FPU287,
	FPU287XL_ONLY,
	FPU387,
	FPU387SL_ONLY,
	FSGSBASE,
	FXSR,
	CYRIX_D3NOW,
	GFNI,
	AVX_and_GFNI,
	AVX512F_and_GFNI,
	AVX512VL_and_GFNI,
	HLE_or_RTM,
	INVPCID,
	LWP,
	LZCNT,
	MCOMMIT,
	MMX,
	MONITOR,
	MONITORX,
	MOVBE,
	MOVDIR64B,
	MOVDIRI,
	MPX,
	MSR,
	MULTIBYTENOP,
	PADLOCK_ACE,
	PADLOCK_PHE,
	PADLOCK_PMM,
	PADLOCK_RNG,
	PAUSE,
	PCLMULQDQ,
	PCLMULQDQ_and_AVX,
	PCOMMIT,
	PCONFIG,
	PKU,
	POPCNT,
	PREFETCHW,
	PREFETCHWT1,
	PTWRITE,
	RDPID,
	RDPMC,
	RDPRU,
	RDRAND,
	RDSEED,
	RDTSCP,
	RTM,
	SEP,
	SGX1,
	SHA,
	SKINIT_or_SVM,
	SMAP,
	SMX,
	SSE,
	SSE2,
	SSE3,
	FPU_and_SSE3,
	SSE4_1,
	SSE4_2,
	SSE4A,
	SSSE3,
	SVM,
	SEV_ES,
	SYSCALL,
	TBM,
	TSC,
	VAES,
	AVX512F_and_VAES,
	AVX512VL_and_VAES,
	VMX,
	VMX_and_INVEPT,
	VMX_and_INVVPID,
	VPCLMULQDQ,
	AVX512F_and_VPCLMULQDQ,
	AVX512VL_and_VPCLMULQDQ,
	WAITPKG,
	WBNOINVD,
	XOP,
	XSAVE,
	XSAVEC,
	XSAVEOPT,
	XSAVES,
	SEV_SNP,
	SERIALIZE,
	TSXLDTRK,
	INVLPGB,
	AMX_BF16,
	AMX_TILE,
	AMX_INT8,
	CYRIX_FPU,
	CYRIX_SMM,
	CYRIX_SMINT,
	CYRIX_SMINT_0F7E,
	CYRIX_SHR,
	CYRIX_DDI,
	CYRIX_EMMI,
	CYRIX_DMI,
	CENTAUR_AIS,
	MOV_TR,
	SMM,
	TDX,
	KL,
	AESKLE,
	AESKLE_and_WIDE_KL,
	UINTR,
	HRESET,
	AVX_VNNI,
	PADLOCK_GMI,
}
#[cfg(feature = "instr_info")]
#[rustfmt::skip]
static GEN_DEBUG_CPUID_FEATURE_INTERNAL: [&str; 176] = [
	"INTEL8086",
	"INTEL8086_ONLY",
	"INTEL186",
	"INTEL286",
	"INTEL286_ONLY",
	"INTEL386",
	"INTEL386_ONLY",
	"INTEL386_A0_ONLY",
	"INTEL486",
	"INTEL486_A_ONLY",
	"UMOV",
	"IA64",
	"X64",
	"ADX",
	"AES",
	"AVX",
	"AES_and_AVX",
	"AVX2",
	"AVX512_4FMAPS",
	"AVX512_4VNNIW",
	"AVX512_BITALG",
	"AVX512_IFMA",
	"AVX512_VBMI",
	"AVX512_VBMI2",
	"AVX512_VNNI",
	"AVX512_VPOPCNTDQ",
	"AVX512BW",
	"AVX512CD",
	"AVX512DQ",
	"AVX512ER",
	"AVX512F",
	"AVX512F_and_AVX512_BF16",
	"AVX512F_and_AVX512_VP2INTERSECT",
	"AVX512PF",
	"AVX512VL_and_AVX512_BF16",
	"AVX512VL_and_AVX512_BITALG",
	"AVX512VL_and_AVX512_IFMA",
	"AVX512VL_and_AVX512_VBMI",
	"AVX512VL_and_AVX512_VBMI2",
	"AVX512VL_and_AVX512_VNNI",
	"AVX512VL_and_AVX512_VP2INTERSECT",
	"AVX512VL_and_AVX512_VPOPCNTDQ",
	"AVX512VL_and_AVX512BW",
	"AVX512VL_and_AVX512CD",
	"AVX512VL_and_AVX512DQ",
	"AVX512VL_and_AVX512F",
	"BMI1",
	"BMI2",
	"CET_IBT",
	"CET_SS",
	"CL1INVMB",
	"CLDEMOTE",
	"CLFLUSHOPT",
	"CLFSH",
	"CLWB",
	"CLZERO",
	"CMOV",
	"CMPXCHG16B",
	"CPUID",
	"CX8",
	"D3NOW",
	"D3NOWEXT",
	"OSS",
	"ENQCMD",
	"F16C",
	"FMA",
	"FMA4",
	"FPU",
	"FPU_and_CMOV",
	"FPU287",
	"FPU287XL_ONLY",
	"FPU387",
	"FPU387SL_ONLY",
	"FSGSBASE",
	"FXSR",
	"CYRIX_D3NOW",
	"GFNI",
	"AVX_and_GFNI",
	"AVX512F_and_GFNI",
	"AVX512VL_and_GFNI",
	"HLE_or_RTM",
	"INVPCID",
	"LWP",
	"LZCNT",
	"MCOMMIT",
	"MMX",
	"MONITOR",
	"MONITORX",
	"MOVBE",
	"MOVDIR64B",
	"MOVDIRI",
	"MPX",
	"MSR",
	"MULTIBYTENOP",
	"PADLOCK_ACE",
	"PADLOCK_PHE",
	"PADLOCK_PMM",
	"PADLOCK_RNG",
	"PAUSE",
	"PCLMULQDQ",
	"PCLMULQDQ_and_AVX",
	"PCOMMIT",
	"PCONFIG",
	"PKU",
	"POPCNT",
	"PREFETCHW",
	"PREFETCHWT1",
	"PTWRITE",
	"RDPID",
	"RDPMC",
	"RDPRU",
	"RDRAND",
	"RDSEED",
	"RDTSCP",
	"RTM",
	"SEP",
	"SGX1",
	"SHA",
	"SKINIT_or_SVM",
	"SMAP",
	"SMX",
	"SSE",
	"SSE2",
	"SSE3",
	"FPU_and_SSE3",
	"SSE4_1",
	"SSE4_2",
	"SSE4A",
	"SSSE3",
	"SVM",
	"SEV_ES",
	"SYSCALL",
	"TBM",
	"TSC",
	"VAES",
	"AVX512F_and_VAES",
	"AVX512VL_and_VAES",
	"VMX",
	"VMX_and_INVEPT",
	"VMX_and_INVVPID",
	"VPCLMULQDQ",
	"AVX512F_and_VPCLMULQDQ",
	"AVX512VL_and_VPCLMULQDQ",
	"WAITPKG",
	"WBNOINVD",
	"XOP",
	"XSAVE",
	"XSAVEC",
	"XSAVEOPT",
	"XSAVES",
	"SEV_SNP",
	"SERIALIZE",
	"TSXLDTRK",
	"INVLPGB",
	"AMX_BF16",
	"AMX_TILE",
	"AMX_INT8",
	"CYRIX_FPU",
	"CYRIX_SMM",
	"CYRIX_SMINT",
	"CYRIX_SMINT_0F7E",
	"CYRIX_SHR",
	"CYRIX_DDI",
	"CYRIX_EMMI",
	"CYRIX_DMI",
	"CENTAUR_AIS",
	"MOV_TR",
	"SMM",
	"TDX",
	"KL",
	"AESKLE",
	"AESKLE_and_WIDE_KL",
	"UINTR",
	"HRESET",
	"AVX_VNNI",
	"PADLOCK_GMI",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for CpuidFeatureInternal {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CPUID_FEATURE_INTERNAL[*self as usize])
	}
}
#[cfg(feature = "instr_info")]
impl Default for CpuidFeatureInternal {
	#[must_use]
	#[inline]
	fn default() -> Self {
		CpuidFeatureInternal::INTEL8086
	}
}
// GENERATOR-END: CpuidFeatureInternal
