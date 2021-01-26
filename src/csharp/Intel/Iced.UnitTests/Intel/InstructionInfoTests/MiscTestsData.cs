// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

#if INSTR_INFO
using System;
using System.Collections.Generic;
using Iced.Intel;

namespace Iced.UnitTests.Intel.InstructionInfoTests {
	// GENERATOR-BEGIN: MiscSectionNames
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	static class MiscSectionNames {
		internal const string JccShort = "jcc-short";
		internal const string JccNear = "jcc-near";
		internal const string JmpShort = "jmp-short";
		internal const string JmpNear = "jmp-near";
		internal const string JmpFar = "jmp-far";
		internal const string JmpNearIndirect = "jmp-near-indirect";
		internal const string JmpFarIndirect = "jmp-far-indirect";
		internal const string CallNear = "call-near";
		internal const string CallFar = "call-far";
		internal const string CallNearIndirect = "call-near-indirect";
		internal const string CallFarIndirect = "call-far-indirect";
		internal const string JmpeNear = "jmpe-near";
		internal const string JmpeNearIndirect = "jmpe-near-indirect";
		internal const string Loop = "loop";
		internal const string Jrcxz = "jrcxz";
		internal const string Xbegin = "xbegin";
		internal const string JmpInfo = "jmp-info";
		internal const string JccShortInfo = "jcc-short-info";
		internal const string JccNearInfo = "jcc-near-info";
		internal const string SetccInfo = "setcc-info";
		internal const string CmovccInfo = "cmovcc-info";
		internal const string LoopccInfo = "loopcc-info";
	}
	// GENERATOR-END: MiscSectionNames

	static class MiscTestsData {
		public static readonly HashSet<Code> JccShort;
		public static readonly HashSet<Code> JmpNear;
		public static readonly HashSet<Code> JmpFar;
		public static readonly HashSet<Code> JmpShort;
		public static readonly HashSet<Code> JmpNearIndirect;
		public static readonly HashSet<Code> JmpFarIndirect;
		public static readonly HashSet<Code> JccNear;
		public static readonly HashSet<Code> CallFar;
		public static readonly HashSet<Code> CallNear;
		public static readonly HashSet<Code> CallNearIndirect;
		public static readonly HashSet<Code> CallFarIndirect;
		public static readonly HashSet<Code> JmpeNear;
		public static readonly HashSet<Code> JmpeNearIndirect;
		public static readonly HashSet<Code> Loop;
		public static readonly HashSet<Code> Jrcxz;
		public static readonly HashSet<Code> Xbegin;
		public static readonly (Code jmpShort, Code jmpNear)[] JmpInfos;
		public static readonly (Code jcc, Code negated, Code jccNear, ConditionCode cc)[] JccShortInfos;
		public static readonly (Code jcc, Code negated, Code jccShort, ConditionCode cc)[] JccNearInfos;
		public static readonly (Code setcc, Code negated, ConditionCode cc)[] SetccInfos;
		public static readonly (Code cmovcc, Code negated, ConditionCode cc)[] CmovccInfos;
		public static readonly (Code loopcc, Code negated, ConditionCode cc)[] LoopccInfos;

