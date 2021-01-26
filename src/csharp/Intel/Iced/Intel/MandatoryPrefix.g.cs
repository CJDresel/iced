// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

#nullable enable

#if ENCODER && OPCODE_INFO
namespace Iced.Intel {
	/// <summary>Mandatory prefix</summary>
	public enum MandatoryPrefix {
		/// <summary>No mandatory prefix (legacy and 3DNow! tables only)</summary>
		None = 0,
		/// <summary>Empty mandatory prefix (no <c>66</c>, <c>F3</c> or <c>F2</c> prefix)</summary>
		PNP = 1,
		/// <summary><c>66</c> prefix</summary>
		P66 = 2,
		/// <summary><c>F3</c> prefix</summary>
		PF3 = 3,
		/// <summary><c>F2</c> prefix</summary>
		PF2 = 4,
	}
}
#endif
