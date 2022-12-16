// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::info::test_utils::*;
use crate::test_utils::from_str_conv::*;
use crate::test_utils::section_file_reader::*;
use crate::*;
use alloc::string::String;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use std::collections::HashSet;

// GENERATOR-BEGIN: MiscSectionNames
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct MiscSectionNames;
#[allow(dead_code)]
impl MiscSectionNames {
	pub(crate) const JCC_SHORT: &'static str = "jcc-short";
	pub(crate) const JCC_NEAR: &'static str = "jcc-near";
	pub(crate) const JMP_SHORT: &'static str = "jmp-short";
	pub(crate) const JMP_NEAR: &'static str = "jmp-near";
	pub(crate) const JMP_FAR: &'static str = "jmp-far";
	pub(crate) const JMP_NEAR_INDIRECT: &'static str = "jmp-near-indirect";
	pub(crate) const JMP_FAR_INDIRECT: &'static str = "jmp-far-indirect";
	pub(crate) const CALL_NEAR: &'static str = "call-near";
	pub(crate) const CALL_FAR: &'static str = "call-far";
	pub(crate) const CALL_NEAR_INDIRECT: &'static str = "call-near-indirect";
	pub(crate) const CALL_FAR_INDIRECT: &'static str = "call-far-indirect";
	pub(crate) const JMPE_NEAR: &'static str = "jmpe-near";
	pub(crate) const JMPE_NEAR_INDIRECT: &'static str = "jmpe-near-indirect";
	pub(crate) const LOOP: &'static str = "loop";
	pub(crate) const JRCXZ: &'static str = "jrcxz";
	pub(crate) const XBEGIN: &'static str = "xbegin";
	pub(crate) const JKCC_SHORT: &'static str = "jkcc-short";
	pub(crate) const JKCC_NEAR: &'static str = "jkcc-near";
	pub(crate) const JMP_INFO: &'static str = "jmp-info";
	pub(crate) const JCC_SHORT_INFO: &'static str = "jcc-short-info";
	pub(crate) const JCC_NEAR_INFO: &'static str = "jcc-near-info";
	pub(crate) const SETCC_INFO: &'static str = "setcc-info";
	pub(crate) const CMOVCC_INFO: &'static str = "cmovcc-info";
	pub(crate) const CMPCCXADD_INFO: &'static str = "cmpccxadd-info";
	pub(crate) const LOOPCC_INFO: &'static str = "loopcc-info";
	pub(crate) const JKCC_SHORT_INFO: &'static str = "jkcc-short-info";
	pub(crate) const JKCC_NEAR_INFO: &'static str = "jkcc-near-info";
	pub(crate) const STRING_INSTRUCTION: &'static str = "string";
}
// GENERATOR-END: MiscSectionNames

struct MiscSectionNameIds;
impl MiscSectionNameIds {
	const JCC_SHORT: u32 = 0;
	const JCC_NEAR: u32 = 1;
	const JMP_SHORT: u32 = 2;
	const JMP_NEAR: u32 = 3;
	const JMP_FAR: u32 = 4;
	const JMP_NEAR_INDIRECT: u32 = 5;
	const JMP_FAR_INDIRECT: u32 = 6;
	const CALL_NEAR: u32 = 7;
	const CALL_FAR: u32 = 8;
	const CALL_NEAR_INDIRECT: u32 = 9;
	const CALL_FAR_INDIRECT: u32 = 10;
	const JMPE_NEAR: u32 = 11;
	const JMPE_NEAR_INDIRECT: u32 = 12;
	const LOOP: u32 = 13;
	const JRCXZ: u32 = 14;
	const XBEGIN: u32 = 15;
	const JMP_INFO: u32 = 16;
	const JCC_SHORT_INFO: u32 = 17;
	const JCC_NEAR_INFO: u32 = 18;
	const SETCC_INFO: u32 = 19;
	const CMOVCC_INFO: u32 = 20;
	const CMPCCXADD_INFO: u32 = 21;
	const LOOPCC_INFO: u32 = 22;
	const STRING_INSTRUCTION: u32 = 23;
	const JKCC_SHORT: u32 = 24;
	const JKCC_NEAR: u32 = 25;
	const JKCC_SHORT_INFO: u32 = 26;
	const JKCC_NEAR_INFO: u32 = 27;
}

