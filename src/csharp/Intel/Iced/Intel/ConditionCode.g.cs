// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

#nullable enable

#if INSTR_INFO
namespace Iced.Intel {
	/// <summary>Instruction condition code (used by <c>Jcc</c>, <c>SETcc</c>, <c>CMOVcc</c>, <c>LOOPcc</c>)</summary>
	public enum ConditionCode {
		/// <summary>The instruction doesn&apos;t have a condition code</summary>
		None = 0,
		/// <summary>Overflow (<c>OF=1</c>)</summary>
		o = 1,
		/// <summary>Not overflow (<c>OF=0</c>)</summary>
		no = 2,
		/// <summary>Below (unsigned) (<c>CF=1</c>)</summary>
		b = 3,
		/// <summary>Above or equal (unsigned) (<c>CF=0</c>)</summary>
		ae = 4,
		/// <summary>Equal / zero (<c>ZF=1</c>)</summary>
		e = 5,
		/// <summary>Not equal / zero (<c>ZF=0</c>)</summary>
		ne = 6,
		/// <summary>Below or equal (unsigned) (<c>CF=1 or ZF=1</c>)</summary>
		be = 7,
		/// <summary>Above (unsigned) (<c>CF=0 and ZF=0</c>)</summary>
		a = 8,
		/// <summary>Signed (<c>SF=1</c>)</summary>
		s = 9,
		/// <summary>Not signed (<c>SF=0</c>)</summary>
		ns = 10,
		/// <summary>Parity (<c>PF=1</c>)</summary>
		p = 11,
		/// <summary>Not parity (<c>PF=0</c>)</summary>
		np = 12,
		/// <summary>Less (signed) (<c>SF!=OF</c>)</summary>
		l = 13,
		/// <summary>Greater than or equal (signed) (<c>SF=OF</c>)</summary>
		ge = 14,
		/// <summary>Less than or equal (signed) (<c>ZF=1 or SF!=OF</c>)</summary>
		le = 15,
		/// <summary>Greater (signed) (<c>ZF=0 and SF=OF</c>)</summary>
		g = 16,
	}
}
#endif
