-- SPDX-License-Identifier: MIT
-- Copyright (C) 2018-present iced project and contributors

-- ⚠️This file was generated by GENERATOR!🦹‍♂️

---Decoder options
return {
	---No option is enabled
	None = 0x00000000,
	---Disable some checks for invalid encodings of instructions, eg. most instructions can't use a `LOCK` prefix so if one is found, they're decoded as `Code.INVALID` unless this option is enabled.
	NoInvalidCheck = 0x00000001,
	---AMD decoder: allow 16-bit branch/ret instructions in 64-bit mode, no `o64 CALL/JMP FAR [mem], o64 LSS/LFS/LGS`, `UD0` has no modr/m byte, decode `LOCK MOV CR`. The AMD decoder can still decode Intel instructions.
	AMD = 0x00000002,
	---Decode opcodes `0F0D` and `0F18-0F1F` as reserved-nop instructions (eg. `Code.Reservednop_rm32_r32_0F1D`)
	ForceReservedNop = 0x00000004,
	---Decode `UMOV` instructions
	Umov = 0x00000008,
	---Decode `XBTS`/`IBTS`
	Xbts = 0x00000010,
	---Decode `0FA6`/`0FA7` as `CMPXCHG`
	Cmpxchg486A = 0x00000020,
	---Decode some old removed FPU instructions (eg. `FRSTPM`)
	OldFpu = 0x00000040,
	---Decode `PCOMMIT`
	Pcommit = 0x00000080,
	---Decode 286 `STOREALL`/`LOADALL` (`0F04` and `0F05`)
	Loadall286 = 0x00000100,
	---Decode 386 `LOADALL`
	Loadall386 = 0x00000200,
	---Decode `CL1INVMB`
	Cl1invmb = 0x00000400,
	---Decode `MOV r32,tr` and `MOV tr,r32`
	MovTr = 0x00000800,
	---Decode `JMPE` instructions
	Jmpe = 0x00001000,
	---Don't decode `PAUSE`, decode `NOP` instead
	NoPause = 0x00002000,
	---Don't decode `WBNOINVD`, decode `WBINVD` instead
	NoWbnoinvd = 0x00004000,
	---Decode undocumented Intel `RDUDBG` and `WRUDBG` instructions
	Udbg = 0x00008000,
	---Don't decode `TZCNT`, decode `BSF` instead
	NoMPFX_0FBC = 0x00010000,
	---Don't decode `LZCNT`, decode `BSR` instead
	NoMPFX_0FBD = 0x00020000,
	---Don't decode `LAHF` and `SAHF` in 64-bit mode
	NoLahfSahf64 = 0x00040000,
	---Decode `MPX` instructions
	MPX = 0x00080000,
	---Decode most Cyrix instructions: `FPU`, `EMMI`, `SMM`, `DDI`
	Cyrix = 0x00100000,
	---Decode Cyrix `SMINT 0F7E` (Cyrix 6x86 or earlier)
	Cyrix_SMINT_0F7E = 0x00200000,
	---Decode Cyrix `DMI` instructions (AMD Geode GX/LX)
	Cyrix_DMI = 0x00400000,
	---Decode Centaur `ALTINST`
	ALTINST = 0x00800000,
	---Decode Intel Knights Corner instructions
	KNC = 0x01000000,
}