pub(super) struct MiscTestsData {
	pub(super) jcc_short: HashSet<Code>,
	pub(super) jmp_near: HashSet<Code>,
	pub(super) jmp_far: HashSet<Code>,
	pub(super) jmp_short: HashSet<Code>,
	pub(super) jmp_near_indirect: HashSet<Code>,
	pub(super) jmp_far_indirect: HashSet<Code>,
	pub(super) jcc_near: HashSet<Code>,
	pub(super) call_far: HashSet<Code>,
	pub(super) call_near: HashSet<Code>,
	pub(super) call_near_indirect: HashSet<Code>,
	pub(super) call_far_indirect: HashSet<Code>,
	pub(super) jmpe_near: HashSet<Code>,
	pub(super) jmpe_near_indirect: HashSet<Code>,
	pub(super) loop_: HashSet<Code>,
	pub(super) jrcxz: HashSet<Code>,
	pub(super) xbegin: HashSet<Code>,
	pub(super) string: HashSet<Code>,
	pub(super) jkcc_short: HashSet<Code>,
	pub(super) jkcc_near: HashSet<Code>,
	pub(super) jmp_infos: Vec<(Code, Code)>,
	pub(super) jcc_short_infos: Vec<(Code, Code, Code, ConditionCode)>,
	pub(super) jcc_near_infos: Vec<(Code, Code, Code, ConditionCode)>,
	pub(super) jkcc_short_infos: Vec<(Code, Code, Code, ConditionCode)>,
	pub(super) jkcc_near_infos: Vec<(Code, Code, Code, ConditionCode)>,
	pub(super) setcc_infos: Vec<(Code, Code, ConditionCode)>,
	pub(super) cmovcc_infos: Vec<(Code, Code, ConditionCode)>,
	pub(super) cmpccxadd_infos: Vec<(Code, Code, ConditionCode)>,
	pub(super) loopcc_infos: Vec<(Code, Code, ConditionCode)>,
}

lazy_static! {
	pub(super) static ref MISC_TESTS_DATA: MiscTestsData = {
		let mut data_reader = MiscTestsDataReader::new();
		data_reader.read();
		data_reader.data
	};
}

struct MiscTestsDataReader {
	data: MiscTestsData,
}

impl MiscTestsDataReader {
	fn new() -> Self {
		Self {
			data: MiscTestsData {
				jcc_short: HashSet::new(),
				jmp_near: HashSet::new(),
				jmp_far: HashSet::new(),
				jmp_short: HashSet::new(),
				jmp_near_indirect: HashSet::new(),
				jmp_far_indirect: HashSet::new(),
				jcc_near: HashSet::new(),
				call_far: HashSet::new(),
				call_near: HashSet::new(),
				call_near_indirect: HashSet::new(),
				call_far_indirect: HashSet::new(),
				jmpe_near: HashSet::new(),
				jmpe_near_indirect: HashSet::new(),
				loop_: HashSet::new(),
				jrcxz: HashSet::new(),
				xbegin: HashSet::new(),
				string: HashSet::new(),
				jkcc_short: HashSet::new(),
				jkcc_near: HashSet::new(),
				jmp_infos: Vec::new(),
				jcc_short_infos: Vec::new(),
				jcc_near_infos: Vec::new(),
				jkcc_short_infos: Vec::new(),
				jkcc_near_infos: Vec::new(),
				setcc_infos: Vec::new(),
				cmovcc_infos: Vec::new(),
				cmpccxadd_infos: Vec::new(),
				loopcc_infos: Vec::new(),
			},
		}
	}

