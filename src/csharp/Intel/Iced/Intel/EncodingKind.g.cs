// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

#nullable enable

#if DECODER || ENCODER || INSTR_INFO
namespace Iced.Intel {
	/// <summary>Instruction encoding</summary>
	public enum EncodingKind {
		/// <summary>Legacy encoding</summary>
		Legacy = 0,
		/// <summary>VEX encoding</summary>
		VEX = 1,
		/// <summary>EVEX encoding</summary>
		EVEX = 2,
		/// <summary>XOP encoding</summary>
		XOP = 3,
		/// <summary>3DNow! encoding</summary>
		D3NOW = 4,
	}
}
#endif
