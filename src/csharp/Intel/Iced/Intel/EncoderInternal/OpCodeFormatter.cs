// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

#if ENCODER && OPCODE_INFO
using System;
using System.Diagnostics;
using System.Text;

namespace Iced.Intel.EncoderInternal {
	// GENERATOR-BEGIN: LKind
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	enum LKind : byte {
		None,
		/// <summary>.128, .256, .512</summary>
		L128,
		/// <summary>.L0, .L1</summary>
		L0,
		/// <summary>.LZ</summary>
		LZ,
	}
	// GENERATOR-END: LKind

	readonly struct OpCodeFormatter {
		readonly OpCodeInfo opCode;
		readonly StringBuilder sb;
		readonly LKind lkind;
		readonly bool hasModrmInfo;

		public OpCodeFormatter(OpCodeInfo opCode, StringBuilder sb, LKind lkind, bool hasModrmInfo) {
			this.opCode = opCode;
			this.sb = sb;
			this.lkind = lkind;
			this.hasModrmInfo = hasModrmInfo;
		}

		public string Format() {
			if (!opCode.IsInstruction) {
				return opCode.Code switch {
					// GENERATOR-BEGIN: OpCodeFmtNotInstructionString
					// ⚠️This was generated by GENERATOR!🦹‍♂️
					Code.INVALID => "<invalid>",
					Code.DeclareByte => "<db>",
					Code.DeclareWord => "<dw>",
					Code.DeclareDword => "<dd>",
					Code.DeclareQword => "<dq>",
					// GENERATOR-END: OpCodeFmtNotInstructionString
					_ => throw new InvalidOperationException(),
				};
			}

			return opCode.Encoding switch {
				EncodingKind.Legacy => Format_Legacy(),
#if !NO_VEX
				EncodingKind.VEX => FormatVecEncoding("VEX"),
#else
				EncodingKind.VEX => string.Empty,
#endif
#if !NO_EVEX
				EncodingKind.EVEX => FormatVecEncoding("EVEX"),
#else
				EncodingKind.EVEX => string.Empty,
#endif
#if !NO_XOP
				EncodingKind.XOP => FormatVecEncoding("XOP"),
#else
				EncodingKind.XOP => string.Empty,
#endif
#if !NO_D3NOW
				EncodingKind.D3NOW => Format_3DNow(),
#else
				EncodingKind.D3NOW => string.Empty,
#endif
#if MVEX
				EncodingKind.MVEX => FormatVecEncoding("MVEX"),
#else
				EncodingKind.MVEX => string.Empty,
#endif
				_ => throw new InvalidOperationException(),
			};
		}

		void AppendHexByte(byte value) => sb.Append(value.ToString("X2"));

		void AppendOpCode(uint value, int valueLen, bool sep) {
			if (valueLen == 1)
				AppendHexByte((byte)value);
			else {
				Debug.Assert(valueLen == 2);
				AppendHexByte((byte)(value >> 8));
				if (sep)
					sb.Append(' ');
				AppendHexByte((byte)value);
			}
		}

		void AppendTable(bool sep) {
			switch (opCode.Table) {
			case OpCodeTableKind.Normal:
				break;

			case OpCodeTableKind.T0F:
				AppendOpCode(0x0F, 1, sep);
				break;

			case OpCodeTableKind.T0F38:
				AppendOpCode(0x0F38, 2, sep);
				break;

			case OpCodeTableKind.T0F3A:
				AppendOpCode(0x0F3A, 2, sep);
				break;

			case OpCodeTableKind.MAP5:
				sb.Append("MAP5");
				break;

			case OpCodeTableKind.MAP6:
				sb.Append("MAP6");
				break;

			case OpCodeTableKind.MAP8:
				sb.Append("X8");
				break;

			case OpCodeTableKind.MAP9:
				sb.Append("X9");
				break;

			case OpCodeTableKind.MAP10:
				sb.Append("XA");
				break;

			default:
				throw new InvalidOperationException();
			}
		}

		bool HasModRM() {
			int opCount = opCode.OpCount;
			if (opCount == 0)
				return false;

			switch (opCode.Encoding) {
			case EncodingKind.Legacy:
				break;
			case EncodingKind.VEX:
			case EncodingKind.EVEX:
			case EncodingKind.XOP:
			case EncodingKind.D3NOW:
			case EncodingKind.MVEX:
				return true;
			default:
				throw new InvalidOperationException();
			}

			for (int i = 0; i < opCount; i++) {
				switch (opCode.GetOpKind(i)) {
				// GENERATOR-BEGIN: HasModRM
				// ⚠️This was generated by GENERATOR!🦹‍♂️
				case OpCodeOperandKind.mem:
				case OpCodeOperandKind.mem_mpx:
				case OpCodeOperandKind.mem_mib:
				case OpCodeOperandKind.mem_vsib32x:
				case OpCodeOperandKind.mem_vsib64x:
				case OpCodeOperandKind.mem_vsib32y:
				case OpCodeOperandKind.mem_vsib64y:
				case OpCodeOperandKind.mem_vsib32z:
				case OpCodeOperandKind.mem_vsib64z:
				case OpCodeOperandKind.r8_or_mem:
				case OpCodeOperandKind.r16_or_mem:
				case OpCodeOperandKind.r32_or_mem:
				case OpCodeOperandKind.r32_or_mem_mpx:
				case OpCodeOperandKind.r64_or_mem:
				case OpCodeOperandKind.r64_or_mem_mpx:
				case OpCodeOperandKind.mm_or_mem:
				case OpCodeOperandKind.xmm_or_mem:
				case OpCodeOperandKind.ymm_or_mem:
				case OpCodeOperandKind.zmm_or_mem:
				case OpCodeOperandKind.bnd_or_mem_mpx:
				case OpCodeOperandKind.k_or_mem:
				case OpCodeOperandKind.r8_reg:
				case OpCodeOperandKind.r16_reg:
				case OpCodeOperandKind.r16_reg_mem:
				case OpCodeOperandKind.r16_rm:
				case OpCodeOperandKind.r32_reg:
				case OpCodeOperandKind.r32_reg_mem:
				case OpCodeOperandKind.r32_rm:
				case OpCodeOperandKind.r64_reg:
				case OpCodeOperandKind.r64_reg_mem:
				case OpCodeOperandKind.r64_rm:
				case OpCodeOperandKind.seg_reg:
				case OpCodeOperandKind.k_reg:
				case OpCodeOperandKind.kp1_reg:
				case OpCodeOperandKind.k_rm:
				case OpCodeOperandKind.mm_reg:
				case OpCodeOperandKind.mm_rm:
				case OpCodeOperandKind.xmm_reg:
				case OpCodeOperandKind.xmm_rm:
				case OpCodeOperandKind.ymm_reg:
				case OpCodeOperandKind.ymm_rm:
				case OpCodeOperandKind.zmm_reg:
				case OpCodeOperandKind.zmm_rm:
				case OpCodeOperandKind.cr_reg:
				case OpCodeOperandKind.dr_reg:
				case OpCodeOperandKind.tr_reg:
				case OpCodeOperandKind.bnd_reg:
				case OpCodeOperandKind.sibmem:
				case OpCodeOperandKind.tmm_reg:
				case OpCodeOperandKind.tmm_rm:
					return true;
				// GENERATOR-END: HasModRM
				default:
					break;
				}
			}
			return false;
		}

		bool HasVsib() {
			int opCount = opCode.OpCount;
			for (int i = 0; i < opCount; i++) {
				switch (opCode.GetOpKind(i)) {
				// GENERATOR-BEGIN: HasVsib
				// ⚠️This was generated by GENERATOR!🦹‍♂️
				case OpCodeOperandKind.mem_vsib32x:
				case OpCodeOperandKind.mem_vsib64x:
				case OpCodeOperandKind.mem_vsib32y:
				case OpCodeOperandKind.mem_vsib64y:
				case OpCodeOperandKind.mem_vsib32z:
				case OpCodeOperandKind.mem_vsib64z:
					return true;
				// GENERATOR-END: HasVsib
				default:
					break;
				}
			}
			return false;
		}

		OpCodeOperandKind GetOpCodeBitsOperand() {
			int opCount = opCode.OpCount;
			for (int i = 0; i < opCount; i++) {
				var opKind = opCode.GetOpKind(i);
				switch (opKind) {
				case OpCodeOperandKind.r8_opcode:
				case OpCodeOperandKind.r16_opcode:
				case OpCodeOperandKind.r32_opcode:
				case OpCodeOperandKind.r64_opcode:
					return opKind;
				}
			}
			return OpCodeOperandKind.None;
		}

		bool TryGetModrmInfo(out bool isRegOnly, out int rrr, out int bbb) {
			isRegOnly = true;
			rrr = opCode.GroupIndex;
			bbb = opCode.RmGroupIndex;
			if (!hasModrmInfo)
				return false;

			int opCount = opCode.OpCount;
			for (int i = 0; i < opCount; i++) {
				switch (opCode.GetOpKind(i)) {
				case OpCodeOperandKind.mem_offs:
				case OpCodeOperandKind.mem:
				case OpCodeOperandKind.mem_mpx:
				case OpCodeOperandKind.mem_mib:
					isRegOnly = false;
					break;
				case OpCodeOperandKind.mem_vsib32x:
				case OpCodeOperandKind.mem_vsib64x:
				case OpCodeOperandKind.mem_vsib32y:
				case OpCodeOperandKind.mem_vsib64y:
				case OpCodeOperandKind.mem_vsib32z:
				case OpCodeOperandKind.mem_vsib64z:
				case OpCodeOperandKind.sibmem:
					isRegOnly = false;
					bbb = 4;
					break;
				}
			}
			return true;
		}

		void AppendBits(string name, int bits, int numBits) {
			if (bits < 0)
				sb.Append(name);
			else {
				for (int i = numBits - 1; i >= 0; i--) {
					if (((bits >> i) & 1) != 0)
						sb.Append('1');
					else
						sb.Append('0');
				}
			}
		}

		void AppendRest() {
			if (TryGetModrmInfo(out var isRegOnly, out var rrr, out var bbb)) {
				if (isRegOnly)
					sb.Append(" 11:");
				else
					sb.Append(" !(11):");
				AppendBits("rrr", rrr, 3);
				sb.Append(':');
				AppendBits("bbb", bbb, 3);
			}
			else {
				bool isVsib = (opCode.Encoding == EncodingKind.EVEX || opCode.Encoding == EncodingKind.MVEX) && HasVsib();
				if (opCode.IsGroup) {
					sb.Append(" /");
					sb.Append(opCode.GroupIndex);
				}
				else if (!isVsib && HasModRM())
					sb.Append(" /r");
				if (isVsib)
					sb.Append(" /vsib");
			}

			int opCount = opCode.OpCount;
			for (int i = 0; i < opCount; i++) {
				switch (opCode.GetOpKind(i)) {
				case OpCodeOperandKind.br16_1:
				case OpCodeOperandKind.br32_1:
				case OpCodeOperandKind.br64_1:
					sb.Append(" cb");
					break;

				case OpCodeOperandKind.br16_2:
				case OpCodeOperandKind.xbegin_2:
				case OpCodeOperandKind.brdisp_2:
					sb.Append(" cw");
					break;

				case OpCodeOperandKind.farbr2_2:
				case OpCodeOperandKind.br32_4:
				case OpCodeOperandKind.br64_4:
				case OpCodeOperandKind.xbegin_4:
				case OpCodeOperandKind.brdisp_4:
					sb.Append(" cd");
					break;

				case OpCodeOperandKind.farbr4_2:
					sb.Append(" cp");
					break;

				case OpCodeOperandKind.imm8:
				case OpCodeOperandKind.imm8sex16:
				case OpCodeOperandKind.imm8sex32:
				case OpCodeOperandKind.imm8sex64:
					sb.Append(" ib");
					break;

				case OpCodeOperandKind.imm16:
					sb.Append(" iw");
					break;

				case OpCodeOperandKind.imm32:
				case OpCodeOperandKind.imm32sex64:
					sb.Append(" id");
					break;

				case OpCodeOperandKind.imm64:
					sb.Append(" io");
					break;

				case OpCodeOperandKind.xmm_is4:
				case OpCodeOperandKind.ymm_is4:
					sb.Append(" /is4");
					break;

				case OpCodeOperandKind.xmm_is5:
				case OpCodeOperandKind.ymm_is5:
					sb.Append(" /is5");
					// don't show the next imm8
					i = opCount;
					break;

				case OpCodeOperandKind.mem_offs:
					sb.Append(" mo");
					break;
				}
			}
		}

		string Format_Legacy() {
			sb.Length = 0;

			if (opCode.Fwait) {
				AppendHexByte(0x9B);
				sb.Append(' ');
			}

			switch (opCode.AddressSize) {
			case 0:
				break;

			case 16:
				sb.Append("a16 ");
				break;

			case 32:
				sb.Append("a32 ");
				break;

			case 64:
				sb.Append("a64 ");
				break;

			default:
				throw new InvalidOperationException();
			}

			switch (opCode.OperandSize) {
			case 0:
				break;

			case 16:
				sb.Append("o16 ");
				break;

			case 32:
				sb.Append("o32 ");
				break;

			case 64:
				// o64 (REX.W) must be immediately before the opcode and is handled below
				break;

			default:
				throw new InvalidOperationException();
			}

			switch (opCode.MandatoryPrefix) {
			case MandatoryPrefix.None:
				break;
			case MandatoryPrefix.PNP:
				sb.Append("NP ");
				break;
			case MandatoryPrefix.P66:
				AppendHexByte(0x66);
				sb.Append(' ');
				break;
			case MandatoryPrefix.PF3:
				AppendHexByte(0xF3);
				sb.Append(' ');
				break;
			case MandatoryPrefix.PF2:
				AppendHexByte(0xF2);
				sb.Append(' ');
				break;
			default:
				throw new InvalidOperationException();
			}

			if (opCode.OperandSize == 64)
				sb.Append("o64 ");

			AppendTable(true);
			if (opCode.Table != OpCodeTableKind.Normal)
				sb.Append(' ');
			AppendOpCode(opCode.OpCode, opCode.OpCodeLength, true);
			switch (GetOpCodeBitsOperand()) {
			case OpCodeOperandKind.r8_opcode:
				sb.Append("+rb");
				break;
			case OpCodeOperandKind.r16_opcode:
				sb.Append("+rw");
				break;
			case OpCodeOperandKind.r32_opcode:
				sb.Append("+rd");
				break;
			case OpCodeOperandKind.r64_opcode:
				sb.Append("+ro");
				break;
			case OpCodeOperandKind.None:
				break;
			default:
				throw new InvalidOperationException();
			}
			int opCount = opCode.OpCount;
			for (int i = 0; i < opCount; i++) {
				if (opCode.GetOpKind(i) == OpCodeOperandKind.sti_opcode) {
					sb.Append("+i");
					break;
				}
			}

			AppendRest();

			return sb.ToString();
		}

#if !NO_D3NOW
		string Format_3DNow() {
			sb.Length = 0;

			AppendOpCode(0x0F0F, 2, true);
			sb.Append(" /r");
			sb.Append(' ');
			AppendOpCode(opCode.OpCode, opCode.OpCodeLength, true);

			return sb.ToString();
		}
#endif

#if !NO_VEX || !NO_XOP || !NO_EVEX || MVEX
		string FormatVecEncoding(string encodingName) {
			sb.Length = 0;

			sb.Append(encodingName);
			sb.Append('.');
			if (opCode.IsLIG)
				sb.Append("LIG");
			else {
				switch (lkind) {
				case LKind.L128:
					sb.Append(128U << (int)opCode.L);
					break;
				case LKind.L0:
					sb.Append('L');
					sb.Append(opCode.L);
					break;
				case LKind.LZ:
					if (opCode.L != 0)
						throw new InvalidOperationException();
					sb.Append("LZ");
					break;
				case LKind.None:
				default:
					throw new InvalidOperationException();
				}
			}
			switch (opCode.MandatoryPrefix) {
			case MandatoryPrefix.None:
			case MandatoryPrefix.PNP:
				break;
			case MandatoryPrefix.P66:
				sb.Append('.');
				AppendHexByte(0x66);
				break;
			case MandatoryPrefix.PF3:
				sb.Append('.');
				AppendHexByte(0xF3);
				break;
			case MandatoryPrefix.PF2:
				sb.Append('.');
				AppendHexByte(0xF2);
				break;
			default:
				throw new InvalidOperationException();
			}
			sb.Append('.');
			AppendTable(false);
			if (opCode.IsWIG)
				sb.Append(".WIG");
			else {
				sb.Append(".W");
				sb.Append(opCode.W);
			}
			sb.Append(' ');
			AppendOpCode(opCode.OpCode, opCode.OpCodeLength, true);
			AppendRest();

			return sb.ToString();
		}
#endif
	}
}
#endif
