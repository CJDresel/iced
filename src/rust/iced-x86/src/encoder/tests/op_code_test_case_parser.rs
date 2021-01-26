// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

#![allow(unused_results)]

use super::super::super::iced_constants::IcedConstants;
use super::super::super::test_utils::from_str_conv::*;
use super::super::super::*;
use super::op_code_test_case::*;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::iter::IntoIterator;
use core::u32;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::path::Path;

pub(super) struct OpCodeInfoTestParser {
	filename: String,
	lines: Lines<BufReader<File>>,
}

impl OpCodeInfoTestParser {
	pub(super) fn new(filename: &Path) -> Self {
		let display_filename = filename.display().to_string();
		let file = File::open(filename).unwrap_or_else(|_| panic!("Couldn't open file {}", display_filename));
		let lines = BufReader::new(file).lines();
		Self { filename: display_filename, lines }
	}
}

impl IntoIterator for OpCodeInfoTestParser {
	type Item = OpCodeInfoTestCase;
	type IntoIter = IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		// GENERATOR-BEGIN: EncodingKindDict
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut to_encoding_kind: HashMap<&'static str, EncodingKind> = HashMap::with_capacity(5);
		to_encoding_kind.insert("legacy", EncodingKind::Legacy);
		to_encoding_kind.insert("VEX", EncodingKind::VEX);
		to_encoding_kind.insert("EVEX", EncodingKind::EVEX);
		to_encoding_kind.insert("XOP", EncodingKind::XOP);
		to_encoding_kind.insert("3DNow!", EncodingKind::D3NOW);
		// GENERATOR-END: EncodingKindDict

		// GENERATOR-BEGIN: MandatoryPrefixDict
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut to_mandatory_prefix: HashMap<&'static str, MandatoryPrefix> = HashMap::with_capacity(5);
		to_mandatory_prefix.insert("", MandatoryPrefix::None);
		to_mandatory_prefix.insert("NP", MandatoryPrefix::PNP);
		to_mandatory_prefix.insert("66", MandatoryPrefix::P66);
		to_mandatory_prefix.insert("F3", MandatoryPrefix::PF3);
		to_mandatory_prefix.insert("F2", MandatoryPrefix::PF2);
		// GENERATOR-END: MandatoryPrefixDict

		// GENERATOR-BEGIN: OpCodeTableKindDict
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut to_op_code_table_kind: HashMap<&'static str, OpCodeTableKind> = HashMap::with_capacity(7);
		to_op_code_table_kind.insert("legacy", OpCodeTableKind::Normal);
		to_op_code_table_kind.insert("0F", OpCodeTableKind::T0F);
		to_op_code_table_kind.insert("0F38", OpCodeTableKind::T0F38);
		to_op_code_table_kind.insert("0F3A", OpCodeTableKind::T0F3A);
		to_op_code_table_kind.insert("X8", OpCodeTableKind::XOP8);
		to_op_code_table_kind.insert("X9", OpCodeTableKind::XOP9);
		to_op_code_table_kind.insert("XA", OpCodeTableKind::XOPA);
		// GENERATOR-END: OpCodeTableKindDict

		IntoIter { filename: self.filename, lines: self.lines, line_number: 0, to_encoding_kind, to_mandatory_prefix, to_op_code_table_kind }
	}
}

pub(super) struct IntoIter {
	filename: String,
	lines: Lines<BufReader<File>>,
	line_number: u32,
	to_encoding_kind: HashMap<&'static str, EncodingKind>,
	to_mandatory_prefix: HashMap<&'static str, MandatoryPrefix>,
	to_op_code_table_kind: HashMap<&'static str, OpCodeTableKind>,
}

impl Iterator for IntoIter {
	type Item = OpCodeInfoTestCase;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.lines.next() {
				None => return None,
				Some(info) => {
					let result = match info {
						Ok(line) => {
							self.line_number += 1;
							if line.is_empty() || line.starts_with('#') {
								continue;
							}
							self.read_next_test_case(line, self.line_number)
						}
						Err(err) => Err(err.to_string()),
					};
					match result {
						Ok(tc) => {
							if let Some(tc) = tc {
								return Some(tc);
							} else {
								continue;
							}
						}
						Err(err) => panic!("Error parsing OpCodeInfo test case file '{}', line {}: {}", self.filename, self.line_number, err),
					}
				}
			}
		}
	}
}