	fn read(&mut self) {
		let infos = &[
			(MiscSectionNames::JCC_SHORT, MiscSectionNameIds::JCC_SHORT),
			(MiscSectionNames::JMP_NEAR, MiscSectionNameIds::JMP_NEAR),
			(MiscSectionNames::JMP_FAR, MiscSectionNameIds::JMP_FAR),
			(MiscSectionNames::JMP_SHORT, MiscSectionNameIds::JMP_SHORT),
			(MiscSectionNames::JMP_NEAR_INDIRECT, MiscSectionNameIds::JMP_NEAR_INDIRECT),
			(MiscSectionNames::JMP_FAR_INDIRECT, MiscSectionNameIds::JMP_FAR_INDIRECT),
			(MiscSectionNames::JCC_NEAR, MiscSectionNameIds::JCC_NEAR),
			(MiscSectionNames::CALL_FAR, MiscSectionNameIds::CALL_FAR),
			(MiscSectionNames::CALL_NEAR, MiscSectionNameIds::CALL_NEAR),
			(MiscSectionNames::CALL_NEAR_INDIRECT, MiscSectionNameIds::CALL_NEAR_INDIRECT),
			(MiscSectionNames::CALL_FAR_INDIRECT, MiscSectionNameIds::CALL_FAR_INDIRECT),
			(MiscSectionNames::JMPE_NEAR, MiscSectionNameIds::JMPE_NEAR),
			(MiscSectionNames::JMPE_NEAR_INDIRECT, MiscSectionNameIds::JMPE_NEAR_INDIRECT),
			(MiscSectionNames::LOOP, MiscSectionNameIds::LOOP),
			(MiscSectionNames::JRCXZ, MiscSectionNameIds::JRCXZ),
			(MiscSectionNames::XBEGIN, MiscSectionNameIds::XBEGIN),
			(MiscSectionNames::JMP_INFO, MiscSectionNameIds::JMP_INFO),
			(MiscSectionNames::JCC_SHORT_INFO, MiscSectionNameIds::JCC_SHORT_INFO),
			(MiscSectionNames::JCC_NEAR_INFO, MiscSectionNameIds::JCC_NEAR_INFO),
			(MiscSectionNames::SETCC_INFO, MiscSectionNameIds::SETCC_INFO),
			(MiscSectionNames::CMOVCC_INFO, MiscSectionNameIds::CMOVCC_INFO),
			(MiscSectionNames::CMPCCXADD_INFO, MiscSectionNameIds::CMPCCXADD_INFO),
			(MiscSectionNames::LOOPCC_INFO, MiscSectionNameIds::LOOPCC_INFO),
			(MiscSectionNames::STRING_INSTRUCTION, MiscSectionNameIds::STRING_INSTRUCTION),
			(MiscSectionNames::JKCC_SHORT, MiscSectionNameIds::JKCC_SHORT),
			(MiscSectionNames::JKCC_NEAR, MiscSectionNameIds::JKCC_NEAR),
			(MiscSectionNames::JKCC_SHORT_INFO, MiscSectionNameIds::JKCC_SHORT_INFO),
			(MiscSectionNames::JKCC_NEAR_INFO, MiscSectionNameIds::JKCC_NEAR_INFO),
		];
		let mut reader = SectionFileReader::new(infos);
		let mut path = get_instr_info_unit_tests_dir();
		path.push("Misc.txt");
		reader.read(&path, self);
	}

	fn add_code(h: &mut HashSet<Code>, line: &str) -> Result<(), String> {
		if !is_ignored_code(line) {
			let _ = h.insert(to_code(line)?);
		}
		Ok(())
	}

	fn add_jmp_info(v: &mut Vec<(Code, Code)>, line: &str) -> Result<(), String> {
		const ELEMS: usize = 2;
		let elems: Vec<_> = line.split(',').collect();
		if elems.len() != ELEMS {
			return Err(format!("Expected {} elements, found {}", ELEMS, elems.len()));
		}
		if !is_ignored_code(elems[0]) && !is_ignored_code(elems[1]) {
			v.push((to_code(elems[0])?, to_code(elems[1])?));
		}
		Ok(())
	}

	fn add_jcc_info(v: &mut Vec<(Code, Code, Code, ConditionCode)>, line: &str) -> Result<(), String> {
		const ELEMS: usize = 4;
		let elems: Vec<_> = line.split(',').collect();
		if elems.len() != ELEMS {
			return Err(format!("Expected {} elements, found {}", ELEMS, elems.len()));
		}
		if !is_ignored_code(elems[0]) && !is_ignored_code(elems[1]) && !is_ignored_code(elems[2]) {
			v.push((to_code(elems[0])?, to_code(elems[1])?, to_code(elems[2])?, to_condition_code(elems[3])?));
		}
		Ok(())
	}

