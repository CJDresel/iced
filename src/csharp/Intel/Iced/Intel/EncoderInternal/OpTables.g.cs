// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

#nullable enable

#if ENCODER
namespace Iced.Intel.EncoderInternal {
	static class OpHandlerData {
		public static readonly Op[] LegacyOps = new Op[75] {
			new OpA(2),
			new OpA(4),
			new OpO(),
			new OpModRM_rm_mem_only(false),
			new OpModRM_rm_mem_only(false),
			new OpModRM_rm_mem_only(false),
			new OpModRM_rm(Register.AL, Register.R15L),
			new OpModRM_rm(Register.AX, Register.R15W),
			new OpModRM_rm(Register.EAX, Register.R15D),
			new OpModRM_rm(Register.EAX, Register.R15D),
			new OpModRM_rm(Register.RAX, Register.R15),
			new OpModRM_rm(Register.RAX, Register.R15),
			new OpModRM_rm(Register.MM0, Register.MM7),
			new OpModRM_rm(Register.XMM0, Register.XMM15),
			new OpModRM_rm(Register.BND0, Register.BND3),
			new OpModRM_reg(Register.AL, Register.R15L),
			new OpRegEmbed8(Register.AL, Register.R15L),
			new OpModRM_reg(Register.AX, Register.R15W),
			new OpModRM_reg_mem(Register.AX, Register.R15W),
			new OpModRM_rm_reg_only(Register.AX, Register.R15W),
			new OpRegEmbed8(Register.AX, Register.R15W),
			new OpModRM_reg(Register.EAX, Register.R15D),
			new OpModRM_reg_mem(Register.EAX, Register.R15D),
			new OpModRM_rm_reg_only(Register.EAX, Register.R15D),
			new OpRegEmbed8(Register.EAX, Register.R15D),
			new OpModRM_reg(Register.RAX, Register.R15),
			new OpModRM_reg_mem(Register.RAX, Register.R15),
			new OpModRM_rm_reg_only(Register.RAX, Register.R15),
			new OpRegEmbed8(Register.RAX, Register.R15),
			new OpModRM_reg(Register.ES, Register.GS),
			new OpModRM_reg(Register.MM0, Register.MM7),
			new OpModRM_rm_reg_only(Register.MM0, Register.MM7),
			new OpModRM_reg(Register.XMM0, Register.XMM15),
			new OpModRM_rm_reg_only(Register.XMM0, Register.XMM15),
			new OpModRM_regF0(Register.CR0, Register.CR15),
			new OpModRM_reg(Register.DR0, Register.DR15),
			new OpModRM_reg(Register.TR0, Register.TR7),
			new OpModRM_reg(Register.BND0, Register.BND3),
			new OpReg(Register.ES),
			new OpReg(Register.CS),
			new OpReg(Register.SS),
			new OpReg(Register.DS),
			new OpReg(Register.FS),
			new OpReg(Register.GS),
			new OpReg(Register.AL),
			new OpReg(Register.CL),
			new OpReg(Register.AX),
			new OpReg(Register.DX),
			new OpReg(Register.EAX),
			new OpReg(Register.RAX),
			new OpReg(Register.ST0),
			new OpRegSTi(),
			new OpIb(OpKind.Immediate8),
			new OpImm(1),
			new OpIb(OpKind.Immediate8to16),
			new OpIb(OpKind.Immediate8to32),
			new OpIb(OpKind.Immediate8to64),
			new OpIw(),
			new OpId(OpKind.Immediate32),
			new OpId(OpKind.Immediate32to64),
			new OpIq(),
			new OpX(),
			new OpY(),
			new OprDI(),
			new OpMRBX(),
			new OpJ(OpKind.NearBranch16, 1),
			new OpJ(OpKind.NearBranch32, 1),
			new OpJ(OpKind.NearBranch64, 1),
			new OpJ(OpKind.NearBranch16, 2),
			new OpJ(OpKind.NearBranch32, 4),
			new OpJ(OpKind.NearBranch64, 4),
			new OpJx(2),
			new OpJx(4),
			new OpJdisp(2),
			new OpJdisp(4),
		};
#if !NO_VEX
		public static readonly Op[] VexOps = new Op[36] {
			new OpModRM_rm_mem_only(false),
			new OpVsib(Register.XMM0, Register.XMM15),
			new OpVsib(Register.XMM0, Register.XMM15),
			new OpVsib(Register.YMM0, Register.YMM15),
			new OpVsib(Register.YMM0, Register.YMM15),
			new OpModRM_rm(Register.EAX, Register.R15D),
			new OpModRM_rm(Register.RAX, Register.R15),
			new OpModRM_rm(Register.XMM0, Register.XMM15),
			new OpModRM_rm(Register.YMM0, Register.YMM15),
			new OpModRM_rm(Register.K0, Register.K7),
			new OpModRM_reg(Register.EAX, Register.R15D),
			new OpModRM_rm_reg_only(Register.EAX, Register.R15D),
			new OpHx(Register.EAX, Register.R15D),
			new OpModRM_reg(Register.RAX, Register.R15),
			new OpModRM_rm_reg_only(Register.RAX, Register.R15),
			new OpHx(Register.RAX, Register.R15),
			new OpModRM_reg(Register.K0, Register.K7),
			new OpModRM_rm_reg_only(Register.K0, Register.K7),
			new OpHx(Register.K0, Register.K7),
			new OpModRM_reg(Register.XMM0, Register.XMM15),
			new OpModRM_rm_reg_only(Register.XMM0, Register.XMM15),
			new OpHx(Register.XMM0, Register.XMM15),
			new OpIsX(Register.XMM0, Register.XMM15),
			new OpIsX(Register.XMM0, Register.XMM15),
			new OpModRM_reg(Register.YMM0, Register.YMM15),
			new OpModRM_rm_reg_only(Register.YMM0, Register.YMM15),
			new OpHx(Register.YMM0, Register.YMM15),
			new OpIsX(Register.YMM0, Register.YMM15),
			new OpIsX(Register.YMM0, Register.YMM15),
			new OpI4(),
			new OpIb(OpKind.Immediate8),
			new OprDI(),
			new OpModRM_rm_mem_only(true),
			new OpModRM_reg(Register.TMM0, Register.TMM7),
			new OpModRM_rm_reg_only(Register.TMM0, Register.TMM7),
			new OpHx(Register.TMM0, Register.TMM7),
		};
#endif
#if !NO_XOP
		public static readonly Op[] XopOps = new Op[18] {
			new OpModRM_rm(Register.EAX, Register.R15D),
			new OpModRM_rm(Register.RAX, Register.R15),
			new OpModRM_rm(Register.XMM0, Register.XMM15),
			new OpModRM_rm(Register.YMM0, Register.YMM15),
			new OpModRM_reg(Register.EAX, Register.R15D),
			new OpModRM_rm_reg_only(Register.EAX, Register.R15D),
			new OpHx(Register.EAX, Register.R15D),
			new OpModRM_reg(Register.RAX, Register.R15),
			new OpModRM_rm_reg_only(Register.RAX, Register.R15),
			new OpHx(Register.RAX, Register.R15),
			new OpModRM_reg(Register.XMM0, Register.XMM15),
			new OpHx(Register.XMM0, Register.XMM15),
			new OpIsX(Register.XMM0, Register.XMM15),
			new OpModRM_reg(Register.YMM0, Register.YMM15),
			new OpHx(Register.YMM0, Register.YMM15),
			new OpIsX(Register.YMM0, Register.YMM15),
			new OpIb(OpKind.Immediate8),
			new OpId(OpKind.Immediate32),
		};
#endif
#if !NO_EVEX
		public static readonly Op[] EvexOps = new Op[31] {
			new OpModRM_rm_mem_only(false),
			new OpVsib(Register.XMM0, Register.XMM31),
			new OpVsib(Register.XMM0, Register.XMM31),
			new OpVsib(Register.YMM0, Register.YMM31),
			new OpVsib(Register.YMM0, Register.YMM31),
			new OpVsib(Register.ZMM0, Register.ZMM31),
			new OpVsib(Register.ZMM0, Register.ZMM31),
			new OpModRM_rm(Register.EAX, Register.R15D),
			new OpModRM_rm(Register.RAX, Register.R15),
			new OpModRM_rm(Register.XMM0, Register.XMM31),
			new OpModRM_rm(Register.YMM0, Register.YMM31),
			new OpModRM_rm(Register.ZMM0, Register.ZMM31),
			new OpModRM_reg(Register.EAX, Register.R15D),
			new OpModRM_rm_reg_only(Register.EAX, Register.R15D),
			new OpModRM_reg(Register.RAX, Register.R15),
			new OpModRM_rm_reg_only(Register.RAX, Register.R15),
			new OpModRM_reg(Register.K0, Register.K7),
			new OpModRM_reg(Register.K0, Register.K7),
			new OpModRM_rm_reg_only(Register.K0, Register.K7),
			new OpModRM_reg(Register.XMM0, Register.XMM31),
			new OpModRM_rm_reg_only(Register.XMM0, Register.XMM31),
			new OpHx(Register.XMM0, Register.XMM31),
			new OpHx(Register.XMM0, Register.XMM31),
			new OpModRM_reg(Register.YMM0, Register.YMM31),
			new OpModRM_rm_reg_only(Register.YMM0, Register.YMM31),
			new OpHx(Register.YMM0, Register.YMM31),
			new OpModRM_reg(Register.ZMM0, Register.ZMM31),
			new OpModRM_rm_reg_only(Register.ZMM0, Register.ZMM31),
			new OpHx(Register.ZMM0, Register.ZMM31),
			new OpHx(Register.ZMM0, Register.ZMM31),
			new OpIb(OpKind.Immediate8),
		};
#endif
	}
}
#endif