// GENERATOR-BEGIN: OpCodeInfoKeys
// ⚠️This was generated by GENERATOR!🦹‍♂️
lazy_static! {
	pub(super) static ref TO_OP_CODE_INFO_KEYS: HashMap<&'static str, u32> = {
		let mut h = HashMap::with_capacity(5);
		h.insert("g", OpCodeInfoKeys::GROUP_INDEX);
		h.insert("rmg", OpCodeInfoKeys::RM_GROUP_INDEX);
		h.insert("op", OpCodeInfoKeys::OP_CODE_OPERAND_KIND);
		h.insert("tt", OpCodeInfoKeys::TUPLE_TYPE);
		h.insert("dec-opt", OpCodeInfoKeys::DECODER_OPTION);
		h
	};
}

pub(crate) struct OpCodeInfoKeys;
#[allow(dead_code)]
impl OpCodeInfoKeys {
	pub(crate) const GROUP_INDEX: u32 = 0;
	pub(crate) const RM_GROUP_INDEX: u32 = 1;
	pub(crate) const OP_CODE_OPERAND_KIND: u32 = 2;
	pub(crate) const TUPLE_TYPE: u32 = 3;
	pub(crate) const DECODER_OPTION: u32 = 4;
}
// GENERATOR-END: OpCodeInfoKeys