	fn add_instr_cc_info(v: &mut Vec<(Code, Code, ConditionCode)>, line: &str) -> Result<(), String> {
		const ELEMS: usize = 3;
		let elems: Vec<_> = line.split(',').collect();
		if elems.len() != ELEMS {
			return Err(format!("Expected {} elements, found {}", ELEMS, elems.len()));
		}
		if !is_ignored_code(elems[0]) && !is_ignored_code(elems[1]) {
			v.push((to_code(elems[0])?, to_code(elems[1])?, to_condition_code(elems[2])?));
		}
		Ok(())
	}
}

impl SectionFileLineHandler for MiscTestsDataReader {
	fn line(&mut self, id: u32, line: &str) -> Result<(), String> {
		match id {
			MiscSectionNameIds::JCC_SHORT => Self::add_code(&mut self.data.jcc_short, line),
			MiscSectionNameIds::JMP_NEAR => Self::add_code(&mut self.data.jmp_near, line),
			MiscSectionNameIds::JMP_FAR => Self::add_code(&mut self.data.jmp_far, line),
			MiscSectionNameIds::JMP_SHORT => Self::add_code(&mut self.data.jmp_short, line),
			MiscSectionNameIds::JMP_NEAR_INDIRECT => Self::add_code(&mut self.data.jmp_near_indirect, line),
			MiscSectionNameIds::JMP_FAR_INDIRECT => Self::add_code(&mut self.data.jmp_far_indirect, line),
			MiscSectionNameIds::JCC_NEAR => Self::add_code(&mut self.data.jcc_near, line),
			MiscSectionNameIds::CALL_FAR => Self::add_code(&mut self.data.call_far, line),
			MiscSectionNameIds::CALL_NEAR => Self::add_code(&mut self.data.call_near, line),
			MiscSectionNameIds::CALL_NEAR_INDIRECT => Self::add_code(&mut self.data.call_near_indirect, line),
			MiscSectionNameIds::CALL_FAR_INDIRECT => Self::add_code(&mut self.data.call_far_indirect, line),
			MiscSectionNameIds::JMPE_NEAR => Self::add_code(&mut self.data.jmpe_near, line),
			MiscSectionNameIds::JMPE_NEAR_INDIRECT => Self::add_code(&mut self.data.jmpe_near_indirect, line),
			MiscSectionNameIds::LOOP => Self::add_code(&mut self.data.loop_, line),
			MiscSectionNameIds::JRCXZ => Self::add_code(&mut self.data.jrcxz, line),
			MiscSectionNameIds::XBEGIN => Self::add_code(&mut self.data.xbegin, line),
			MiscSectionNameIds::JMP_INFO => Self::add_jmp_info(&mut self.data.jmp_infos, line),
			MiscSectionNameIds::JCC_SHORT_INFO => Self::add_jcc_info(&mut self.data.jcc_short_infos, line),
			MiscSectionNameIds::JCC_NEAR_INFO => Self::add_jcc_info(&mut self.data.jcc_near_infos, line),
			MiscSectionNameIds::SETCC_INFO => Self::add_instr_cc_info(&mut self.data.setcc_infos, line),
			MiscSectionNameIds::CMOVCC_INFO => Self::add_instr_cc_info(&mut self.data.cmovcc_infos, line),
			MiscSectionNameIds::CMPCCXADD_INFO => Self::add_instr_cc_info(&mut self.data.cmpccxadd_infos, line),
			MiscSectionNameIds::LOOPCC_INFO => Self::add_instr_cc_info(&mut self.data.loopcc_infos, line),
			MiscSectionNameIds::STRING_INSTRUCTION => Self::add_code(&mut self.data.string, line),
			MiscSectionNameIds::JKCC_SHORT => Self::add_code(&mut self.data.jkcc_short, line),
			MiscSectionNameIds::JKCC_NEAR => Self::add_code(&mut self.data.jkcc_near, line),
			MiscSectionNameIds::JKCC_SHORT_INFO => Self::add_jcc_info(&mut self.data.jkcc_short_infos, line),
			MiscSectionNameIds::JKCC_NEAR_INFO => Self::add_jcc_info(&mut self.data.jkcc_near_infos, line),
			_ => unreachable!(),
		}
	}
}
