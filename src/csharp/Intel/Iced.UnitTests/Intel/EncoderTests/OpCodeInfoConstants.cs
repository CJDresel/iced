// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

#if ENCODER && OPCODE_INFO
using System;
using System.Collections.Generic;
using Iced.Intel;

namespace Iced.UnitTests.Intel.EncoderTests {
	static class OpCodeInfoDicts {
		// GENERATOR-BEGIN: Dicts
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		internal static readonly Dictionary<string, EncodingKind> ToEncodingKind = new Dictionary<string, EncodingKind>(6, StringComparer.Ordinal) {
			{ "legacy", EncodingKind.Legacy },
			{ "VEX", EncodingKind.VEX },
			{ "EVEX", EncodingKind.EVEX },
			{ "XOP", EncodingKind.XOP },
			{ "3DNow!", EncodingKind.D3NOW },
			{ "MVEX", EncodingKind.MVEX },
		};
		internal static readonly Dictionary<string, MandatoryPrefix> ToMandatoryPrefix = new Dictionary<string, MandatoryPrefix>(5, StringComparer.Ordinal) {
			{ "", MandatoryPrefix.None },
			{ "NP", MandatoryPrefix.PNP },
			{ "66", MandatoryPrefix.P66 },
			{ "F3", MandatoryPrefix.PF3 },
			{ "F2", MandatoryPrefix.PF2 },
		};
		internal static readonly Dictionary<string, OpCodeTableKind> ToOpCodeTableKind = new Dictionary<string, OpCodeTableKind>(9, StringComparer.Ordinal) {
			{ "legacy", OpCodeTableKind.Normal },
			{ "0F", OpCodeTableKind.T0F },
			{ "0F38", OpCodeTableKind.T0F38 },
			{ "0F3A", OpCodeTableKind.T0F3A },
			{ "MAP5", OpCodeTableKind.MAP5 },
			{ "MAP6", OpCodeTableKind.MAP6 },
			{ "X8", OpCodeTableKind.MAP8 },
			{ "X9", OpCodeTableKind.MAP9 },
			{ "XA", OpCodeTableKind.MAP10 },
		};
		// GENERATOR-END: Dicts
	}

	// GENERATOR-BEGIN: OpCodeInfoKeys
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	static class OpCodeInfoKeys {
		internal const string GroupIndex = "g";
		internal const string RmGroupIndex = "rmg";
		internal const string OpCodeOperandKind = "op";
		internal const string TupleType = "tt";
		internal const string DecoderOption = "dec-opt";
	}
	// GENERATOR-END: OpCodeInfoKeys

	// GENERATOR-BEGIN: OpCodeInfoFlags
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	static class OpCodeInfoFlags {
		internal const string NoInstruction = "no-instr";
		internal const string Bit16 = "16";
		internal const string Bit32 = "32";
		internal const string Bit64 = "64";
		internal const string Fwait = "fwait";
		internal const string OperandSize16 = "o16";
		internal const string OperandSize32 = "o32";
		internal const string OperandSize64 = "o64";
		internal const string AddressSize16 = "a16";
		internal const string AddressSize32 = "a32";
		internal const string AddressSize64 = "a64";
		internal const string LIG = "LIG";
		internal const string L0 = "L0";
		internal const string L1 = "L1";
		internal const string L128 = "L128";
		internal const string L256 = "L256";
		internal const string L512 = "L512";
		internal const string WIG = "WIG";
		internal const string WIG32 = "WIG32";
		internal const string W0 = "W0";
		internal const string W1 = "W1";
		internal const string Broadcast = "b";
		internal const string RoundingControl = "er";
		internal const string SuppressAllExceptions = "sae";
		internal const string OpMaskRegister = "k";
		internal const string RequireOpMaskRegister = "knz";
		internal const string ZeroingMasking = "z";
		internal const string Lock = "lock";
		internal const string Xacquire = "xacquire";
		internal const string Xrelease = "xrelease";
		internal const string Rep = "rep";
		internal const string Repe = "repe";
		internal const string Repne = "repne";
		internal const string Bnd = "bnd";
		internal const string HintTaken = "ht";
		internal const string Notrack = "notrack";
		internal const string IgnoresRoundingControl = "ignore-er";
		internal const string AmdLockRegBit = "amd-lock-reg-bit";
		internal const string DefaultOpSize64 = "do64";
		internal const string ForceOpSize64 = "fo64";
		internal const string IntelForceOpSize64 = "intel-fo64";
		internal const string Cpl0 = "cpl0";
		internal const string Cpl1 = "cpl1";
		internal const string Cpl2 = "cpl2";
		internal const string Cpl3 = "cpl3";
		internal const string InputOutput = "io";
		internal const string Nop = "nop";
		internal const string ReservedNop = "res-nop";
		internal const string SerializingIntel = "intel-serialize";
		internal const string SerializingAmd = "amd-serialize";
		internal const string MayRequireCpl0 = "may-require-cpl0";
		internal const string CetTracked = "cet-tracked";
		internal const string NonTemporal = "nt";
		internal const string FpuNoWait = "nowait";
		internal const string IgnoresModBits = "ignores-mod";
		internal const string No66 = "no66";
		internal const string NFx = "nfx";
		internal const string RequiresUniqueRegNums = "unique-reg-nums";
		internal const string Privileged = "priv";
		internal const string SaveRestore = "save-restore";
		internal const string StackInstruction = "stack";
		internal const string IgnoresSegment = "ignore-seg";
		internal const string OpMaskReadWrite = "krw";
		internal const string RealMode = "rm";
		internal const string ProtectedMode = "pm";
		internal const string Virtual8086Mode = "v86";
		internal const string CompatibilityMode = "cm";
		internal const string LongMode = "lm";
		internal const string UseOutsideSmm = "outside-smm";
		internal const string UseInSmm = "in-smm";
		internal const string UseOutsideEnclaveSgx = "outside-sgx";
		internal const string UseInEnclaveSgx1 = "in-sgx1";
		internal const string UseInEnclaveSgx2 = "in-sgx2";
		internal const string UseOutsideVmxOp = "outside-vmx-op";
		internal const string UseInVmxRootOp = "in-vmx-root-op";
		internal const string UseInVmxNonRootOp = "in-vmx-non-root-op";
		internal const string UseOutsideSeam = "outside-seam";
		internal const string UseInSeam = "in-seam";
		internal const string TdxNonRootGenUd = "tdx-non-root-gen-ud";
		internal const string TdxNonRootGenVe = "td-non-root-gen-ve";
		internal const string TdxNonRootMayGenEx = "tdx-non-root-may-gen-ex";
		internal const string IntelVmExit = "intel-vm-exit";
		internal const string IntelMayVmExit = "intel-may-vm-exit";
		internal const string IntelSmmVmExit = "intel-smm-vm-exit";
		internal const string AmdVmExit = "amd-vm-exit";
		internal const string AmdMayVmExit = "amd-may-vm-exit";
		internal const string TsxAbort = "tsx-abort";
		internal const string TsxImplAbort = "tsx-impl-abort";
		internal const string TsxMayAbort = "tsx-may-abort";
		internal const string IntelDecoder16 = "intel16";
		internal const string IntelDecoder32 = "intel32";
		internal const string IntelDecoder64 = "intel64";
		internal const string AmdDecoder16 = "amd16";
		internal const string AmdDecoder32 = "amd32";
		internal const string AmdDecoder64 = "amd64";
		internal const string RequiresUniqueDestRegNum = "unique-dest-reg-num";
	}
	// GENERATOR-END: OpCodeInfoFlags
}
#endif