// GENERATOR-BEGIN: OpCodeInfoFlags
// ⚠️This was generated by GENERATOR!🦹‍♂️
lazy_static! {
	pub(super) static ref TO_OP_CODE_INFO_FLAGS: HashMap<&'static str, u32> = {
		let mut h = HashMap::with_capacity(95);
		h.insert("no-instr", OpCodeInfoFlags::NO_INSTRUCTION);
		h.insert("16", OpCodeInfoFlags::BIT16);
		h.insert("32", OpCodeInfoFlags::BIT32);
		h.insert("64", OpCodeInfoFlags::BIT64);
		h.insert("fwait", OpCodeInfoFlags::FWAIT);
		h.insert("o16", OpCodeInfoFlags::OPERAND_SIZE16);
		h.insert("o32", OpCodeInfoFlags::OPERAND_SIZE32);
		h.insert("o64", OpCodeInfoFlags::OPERAND_SIZE64);
		h.insert("a16", OpCodeInfoFlags::ADDRESS_SIZE16);
		h.insert("a32", OpCodeInfoFlags::ADDRESS_SIZE32);
		h.insert("a64", OpCodeInfoFlags::ADDRESS_SIZE64);
		h.insert("LIG", OpCodeInfoFlags::LIG);
		h.insert("L0", OpCodeInfoFlags::L0);
		h.insert("L1", OpCodeInfoFlags::L1);
		h.insert("L128", OpCodeInfoFlags::L128);
		h.insert("L256", OpCodeInfoFlags::L256);
		h.insert("L512", OpCodeInfoFlags::L512);
		h.insert("WIG", OpCodeInfoFlags::WIG);
		h.insert("WIG32", OpCodeInfoFlags::WIG32);
		h.insert("W0", OpCodeInfoFlags::W0);
		h.insert("W1", OpCodeInfoFlags::W1);
		h.insert("b", OpCodeInfoFlags::BROADCAST);
		h.insert("er", OpCodeInfoFlags::ROUNDING_CONTROL);
		h.insert("sae", OpCodeInfoFlags::SUPPRESS_ALL_EXCEPTIONS);
		h.insert("k", OpCodeInfoFlags::OP_MASK_REGISTER);
		h.insert("knz", OpCodeInfoFlags::REQUIRE_OP_MASK_REGISTER);
		h.insert("z", OpCodeInfoFlags::ZEROING_MASKING);
		h.insert("lock", OpCodeInfoFlags::LOCK);
		h.insert("xacquire", OpCodeInfoFlags::XACQUIRE);
		h.insert("xrelease", OpCodeInfoFlags::XRELEASE);
		h.insert("rep", OpCodeInfoFlags::REP);
		h.insert("repe", OpCodeInfoFlags::REPE);
		h.insert("repne", OpCodeInfoFlags::REPNE);
		h.insert("bnd", OpCodeInfoFlags::BND);
		h.insert("ht", OpCodeInfoFlags::HINT_TAKEN);
		h.insert("notrack", OpCodeInfoFlags::NOTRACK);
		h.insert("ignore-er", OpCodeInfoFlags::IGNORES_ROUNDING_CONTROL);
		h.insert("amd-lock-reg-bit", OpCodeInfoFlags::AMD_LOCK_REG_BIT);
		h.insert("do64", OpCodeInfoFlags::DEFAULT_OP_SIZE64);
		h.insert("fo64", OpCodeInfoFlags::FORCE_OP_SIZE64);
		h.insert("intel-fo64", OpCodeInfoFlags::INTEL_FORCE_OP_SIZE64);
		h.insert("cpl0", OpCodeInfoFlags::CPL0);
		h.insert("cpl1", OpCodeInfoFlags::CPL1);
		h.insert("cpl2", OpCodeInfoFlags::CPL2);
		h.insert("cpl3", OpCodeInfoFlags::CPL3);
		h.insert("io", OpCodeInfoFlags::INPUT_OUTPUT);
		h.insert("nop", OpCodeInfoFlags::NOP);
		h.insert("res-nop", OpCodeInfoFlags::RESERVED_NOP);
		h.insert("intel-serialize", OpCodeInfoFlags::SERIALIZING_INTEL);
		h.insert("amd-serialize", OpCodeInfoFlags::SERIALIZING_AMD);
		h.insert("may-require-cpl0", OpCodeInfoFlags::MAY_REQUIRE_CPL0);
		h.insert("cet-tracked", OpCodeInfoFlags::CET_TRACKED);
		h.insert("nt", OpCodeInfoFlags::NON_TEMPORAL);
		h.insert("nowait", OpCodeInfoFlags::FPU_NO_WAIT);
		h.insert("ignores-mod", OpCodeInfoFlags::IGNORES_MOD_BITS);
		h.insert("no66", OpCodeInfoFlags::NO66);
		h.insert("nfx", OpCodeInfoFlags::NFX);
		h.insert("unique-reg-nums", OpCodeInfoFlags::REQUIRES_UNIQUE_REG_NUMS);
		h.insert("priv", OpCodeInfoFlags::PRIVILEGED);
		h.insert("save-restore", OpCodeInfoFlags::SAVE_RESTORE);
		h.insert("stack", OpCodeInfoFlags::STACK_INSTRUCTION);
		h.insert("ignore-seg", OpCodeInfoFlags::IGNORES_SEGMENT);
		h.insert("krw", OpCodeInfoFlags::OP_MASK_READ_WRITE);
		h.insert("rm", OpCodeInfoFlags::REAL_MODE);
		h.insert("pm", OpCodeInfoFlags::PROTECTED_MODE);
		h.insert("v86", OpCodeInfoFlags::VIRTUAL8086_MODE);
		h.insert("cm", OpCodeInfoFlags::COMPATIBILITY_MODE);
		h.insert("lm", OpCodeInfoFlags::LONG_MODE);
		h.insert("outside-smm", OpCodeInfoFlags::USE_OUTSIDE_SMM);
		h.insert("in-smm", OpCodeInfoFlags::USE_IN_SMM);
		h.insert("outside-sgx", OpCodeInfoFlags::USE_OUTSIDE_ENCLAVE_SGX);
		h.insert("in-sgx1", OpCodeInfoFlags::USE_IN_ENCLAVE_SGX1);
		h.insert("in-sgx2", OpCodeInfoFlags::USE_IN_ENCLAVE_SGX2);
		h.insert("outside-vmx-op", OpCodeInfoFlags::USE_OUTSIDE_VMX_OP);
		h.insert("in-vmx-root-op", OpCodeInfoFlags::USE_IN_VMX_ROOT_OP);
		h.insert("in-vmx-non-root-op", OpCodeInfoFlags::USE_IN_VMX_NON_ROOT_OP);
		h.insert("outside-seam", OpCodeInfoFlags::USE_OUTSIDE_SEAM);
		h.insert("in-seam", OpCodeInfoFlags::USE_IN_SEAM);
		h.insert("tdx-non-root-gen-ud", OpCodeInfoFlags::TDX_NON_ROOT_GEN_UD);
		h.insert("td-non-root-gen-ve", OpCodeInfoFlags::TDX_NON_ROOT_GEN_VE);
		h.insert("tdx-non-root-may-gen-ex", OpCodeInfoFlags::TDX_NON_ROOT_MAY_GEN_EX);
		h.insert("intel-vm-exit", OpCodeInfoFlags::INTEL_VM_EXIT);
		h.insert("intel-may-vm-exit", OpCodeInfoFlags::INTEL_MAY_VM_EXIT);
		h.insert("intel-smm-vm-exit", OpCodeInfoFlags::INTEL_SMM_VM_EXIT);
		h.insert("amd-vm-exit", OpCodeInfoFlags::AMD_VM_EXIT);
		h.insert("amd-may-vm-exit", OpCodeInfoFlags::AMD_MAY_VM_EXIT);
		h.insert("tsx-abort", OpCodeInfoFlags::TSX_ABORT);
		h.insert("tsx-impl-abort", OpCodeInfoFlags::TSX_IMPL_ABORT);
		h.insert("tsx-may-abort", OpCodeInfoFlags::TSX_MAY_ABORT);
		h.insert("intel16", OpCodeInfoFlags::INTEL_DECODER16);
		h.insert("intel32", OpCodeInfoFlags::INTEL_DECODER32);
		h.insert("intel64", OpCodeInfoFlags::INTEL_DECODER64);
		h.insert("amd16", OpCodeInfoFlags::AMD_DECODER16);
		h.insert("amd32", OpCodeInfoFlags::AMD_DECODER32);
		h.insert("amd64", OpCodeInfoFlags::AMD_DECODER64);
		h
	};
}

