// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

#if DECODER
using System.Diagnostics;

namespace Iced.Intel.DecoderInternal {
#if !NO_D3NOW
	sealed class OpCodeHandler_D3NOW : OpCodeHandlerModRM {
		static readonly Code[] CodeValues = CreateCodeValues();

		static Code[] CreateCodeValues() {
			var result = new Code[0x100];
			Static.Assert(Code.INVALID == 0 ? 0 : -1);
			// GENERATOR-BEGIN: D3nowCodeValues
			// ⚠️This was generated by GENERATOR!🦹‍♂️
			result[0xBF] = Code.D3NOW_Pavgusb_mm_mmm64;
			result[0xBB] = Code.D3NOW_Pswapd_mm_mmm64;
			result[0xB7] = Code.D3NOW_Pmulhrw_mm_mmm64;
			result[0xB6] = Code.D3NOW_Pfrcpit2_mm_mmm64;
			result[0xB4] = Code.D3NOW_Pfmul_mm_mmm64;
			result[0xB0] = Code.D3NOW_Pfcmpeq_mm_mmm64;
			result[0xAE] = Code.D3NOW_Pfacc_mm_mmm64;
			result[0xAA] = Code.D3NOW_Pfsubr_mm_mmm64;
			result[0xA7] = Code.D3NOW_Pfrsqit1_mm_mmm64;
			result[0xA6] = Code.D3NOW_Pfrcpit1_mm_mmm64;
			result[0xA4] = Code.D3NOW_Pfmax_mm_mmm64;
			result[0xA0] = Code.D3NOW_Pfcmpgt_mm_mmm64;
			result[0x9E] = Code.D3NOW_Pfadd_mm_mmm64;
			result[0x9A] = Code.D3NOW_Pfsub_mm_mmm64;
			result[0x97] = Code.D3NOW_Pfrsqrt_mm_mmm64;
			result[0x96] = Code.D3NOW_Pfrcp_mm_mmm64;
			result[0x94] = Code.D3NOW_Pfmin_mm_mmm64;
			result[0x90] = Code.D3NOW_Pfcmpge_mm_mmm64;
			result[0x8E] = Code.D3NOW_Pfpnacc_mm_mmm64;
			result[0x8A] = Code.D3NOW_Pfnacc_mm_mmm64;
			result[0x87] = Code.D3NOW_Pfrsqrtv_mm_mmm64;
			result[0x86] = Code.D3NOW_Pfrcpv_mm_mmm64;
			result[0x1D] = Code.D3NOW_Pf2id_mm_mmm64;
			result[0x1C] = Code.D3NOW_Pf2iw_mm_mmm64;
			result[0x0D] = Code.D3NOW_Pi2fd_mm_mmm64;
			result[0x0C] = Code.D3NOW_Pi2fw_mm_mmm64;
			// GENERATOR-END: D3nowCodeValues
			return result;
		}

		readonly Code[] codeValues = CodeValues;

		public override void Decode(Decoder decoder, ref Instruction instruction) {
			Debug.Assert(decoder.state.Encoding == EncodingKind.Legacy);
			Static.Assert(OpKind.Register == 0 ? 0 : -1);
			//instruction.Op0Kind = OpKind.Register;
			instruction.Op0Register = (int)decoder.state.reg + Register.MM0;
			if (decoder.state.mod == 3) {
				Static.Assert(OpKind.Register == 0 ? 0 : -1);
				//instruction.Op1Kind = OpKind.Register;
				instruction.Op1Register = (int)decoder.state.rm + Register.MM0;
			}
			else {
				instruction.Op1Kind = OpKind.Memory;
				decoder.ReadOpMem(ref instruction);
			}
			var code = codeValues[(int)decoder.ReadByte()];
			switch (code) {
			case Code.D3NOW_Pfrcpv_mm_mmm64:
			case Code.D3NOW_Pfrsqrtv_mm_mmm64:
				if ((decoder.options & DecoderOptions.Cyrix) == 0 || decoder.Bitness == 64)
					code = Code.INVALID;
				break;
			}
			instruction.InternalSetCodeNoCheck(code);
			if (code == Code.INVALID)
				decoder.SetInvalidInstruction();
		}
	}
#else
	sealed class OpCodeHandler_D3NOW : OpCodeHandlerModRM {
		public override void Decode(Decoder decoder, ref Instruction instruction) {
			Debug.Assert(decoder.state.Encoding == EncodingKind.Legacy);
			decoder.SetInvalidInstruction();
		}
	}
#endif
}
#endif