		static MiscTestsData() {
			var jccShort = new HashSet<Code>();
			var jmpNear = new HashSet<Code>();
			var jmpFar = new HashSet<Code>();
			var jmpShort = new HashSet<Code>();
			var jmpNearIndirect = new HashSet<Code>();
			var jmpFarIndirect = new HashSet<Code>();
			var jccNear = new HashSet<Code>();
			var callFar = new HashSet<Code>();
			var callNear = new HashSet<Code>();
			var callNearIndirect = new HashSet<Code>();
			var callFarIndirect = new HashSet<Code>();
			var jmpeNear = new HashSet<Code>();
			var jmpeNearIndirect = new HashSet<Code>();
			var loop = new HashSet<Code>();
			var jrcxz = new HashSet<Code>();
			var xbegin = new HashSet<Code>();
			var jmpInfos = new List<(Code jmpShort, Code jmpNear)>();
			var jccShortInfos = new List<(Code jcc, Code negated, Code jccNear, ConditionCode cc)>();
			var jccNearInfos = new List<(Code jcc, Code negated, Code jccShort, ConditionCode cc)>();
			var setccInfos = new List<(Code setcc, Code negated, ConditionCode cc)>();
			var cmovccInfos = new List<(Code cmovcc, Code negated, ConditionCode cc)>();
			var loopccInfos = new List<(Code loopcc, Code negated, ConditionCode cc)>();

			var sectionInfos = new (string sectionName, Action<string, string> handler)[] {
				(MiscSectionNames.JccShort, (_, line) => AddCode(jccShort, line)),
				(MiscSectionNames.JmpNear, (_, line) => AddCode(jmpNear, line)),
				(MiscSectionNames.JmpFar, (_, line) => AddCode(jmpFar, line)),
				(MiscSectionNames.JmpShort, (_, line) => AddCode(jmpShort, line)),
				(MiscSectionNames.JmpNearIndirect, (_, line) => AddCode(jmpNearIndirect, line)),
				(MiscSectionNames.JmpFarIndirect, (_, line) => AddCode(jmpFarIndirect, line)),
				(MiscSectionNames.JccNear, (_, line) => AddCode(jccNear, line)),
				(MiscSectionNames.CallFar, (_, line) => AddCode(callFar, line)),
				(MiscSectionNames.CallNear, (_, line) => AddCode(callNear, line)),
				(MiscSectionNames.CallNearIndirect, (_, line) => AddCode(callNearIndirect, line)),
				(MiscSectionNames.CallFarIndirect, (_, line) => AddCode(callFarIndirect, line)),
				(MiscSectionNames.JmpeNear, (_, line) => AddCode(jmpeNear, line)),
				(MiscSectionNames.JmpeNearIndirect, (_, line) => AddCode(jmpeNearIndirect, line)),
				(MiscSectionNames.Loop, (_, line) => AddCode(loop, line)),
				(MiscSectionNames.Jrcxz, (_, line) => AddCode(jrcxz, line)),
				(MiscSectionNames.Xbegin, (_, line) => AddCode(xbegin, line)),
				(MiscSectionNames.JmpInfo, (_, line) => AddJmpInfo(jmpInfos, line)),
				(MiscSectionNames.JccShortInfo, (_, line) => AddJccInfo(jccShortInfos, line)),
				(MiscSectionNames.JccNearInfo, (_, line) => AddJccInfo(jccNearInfos, line)),
				(MiscSectionNames.SetccInfo, (_, line) => AddInstrCcInfo(setccInfos, line)),
				(MiscSectionNames.CmovccInfo, (_, line) => AddInstrCcInfo(cmovccInfos, line)),
				(MiscSectionNames.LoopccInfo, (_, line) => AddInstrCcInfo(loopccInfos, line)),
			};
			var filename = PathUtils.GetTestTextFilename("Misc.txt", "InstructionInfo");
			SectionFileReader.Read(filename, sectionInfos);

			JccShort = jccShort;
			JmpNear = jmpNear;
			JmpFar = jmpFar;
			JmpShort = jmpShort;
			JmpNearIndirect = jmpNearIndirect;
			JmpFarIndirect = jmpFarIndirect;
			JccNear = jccNear;
			CallFar = callFar;
			CallNear = callNear;
			CallNearIndirect = callNearIndirect;
			CallFarIndirect = callFarIndirect;
			JmpeNear = jmpeNear;
			JmpeNearIndirect = jmpeNearIndirect;
			Loop = loop;
			Jrcxz = jrcxz;
			Xbegin = xbegin;
			JmpInfos = jmpInfos.ToArray();
			JccShortInfos = jccShortInfos.ToArray();
			JccNearInfos = jccNearInfos.ToArray();
			SetccInfos = setccInfos.ToArray();
			CmovccInfos = cmovccInfos.ToArray();
			LoopccInfos = loopccInfos.ToArray();
		}

		static void AddCode(HashSet<Code> hash, string line) {
			var code = line.Trim();
			if (CodeUtils.IsIgnored(code))
				return;
			if (!hash.Add(ToEnumConverter.GetCode(code)))
				throw new Exception($"Duplicate {nameof(Code)} value");
		}

		static readonly char[] commaSep = new char[] { ',' };
		static void AddJmpInfo(List<(Code, Code)> infos, string line) {
			const int ELEMS = 2;
			var elems = line.Split(commaSep);
			if (elems.Length != ELEMS)
				throw new InvalidOperationException($"Expected {ELEMS} elements, found {elems.Length}");
			var code1 = elems[0].Trim();
			var code2 = elems[1].Trim();
			if (CodeUtils.IsIgnored(code1) || CodeUtils.IsIgnored(code2))
				return;
			infos.Add((ToEnumConverter.GetCode(code1), ToEnumConverter.GetCode(code2)));
		}

		static void AddJccInfo(List<(Code, Code, Code, ConditionCode)> infos, string line) {
			const int ELEMS = 4;
			var elems = line.Split(commaSep);
			if (elems.Length != ELEMS)
				throw new InvalidOperationException($"Expected {ELEMS} elements, found {elems.Length}");
			var code1 = elems[0].Trim();
			var code2 = elems[1].Trim();
			var code3 = elems[2].Trim();
			if (CodeUtils.IsIgnored(code1) || CodeUtils.IsIgnored(code2) || CodeUtils.IsIgnored(code3))
				return;
			infos.Add((ToEnumConverter.GetCode(code1), ToEnumConverter.GetCode(code2), ToEnumConverter.GetCode(code3), ToEnumConverter.GetConditionCode(elems[3].Trim())));
		}

		static void AddInstrCcInfo(List<(Code, Code, ConditionCode)> infos, string line) {
			const int ELEMS = 3;
			var elems = line.Split(commaSep);
			if (elems.Length != ELEMS)
				throw new InvalidOperationException($"Expected {ELEMS} elements, found {elems.Length}");
			var code1 = elems[0].Trim();
			var code2 = elems[1].Trim();
			if (CodeUtils.IsIgnored(code1) || CodeUtils.IsIgnored(code2))
				return;
			infos.Add((ToEnumConverter.GetCode(code1), ToEnumConverter.GetCode(code2), ToEnumConverter.GetConditionCode(elems[2].Trim())));
		}
	}
}
#endif