pub(crate) struct OpCodeInfoFlags;
#[allow(dead_code)]
impl OpCodeInfoFlags {
	pub(crate) const NO_INSTRUCTION: u32 = 0;
	pub(crate) const BIT16: u32 = 1;
	pub(crate) const BIT32: u32 = 2;
	pub(crate) const BIT64: u32 = 3;
	pub(crate) const FWAIT: u32 = 4;
	pub(crate) const OPERAND_SIZE16: u32 = 5;
	pub(crate) const OPERAND_SIZE32: u32 = 6;
	pub(crate) const OPERAND_SIZE64: u32 = 7;
	pub(crate) const ADDRESS_SIZE16: u32 = 8;
	pub(crate) const ADDRESS_SIZE32: u32 = 9;
	pub(crate) const ADDRESS_SIZE64: u32 = 10;
	pub(crate) const LIG: u32 = 11;
	pub(crate) const L0: u32 = 12;
	pub(crate) const L1: u32 = 13;
	pub(crate) const L128: u32 = 14;
	pub(crate) const L256: u32 = 15;
	pub(crate) const L512: u32 = 16;
	pub(crate) const WIG: u32 = 17;
	pub(crate) const WIG32: u32 = 18;
	pub(crate) const W0: u32 = 19;
	pub(crate) const W1: u32 = 20;
	pub(crate) const BROADCAST: u32 = 21;
	pub(crate) const ROUNDING_CONTROL: u32 = 22;
	pub(crate) const SUPPRESS_ALL_EXCEPTIONS: u32 = 23;
	pub(crate) const OP_MASK_REGISTER: u32 = 24;
	pub(crate) const REQUIRE_OP_MASK_REGISTER: u32 = 25;
	pub(crate) const ZEROING_MASKING: u32 = 26;
	pub(crate) const LOCK: u32 = 27;
	pub(crate) const XACQUIRE: u32 = 28;
	pub(crate) const XRELEASE: u32 = 29;
	pub(crate) const REP: u32 = 30;
	pub(crate) const REPE: u32 = 31;
	pub(crate) const REPNE: u32 = 32;
	pub(crate) const BND: u32 = 33;
	pub(crate) const HINT_TAKEN: u32 = 34;
	pub(crate) const NOTRACK: u32 = 35;
	pub(crate) const IGNORES_ROUNDING_CONTROL: u32 = 36;
	pub(crate) const AMD_LOCK_REG_BIT: u32 = 37;
	pub(crate) const DEFAULT_OP_SIZE64: u32 = 38;
	pub(crate) const FORCE_OP_SIZE64: u32 = 39;
	pub(crate) const INTEL_FORCE_OP_SIZE64: u32 = 40;
	pub(crate) const CPL0: u32 = 41;
	pub(crate) const CPL1: u32 = 42;
	pub(crate) const CPL2: u32 = 43;
	pub(crate) const CPL3: u32 = 44;
	pub(crate) const INPUT_OUTPUT: u32 = 45;
	pub(crate) const NOP: u32 = 46;
	pub(crate) const RESERVED_NOP: u32 = 47;
	pub(crate) const SERIALIZING_INTEL: u32 = 48;
	pub(crate) const SERIALIZING_AMD: u32 = 49;
	pub(crate) const MAY_REQUIRE_CPL0: u32 = 50;
	pub(crate) const CET_TRACKED: u32 = 51;
	pub(crate) const NON_TEMPORAL: u32 = 52;
	pub(crate) const FPU_NO_WAIT: u32 = 53;
	pub(crate) const IGNORES_MOD_BITS: u32 = 54;
	pub(crate) const NO66: u32 = 55;
	pub(crate) const NFX: u32 = 56;
	pub(crate) const REQUIRES_UNIQUE_REG_NUMS: u32 = 57;
	pub(crate) const PRIVILEGED: u32 = 58;
	pub(crate) const SAVE_RESTORE: u32 = 59;
	pub(crate) const STACK_INSTRUCTION: u32 = 60;
	pub(crate) const IGNORES_SEGMENT: u32 = 61;
	pub(crate) const OP_MASK_READ_WRITE: u32 = 62;
	pub(crate) const REAL_MODE: u32 = 63;
	pub(crate) const PROTECTED_MODE: u32 = 64;
	pub(crate) const VIRTUAL8086_MODE: u32 = 65;
	pub(crate) const COMPATIBILITY_MODE: u32 = 66;
	pub(crate) const LONG_MODE: u32 = 67;
	pub(crate) const USE_OUTSIDE_SMM: u32 = 68;
	pub(crate) const USE_IN_SMM: u32 = 69;
	pub(crate) const USE_OUTSIDE_ENCLAVE_SGX: u32 = 70;
	pub(crate) const USE_IN_ENCLAVE_SGX1: u32 = 71;
	pub(crate) const USE_IN_ENCLAVE_SGX2: u32 = 72;
	pub(crate) const USE_OUTSIDE_VMX_OP: u32 = 73;
	pub(crate) const USE_IN_VMX_ROOT_OP: u32 = 74;
	pub(crate) const USE_IN_VMX_NON_ROOT_OP: u32 = 75;
	pub(crate) const USE_OUTSIDE_SEAM: u32 = 76;
	pub(crate) const USE_IN_SEAM: u32 = 77;
	pub(crate) const TDX_NON_ROOT_GEN_UD: u32 = 78;
	pub(crate) const TDX_NON_ROOT_GEN_VE: u32 = 79;
	pub(crate) const TDX_NON_ROOT_MAY_GEN_EX: u32 = 80;
	pub(crate) const INTEL_VM_EXIT: u32 = 81;
	pub(crate) const INTEL_MAY_VM_EXIT: u32 = 82;
	pub(crate) const INTEL_SMM_VM_EXIT: u32 = 83;
	pub(crate) const AMD_VM_EXIT: u32 = 84;
	pub(crate) const AMD_MAY_VM_EXIT: u32 = 85;
	pub(crate) const TSX_ABORT: u32 = 86;
	pub(crate) const TSX_IMPL_ABORT: u32 = 87;
	pub(crate) const TSX_MAY_ABORT: u32 = 88;
	pub(crate) const INTEL_DECODER16: u32 = 89;
	pub(crate) const INTEL_DECODER32: u32 = 90;
	pub(crate) const INTEL_DECODER64: u32 = 91;
	pub(crate) const AMD_DECODER16: u32 = 92;
	pub(crate) const AMD_DECODER32: u32 = 93;
	pub(crate) const AMD_DECODER64: u32 = 94;
}
// GENERATOR-END: OpCodeInfoFlags

