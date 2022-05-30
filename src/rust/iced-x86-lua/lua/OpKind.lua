-- SPDX-License-Identifier: MIT
-- Copyright (C) 2018-present iced project and contributors

-- ⚠️This file was generated by GENERATOR!🦹‍♂️

---Instruction operand kind
return {
	---A register (`Register`).
	---
	---This operand kind uses `Instruction:op0_register()`, `Instruction:op1_register()`, `Instruction:op2_register()`, `Instruction:op3_register()` or `Instruction:op4_register()` depending on operand number. See also `Instruction:op_register()`.
	Register = 0,
	---Near 16-bit branch. This operand kind uses `Instruction:near_branch16()`
	NearBranch16 = 1,
	---Near 32-bit branch. This operand kind uses `Instruction:near_branch32()`
	NearBranch32 = 2,
	---Near 64-bit branch. This operand kind uses `Instruction:near_branch64()`
	NearBranch64 = 3,
	---Far 16-bit branch. This operand kind uses `Instruction:far_branch16()` and `Instruction:far_branch_selector()`
	FarBranch16 = 4,
	---Far 32-bit branch. This operand kind uses `Instruction:far_branch32()` and `Instruction:far_branch_selector()`
	FarBranch32 = 5,
	---8-bit constant. This operand kind uses `Instruction:immediate8()`
	Immediate8 = 6,
	---8-bit constant used by the `ENTER`, `EXTRQ`, `INSERTQ` instructions. This operand kind uses `Instruction:immediate8_2nd()`
	Immediate8_2nd = 7,
	---16-bit constant. This operand kind uses `Instruction:immediate16()`
	Immediate16 = 8,
	---32-bit constant. This operand kind uses `Instruction:immediate32()`
	Immediate32 = 9,
	---64-bit constant. This operand kind uses `Instruction:immediate64()`
	Immediate64 = 10,
	---An 8-bit value sign extended to 16 bits. This operand kind uses `Instruction:immediate8to16()`
	Immediate8to16 = 11,
	---An 8-bit value sign extended to 32 bits. This operand kind uses `Instruction:immediate8to32()`
	Immediate8to32 = 12,
	---An 8-bit value sign extended to 64 bits. This operand kind uses `Instruction:immediate8to64()`
	Immediate8to64 = 13,
	---A 32-bit value sign extended to 64 bits. This operand kind uses `Instruction:immediate32to64()`
	Immediate32to64 = 14,
	---`seg:[SI]`. This operand kind uses `Instruction:memory_size()`, `Instruction:memory_segment()`, `Instruction:segment_prefix()`
	MemorySegSI = 15,
	---`seg:[ESI]`. This operand kind uses `Instruction:memory_size()`, `Instruction:memory_segment()`, `Instruction:segment_prefix()`
	MemorySegESI = 16,
	---`seg:[RSI]`. This operand kind uses `Instruction:memory_size()`, `Instruction:memory_segment()`, `Instruction:segment_prefix()`
	MemorySegRSI = 17,
	---`seg:[DI]`. This operand kind uses `Instruction:memory_size()`, `Instruction:memory_segment()`, `Instruction:segment_prefix()`
	MemorySegDI = 18,
	---`seg:[EDI]`. This operand kind uses `Instruction:memory_size()`, `Instruction:memory_segment()`, `Instruction:segment_prefix()`
	MemorySegEDI = 19,
	---`seg:[RDI]`. This operand kind uses `Instruction:memory_size()`, `Instruction:memory_segment()`, `Instruction:segment_prefix()`
	MemorySegRDI = 20,
	---`ES:[DI]`. This operand kind uses `Instruction:memory_size()`
	MemoryESDI = 21,
	---`ES:[EDI]`. This operand kind uses `Instruction:memory_size()`
	MemoryESEDI = 22,
	---`ES:[RDI]`. This operand kind uses `Instruction:memory_size()`
	MemoryESRDI = 23,
	---Memory operand.
	---
	---This operand kind uses `Instruction:memory_displ_size()`, `Instruction:memory_size()`, `Instruction:memory_index_scale()`, `Instruction:memory_displacement64()`, `Instruction:memory_base()`, `Instruction:memory_index()`, `Instruction:memory_segment()`, `Instruction:segment_prefix()`
	Memory = 24,
}