impl IntoIter {
	#[allow(clippy::len_zero)]
	fn read_next_test_case(&self, line: String, line_number: u32) -> Result<Option<OpCodeInfoTestCase>, String> {
		let elems: Vec<_> = line.split(',').collect();
		if elems.len() != 11 {
			return Err(format!("Invalid number of commas: {}", elems.len() - 1));
		}

		let mut tc = OpCodeInfoTestCase::default();
		tc.line_number = line_number;
		tc.is_instruction = true;
		tc.group_index = -1;
		tc.rm_group_index = -1;

		if is_ignored_code(elems[0].trim()) {
			return Ok(None);
		}
		tc.code = to_code(elems[0].trim())?;
		tc.mnemonic = to_mnemonic(elems[1].trim())?;
		tc.memory_size = to_memory_size(elems[2].trim())?;
		tc.broadcast_memory_size = to_memory_size(elems[3].trim())?;
		tc.encoding = self.to_encoding(elems[4].trim())?;
		tc.mandatory_prefix = self.to_mandatory_prefix(elems[5].trim())?;
		tc.table = self.to_table(elems[6].trim())?;
		let (op_code, op_code_len) = Self::to_op_code(elems[7].trim())?;
		tc.op_code = op_code;
		tc.op_code_len = op_code_len;
		tc.op_code_string = String::from(elems[8].trim());
		tc.instruction_string = elems[9].trim().replace('|', ",");

		let mut got_vector_length = false;
		let mut got_w = false;
		for part in elems[10].split_whitespace() {
			let mut key = part.trim();
			if key.is_empty() {
				continue;
			}
			let kv_parts: Vec<_> = key.splitn(2, '=').collect();
			if kv_parts.len() == 2 {
				key = kv_parts[0];
				let value = kv_parts[1];
				match *(*TO_OP_CODE_INFO_KEYS).get(key).unwrap_or(&u32::MAX) {
					OpCodeInfoKeys::GROUP_INDEX => {
						tc.group_index = to_i32(value)?;
						if tc.group_index > 7 {
							return Err(format!("Invalid group index: {}", value));
						}
						tc.is_group = true;
					}

					OpCodeInfoKeys::RM_GROUP_INDEX => {
						tc.rm_group_index = to_i32(value)?;
						if tc.rm_group_index > 7 {
							return Err(format!("Invalid rm group index: {}", value));
						}
						tc.is_rm_group = true;
					}

					OpCodeInfoKeys::OP_CODE_OPERAND_KIND => {
						let op_parts: Vec<_> = value.split(';').collect();
						tc.op_count = op_parts.len() as u32;
						if op_parts.len() >= 1 {
							tc.op_kinds[0] = to_op_code_operand_kind(op_parts[0])?;
						}
						if op_parts.len() >= 2 {
							tc.op_kinds[1] = to_op_code_operand_kind(op_parts[1])?;
						}
						if op_parts.len() >= 3 {
							tc.op_kinds[2] = to_op_code_operand_kind(op_parts[2])?;
						}
						if op_parts.len() >= 4 {
							tc.op_kinds[3] = to_op_code_operand_kind(op_parts[3])?;
						}
						if op_parts.len() >= 5 {
							tc.op_kinds[4] = to_op_code_operand_kind(op_parts[4])?;
						}
						const_assert_eq!(5, IcedConstants::MAX_OP_COUNT);
						if op_parts.len() >= 6 {
							return Err(format!("Invalid number of operands: '{}'", value));
						}
					}

					OpCodeInfoKeys::TUPLE_TYPE => tc.tuple_type = to_tuple_type(value.trim())?,
					OpCodeInfoKeys::DECODER_OPTION => tc.decoder_option = to_decoder_options(value.trim())?,

					_ => return Err(format!("Invalid key: '{}'", key)),
				}
			} else {
				assert_eq!(1, kv_parts.len());
				match *(*TO_OP_CODE_INFO_FLAGS).get(key).unwrap_or(&u32::MAX) {
					OpCodeInfoFlags::NO_INSTRUCTION => tc.is_instruction = false,
					OpCodeInfoFlags::BIT16 => tc.mode16 = true,
					OpCodeInfoFlags::BIT32 => tc.mode32 = true,
					OpCodeInfoFlags::BIT64 => tc.mode64 = true,
					OpCodeInfoFlags::FWAIT => tc.fwait = true,
					OpCodeInfoFlags::OPERAND_SIZE16 => tc.operand_size = 16,
					OpCodeInfoFlags::OPERAND_SIZE32 => tc.operand_size = 32,
					OpCodeInfoFlags::OPERAND_SIZE64 => tc.operand_size = 64,
					OpCodeInfoFlags::ADDRESS_SIZE16 => tc.address_size = 16,
					OpCodeInfoFlags::ADDRESS_SIZE32 => tc.address_size = 32,
					OpCodeInfoFlags::ADDRESS_SIZE64 => tc.address_size = 64,

					OpCodeInfoFlags::LIG => {
						tc.is_lig = true;
						got_vector_length = true;
					}

					OpCodeInfoFlags::L0 => {
						tc.l = 0;
						got_vector_length = true;
					}

					OpCodeInfoFlags::L1 => {
						tc.l = 1;
						got_vector_length = true;
					}

					OpCodeInfoFlags::L128 => {
						tc.l = 0;
						got_vector_length = true;
					}

					OpCodeInfoFlags::L256 => {
						tc.l = 1;
						got_vector_length = true;
					}

					OpCodeInfoFlags::L512 => {
						tc.l = 2;
						got_vector_length = true;
					}

					OpCodeInfoFlags::WIG => {
						tc.is_wig = true;
						got_w = true;
					}

					OpCodeInfoFlags::WIG32 => {
						tc.w = 0;
						tc.is_wig32 = true;
						got_w = true;
					}

					OpCodeInfoFlags::W0 => {
						tc.w = 0;
						got_w = true;
					}

					OpCodeInfoFlags::W1 => {
						tc.w = 1;
						got_w = true;
					}

					OpCodeInfoFlags::BROADCAST => tc.can_broadcast = true,
					OpCodeInfoFlags::ROUNDING_CONTROL => tc.can_use_rounding_control = true,
					OpCodeInfoFlags::SUPPRESS_ALL_EXCEPTIONS => tc.can_suppress_all_exceptions = true,
					OpCodeInfoFlags::OP_MASK_REGISTER => tc.can_use_op_mask_register = true,

					OpCodeInfoFlags::REQUIRE_OP_MASK_REGISTER => {
						tc.can_use_op_mask_register = true;
						tc.require_op_mask_register = true;
					}

					OpCodeInfoFlags::ZEROING_MASKING => tc.can_use_zeroing_masking = true,
					OpCodeInfoFlags::LOCK => tc.can_use_lock_prefix = true,
					OpCodeInfoFlags::XACQUIRE => tc.can_use_xacquire_prefix = true,
					OpCodeInfoFlags::XRELEASE => tc.can_use_xrelease_prefix = true,
					OpCodeInfoFlags::REP | OpCodeInfoFlags::REPE => tc.can_use_rep_prefix = true,
					OpCodeInfoFlags::REPNE => tc.can_use_repne_prefix = true,
					OpCodeInfoFlags::BND => tc.can_use_bnd_prefix = true,
					OpCodeInfoFlags::HINT_TAKEN => tc.can_use_hint_taken_prefix = true,
					OpCodeInfoFlags::NOTRACK => tc.can_use_notrack_prefix = true,
					OpCodeInfoFlags::IGNORES_ROUNDING_CONTROL => tc.ignores_rounding_control = true,
					OpCodeInfoFlags::AMD_LOCK_REG_BIT => tc.amd_lock_reg_bit = true,
					OpCodeInfoFlags::DEFAULT_OP_SIZE64 => tc.default_op_size64 = true,
					OpCodeInfoFlags::FORCE_OP_SIZE64 => tc.force_op_size64 = true,
					OpCodeInfoFlags::INTEL_FORCE_OP_SIZE64 => tc.intel_force_op_size64 = true,
					OpCodeInfoFlags::CPL0 => tc.cpl0 = true,
					OpCodeInfoFlags::CPL1 => tc.cpl1 = true,
					OpCodeInfoFlags::CPL2 => tc.cpl2 = true,
					OpCodeInfoFlags::CPL3 => tc.cpl3 = true,
					OpCodeInfoFlags::INPUT_OUTPUT => tc.is_input_output = true,
					OpCodeInfoFlags::NOP => tc.is_nop = true,
					OpCodeInfoFlags::RESERVED_NOP => tc.is_reserved_nop = true,
					OpCodeInfoFlags::SERIALIZING_INTEL => tc.is_serializing_intel = true,
					OpCodeInfoFlags::SERIALIZING_AMD => tc.is_serializing_amd = true,
					OpCodeInfoFlags::MAY_REQUIRE_CPL0 => tc.may_require_cpl0 = true,
					OpCodeInfoFlags::CET_TRACKED => tc.is_cet_tracked = true,
					OpCodeInfoFlags::NON_TEMPORAL => tc.is_non_temporal = true,
					OpCodeInfoFlags::FPU_NO_WAIT => tc.is_fpu_no_wait = true,
					OpCodeInfoFlags::IGNORES_MOD_BITS => tc.ignores_mod_bits = true,
					OpCodeInfoFlags::NO66 => tc.no66 = true,
					OpCodeInfoFlags::NFX => tc.nfx = true,
					OpCodeInfoFlags::REQUIRES_UNIQUE_REG_NUMS => tc.requires_unique_reg_nums = true,
					OpCodeInfoFlags::PRIVILEGED => tc.is_privileged = true,
					OpCodeInfoFlags::SAVE_RESTORE => tc.is_save_restore = true,
					OpCodeInfoFlags::STACK_INSTRUCTION => tc.is_stack_instruction = true,
					OpCodeInfoFlags::IGNORES_SEGMENT => tc.ignores_segment = true,
					OpCodeInfoFlags::OP_MASK_READ_WRITE => tc.is_op_mask_read_write = true,
					OpCodeInfoFlags::REAL_MODE => tc.real_mode = true,
					OpCodeInfoFlags::PROTECTED_MODE => tc.protected_mode = true,
					OpCodeInfoFlags::VIRTUAL8086_MODE => tc.virtual8086_mode = true,
					OpCodeInfoFlags::COMPATIBILITY_MODE => tc.compatibility_mode = true,
					OpCodeInfoFlags::LONG_MODE => tc.long_mode = true,
					OpCodeInfoFlags::USE_OUTSIDE_SMM => tc.use_outside_smm = true,
					OpCodeInfoFlags::USE_IN_SMM => tc.use_in_smm = true,
					OpCodeInfoFlags::USE_OUTSIDE_ENCLAVE_SGX => tc.use_outside_enclave_sgx = true,
					OpCodeInfoFlags::USE_IN_ENCLAVE_SGX1 => tc.use_in_enclave_sgx1 = true,
					OpCodeInfoFlags::USE_IN_ENCLAVE_SGX2 => tc.use_in_enclave_sgx2 = true,
					OpCodeInfoFlags::USE_OUTSIDE_VMX_OP => tc.use_outside_vmx_op = true,
					OpCodeInfoFlags::USE_IN_VMX_ROOT_OP => tc.use_in_vmx_root_op = true,
					OpCodeInfoFlags::USE_IN_VMX_NON_ROOT_OP => tc.use_in_vmx_non_root_op = true,
					OpCodeInfoFlags::USE_OUTSIDE_SEAM => tc.use_outside_seam = true,
					OpCodeInfoFlags::USE_IN_SEAM => tc.use_in_seam = true,
					OpCodeInfoFlags::TDX_NON_ROOT_GEN_UD => tc.tdx_non_root_gen_ud = true,
					OpCodeInfoFlags::TDX_NON_ROOT_GEN_VE => tc.tdx_non_root_gen_ve = true,
					OpCodeInfoFlags::TDX_NON_ROOT_MAY_GEN_EX => tc.tdx_non_root_may_gen_ex = true,
					OpCodeInfoFlags::INTEL_VM_EXIT => tc.intel_vm_exit = true,
					OpCodeInfoFlags::INTEL_MAY_VM_EXIT => tc.intel_may_vm_exit = true,
					OpCodeInfoFlags::INTEL_SMM_VM_EXIT => tc.intel_smm_vm_exit = true,
					OpCodeInfoFlags::AMD_VM_EXIT => tc.amd_vm_exit = true,
					OpCodeInfoFlags::AMD_MAY_VM_EXIT => tc.amd_may_vm_exit = true,
					OpCodeInfoFlags::TSX_ABORT => tc.tsx_abort = true,
					OpCodeInfoFlags::TSX_IMPL_ABORT => tc.tsx_impl_abort = true,
					OpCodeInfoFlags::TSX_MAY_ABORT => tc.tsx_may_abort = true,
					OpCodeInfoFlags::INTEL_DECODER16 => tc.intel_decoder16 = true,
					OpCodeInfoFlags::INTEL_DECODER32 => tc.intel_decoder32 = true,
					OpCodeInfoFlags::INTEL_DECODER64 => tc.intel_decoder64 = true,
					OpCodeInfoFlags::AMD_DECODER16 => tc.amd_decoder16 = true,
					OpCodeInfoFlags::AMD_DECODER32 => tc.amd_decoder32 = true,
					OpCodeInfoFlags::AMD_DECODER64 => tc.amd_decoder64 = true,

					_ => return Err(format!("Invalid key: '{}'", key)),
				}
			}
		}
		match tc.encoding {
			EncodingKind::Legacy | EncodingKind::D3NOW => {}
			EncodingKind::VEX | EncodingKind::EVEX | EncodingKind::XOP => {
				if !got_vector_length {
					return Err(String::from("Missing vector length: L0/L1/L128/L256/L512/LIG"));
				}
				if !got_w {
					return Err(String::from("Missing W bit: W0/W1/WIG/WIG32"));
				}
			}
		}

		Ok(Some(tc))
	}

	fn to_encoding(&self, value: &str) -> Result<EncodingKind, String> {
		self.to_encoding_kind.get(value).cloned().ok_or_else(|| format!("Invalid encoding value: '{}'", value))
	}

	fn to_mandatory_prefix(&self, value: &str) -> Result<MandatoryPrefix, String> {
		self.to_mandatory_prefix.get(value).cloned().ok_or_else(|| format!("Invalid mandatory prefix value: '{}'", value))
	}

	fn to_table(&self, value: &str) -> Result<OpCodeTableKind, String> {
		self.to_op_code_table_kind.get(value).cloned().ok_or_else(|| format!("Invalid opcode table value: '{}'", value))
	}

	fn to_op_code(value: &str) -> Result<(u32, u32), String> {
		match u32::from_str_radix(value, 16) {
			Ok(op_code) => Ok((op_code, (value.len() as u32) / 2)),
			Err(_) => Err(format!("Invalid opcode: '{}'", value)),
		}
	}
}
