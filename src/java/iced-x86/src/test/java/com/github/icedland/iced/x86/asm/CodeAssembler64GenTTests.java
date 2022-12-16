// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

package com.github.icedland.iced.x86.asm;

import org.junit.jupiter.api.Test;

import com.github.icedland.iced.x86.*;
import static com.github.icedland.iced.x86.asm.AsmRegisters.*;

final class CodeAssembler64GenTTests extends CodeAssemblerTestsBase {
	CodeAssembler64GenTTests() {
		super(64);
	}

	@Test
	void t1mskc_r32_r32() {
		testAssembler(c -> c.t1mskc(edx, ebx), Instruction.create(Code.XOP_T1MSKC_R32_RM32, ICRegisters.edx, ICRegisters.ebx));
	}

	@Test
	void t1mskc_r64_r64() {
		testAssembler(c -> c.t1mskc(rdx, rbx), Instruction.create(Code.XOP_T1MSKC_R64_RM64, ICRegisters.rdx, ICRegisters.rbx));
	}

	@Test
	void t1mskc_r32_m() {
		testAssembler(c -> c.t1mskc(edx, dword_ptr(0x0L).base(rcx)), Instruction.create(Code.XOP_T1MSKC_R32_RM32, ICRegisters.edx, new MemoryOperand(ICRegisters.rcx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE)));
	}

	@Test
	void t1mskc_r64_m() {
		testAssembler(c -> c.t1mskc(rdx, qword_ptr(0x0L).base(rcx)), Instruction.create(Code.XOP_T1MSKC_R64_RM64, ICRegisters.rdx, new MemoryOperand(ICRegisters.rcx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE)));
	}

	@Test
	void tdcall() {
		testAssembler(c -> c.tdcall(), Instruction.create(Code.TDCALL));
	}

	@Test
	void tdpbf16ps_tmm_tmm_tmm() {
		testAssembler(c -> c.tdpbf16ps(tmm2, tmm3, tmm4), Instruction.create(Code.VEX_TDPBF16PS_TMM_TMM_TMM, ICRegisters.tmm2, ICRegisters.tmm3, ICRegisters.tmm4));
	}

	@Test
	void tdpbssd_tmm_tmm_tmm() {
		testAssembler(c -> c.tdpbssd(tmm2, tmm3, tmm4), Instruction.create(Code.VEX_TDPBSSD_TMM_TMM_TMM, ICRegisters.tmm2, ICRegisters.tmm3, ICRegisters.tmm4));
	}

	@Test
	void tdpbsud_tmm_tmm_tmm() {
		testAssembler(c -> c.tdpbsud(tmm2, tmm3, tmm4), Instruction.create(Code.VEX_TDPBSUD_TMM_TMM_TMM, ICRegisters.tmm2, ICRegisters.tmm3, ICRegisters.tmm4));
	}

	@Test
	void tdpbusd_tmm_tmm_tmm() {
		testAssembler(c -> c.tdpbusd(tmm2, tmm3, tmm4), Instruction.create(Code.VEX_TDPBUSD_TMM_TMM_TMM, ICRegisters.tmm2, ICRegisters.tmm3, ICRegisters.tmm4));
	}

	@Test
	void tdpbuud_tmm_tmm_tmm() {
		testAssembler(c -> c.tdpbuud(tmm2, tmm3, tmm4), Instruction.create(Code.VEX_TDPBUUD_TMM_TMM_TMM, ICRegisters.tmm2, ICRegisters.tmm3, ICRegisters.tmm4));
	}

	@Test
	void tdpfp16ps_tmm_tmm_tmm() {
		testAssembler(c -> c.tdpfp16ps(tmm2, tmm3, tmm4), Instruction.create(Code.VEX_TDPFP16PS_TMM_TMM_TMM, ICRegisters.tmm2, ICRegisters.tmm3, ICRegisters.tmm4));
	}

	@Test
	void test_r8_r8() {
		testAssembler(c -> c.test(dl, bl), Instruction.create(Code.TEST_RM8_R8, ICRegisters.dl, ICRegisters.bl));
	}

	@Test
	void test_m_r8() {
		testAssembler(c -> c.test(byte_ptr(0x0L).base(rcx), bl), Instruction.create(Code.TEST_RM8_R8, new MemoryOperand(ICRegisters.rcx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE), ICRegisters.bl));
	}

	@Test
	void test_r16_r16() {
		testAssembler(c -> c.test(dx, bx), Instruction.create(Code.TEST_RM16_R16, ICRegisters.dx, ICRegisters.bx));
	}

	@Test
	void test_m_r16() {
		testAssembler(c -> c.test(word_ptr(0x0L).base(rcx), bx), Instruction.create(Code.TEST_RM16_R16, new MemoryOperand(ICRegisters.rcx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE), ICRegisters.bx));
	}

	@Test
	void test_r32_r32() {
		testAssembler(c -> c.test(edx, ebx), Instruction.create(Code.TEST_RM32_R32, ICRegisters.edx, ICRegisters.ebx));
	}

	@Test
	void test_m_r32() {
		testAssembler(c -> c.test(dword_ptr(0x0L).base(rcx), ebx), Instruction.create(Code.TEST_RM32_R32, new MemoryOperand(ICRegisters.rcx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE), ICRegisters.ebx));
	}

	@Test
	void test_r64_r64() {
		testAssembler(c -> c.test(rdx, rbx), Instruction.create(Code.TEST_RM64_R64, ICRegisters.rdx, ICRegisters.rbx));
	}

	@Test
	void test_m_r64() {
		testAssembler(c -> c.test(qword_ptr(0x0L).base(rcx), rbx), Instruction.create(Code.TEST_RM64_R64, new MemoryOperand(ICRegisters.rcx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE), ICRegisters.rbx));
	}

	@Test
	void test_r8_i() {
		{ /* if (dst.getRegister() == Register.AL) */
			testAssembler(c -> c.test(al, -5), Instruction.create(Code.TEST_AL_IMM8, ICRegisters.al, -5));
		} /* else */ testAssembler(c -> c.test(dl, -5), Instruction.create(Code.TEST_RM8_IMM8, ICRegisters.dl, -5));
	}

	@Test
	void test_r16_i() {
		{ /* if (dst.getRegister() == Register.AX) */
			testAssembler(c -> c.test(ax, 0x40B7), Instruction.create(Code.TEST_AX_IMM16, ICRegisters.ax, 0x40B7));
		} /* else */ testAssembler(c -> c.test(dx, 0x40B7), Instruction.create(Code.TEST_RM16_IMM16, ICRegisters.dx, 0x40B7));
	}

	@Test
	void test_r32_i() {
		{ /* if (dst.getRegister() == Register.EAX) */
			testAssembler(c -> c.test(eax, 0x7FFFFFFF), Instruction.create(Code.TEST_EAX_IMM32, ICRegisters.eax, 0x7FFFFFFF));
		} /* else */ testAssembler(c -> c.test(edx, 0x7FFFFFFF), Instruction.create(Code.TEST_RM32_IMM32, ICRegisters.edx, 0x7FFFFFFF));
	}

	@Test
	void test_r64_i() {
		{ /* if (dst.getRegister() == Register.RAX) */
			testAssembler(c -> c.test(rax, -0x80000000), Instruction.create(Code.TEST_RAX_IMM32, ICRegisters.rax, -0x80000000));
		} /* else */ testAssembler(c -> c.test(rdx, -0x80000000), Instruction.create(Code.TEST_RM64_IMM32, ICRegisters.rdx, -0x80000000));
	}

	@Test
	void test_m_i() {
		{ /* if (dst.size == MemoryOperandSize.QWORD) */
			testAssembler(c -> c.test(qword_ptr(0x0L).base(rdx), -0x80000000), Instruction.create(Code.TEST_RM64_IMM32, new MemoryOperand(ICRegisters.rdx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE), -0x80000000));
		} /* else */ { /* if (dst.size == MemoryOperandSize.DWORD) */
			testAssembler(c -> c.test(dword_ptr(0x0L).base(rdx), 0x7FFFFFFF), Instruction.create(Code.TEST_RM32_IMM32, new MemoryOperand(ICRegisters.rdx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE), 0x7FFFFFFF));
		} /* else */ { /* if (dst.size == MemoryOperandSize.WORD) */
			testAssembler(c -> c.test(word_ptr(0x0L).base(rdx), 0x40B7), Instruction.create(Code.TEST_RM16_IMM16, new MemoryOperand(ICRegisters.rdx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE), 0x40B7));
		} /* else */ { /* if (dst.size == MemoryOperandSize.BYTE) */
			testAssembler(c -> c.test(byte_ptr(0x0L).base(rdx), -5), Instruction.create(Code.TEST_RM8_IMM8, new MemoryOperand(ICRegisters.rdx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE), -5));
		}
		{
			assertInvalid(() -> {
				testAssembler(c -> c.test(zmmword_ptr(0x0L).base(rdx), -5), Instruction.create(Code.TEST_RM8_IMM8, new MemoryOperand(ICRegisters.rdx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE), -5));
			});
		}
	}

	@Test
	void testui() {
		testAssembler(c -> c.testui(), Instruction.create(Code.TESTUI));
	}

	@Test
	void tileloadd_tmm_m() {
		testAssembler(c -> c.tileloadd(tmm2, mem_ptr(0x0L).base(rcx).index(rdx).scale(4)), Instruction.create(Code.VEX_TILELOADD_TMM_SIBMEM, ICRegisters.tmm2, new MemoryOperand(ICRegisters.rcx, ICRegisters.rdx, 4, 0x0L, 0, false, ICRegister.NONE)));
	}

	@Test
	void tileloaddt1_tmm_m() {
		testAssembler(c -> c.tileloaddt1(tmm2, mem_ptr(0x0L).base(rcx).index(rdx).scale(4)), Instruction.create(Code.VEX_TILELOADDT1_TMM_SIBMEM, ICRegisters.tmm2, new MemoryOperand(ICRegisters.rcx, ICRegisters.rdx, 4, 0x0L, 0, false, ICRegister.NONE)));
	}

	@Test
	void tilerelease() {
		testAssembler(c -> c.tilerelease(), Instruction.create(Code.VEX_TILERELEASE));
	}

	@Test
	void tilestored_m_tmm() {
		testAssembler(c -> c.tilestored(mem_ptr(0x0L).base(rcx).index(rdx).scale(4), tmm3), Instruction.create(Code.VEX_TILESTORED_SIBMEM_TMM, new MemoryOperand(ICRegisters.rcx, ICRegisters.rdx, 4, 0x0L, 0, false, ICRegister.NONE), ICRegisters.tmm3));
	}

	@Test
	void tilezero_tmm() {
		testAssembler(c -> c.tilezero(tmm2), Instruction.create(Code.VEX_TILEZERO_TMM, ICRegisters.tmm2));
	}

	@Test
	void tlbsync() {
		testAssembler(c -> c.tlbsync(), Instruction.create(Code.TLBSYNC));
	}

	@Test
	void tpause_r32() {
		testAssembler(c -> c.tpause(edx), Instruction.create(Code.TPAUSE_R32, ICRegisters.edx));
	}

	@Test
	void tpause_r64() {
		testAssembler(c -> c.tpause(rdx), Instruction.create(Code.TPAUSE_R64, ICRegisters.rdx));
	}

	@Test
	void tzcnt_r16_r16() {
		testAssembler(c -> c.tzcnt(dx, bx), Instruction.create(Code.TZCNT_R16_RM16, ICRegisters.dx, ICRegisters.bx));
	}

	@Test
	void tzcnt_r32_r32() {
		testAssembler(c -> c.tzcnt(edx, ebx), Instruction.create(Code.TZCNT_R32_RM32, ICRegisters.edx, ICRegisters.ebx));
	}

	@Test
	void tzcnt_r64_r64() {
		testAssembler(c -> c.tzcnt(rdx, rbx), Instruction.create(Code.TZCNT_R64_RM64, ICRegisters.rdx, ICRegisters.rbx));
	}

	@Test
	void tzcnt_r16_m() {
		testAssembler(c -> c.tzcnt(dx, word_ptr(0x0L).base(rcx)), Instruction.create(Code.TZCNT_R16_RM16, ICRegisters.dx, new MemoryOperand(ICRegisters.rcx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE)));
	}

	@Test
	void tzcnt_r32_m() {
		testAssembler(c -> c.tzcnt(edx, dword_ptr(0x0L).base(rcx)), Instruction.create(Code.TZCNT_R32_RM32, ICRegisters.edx, new MemoryOperand(ICRegisters.rcx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE)));
	}

	@Test
	void tzcnt_r64_m() {
		testAssembler(c -> c.tzcnt(rdx, qword_ptr(0x0L).base(rcx)), Instruction.create(Code.TZCNT_R64_RM64, ICRegisters.rdx, new MemoryOperand(ICRegisters.rcx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE)));
	}

	@Test
	void tzmsk_r32_r32() {
		testAssembler(c -> c.tzmsk(edx, ebx), Instruction.create(Code.XOP_TZMSK_R32_RM32, ICRegisters.edx, ICRegisters.ebx));
	}

	@Test
	void tzmsk_r64_r64() {
		testAssembler(c -> c.tzmsk(rdx, rbx), Instruction.create(Code.XOP_TZMSK_R64_RM64, ICRegisters.rdx, ICRegisters.rbx));
	}

	@Test
	void tzmsk_r32_m() {
		testAssembler(c -> c.tzmsk(edx, dword_ptr(0x0L).base(rcx)), Instruction.create(Code.XOP_TZMSK_R32_RM32, ICRegisters.edx, new MemoryOperand(ICRegisters.rcx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE)));
	}

	@Test
	void tzmsk_r64_m() {
		testAssembler(c -> c.tzmsk(rdx, qword_ptr(0x0L).base(rcx)), Instruction.create(Code.XOP_TZMSK_R64_RM64, ICRegisters.rdx, new MemoryOperand(ICRegisters.rcx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE)));
	}

	@Test
	void vtestpd_xmm_xmm() {
		testAssembler(c -> c.vtestpd(xmm2, xmm3), Instruction.create(Code.VEX_VTESTPD_XMM_XMMM128, ICRegisters.xmm2, ICRegisters.xmm3));
	}

	@Test
	void vtestpd_ymm_ymm() {
		testAssembler(c -> c.vtestpd(ymm2, ymm3), Instruction.create(Code.VEX_VTESTPD_YMM_YMMM256, ICRegisters.ymm2, ICRegisters.ymm3));
	}

	@Test
	void vtestpd_xmm_m() {
		testAssembler(c -> c.vtestpd(xmm2, xmmword_ptr(0x0L).base(rcx)), Instruction.create(Code.VEX_VTESTPD_XMM_XMMM128, ICRegisters.xmm2, new MemoryOperand(ICRegisters.rcx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE)));
	}

	@Test
	void vtestpd_ymm_m() {
		testAssembler(c -> c.vtestpd(ymm2, ymmword_ptr(0x0L).base(rcx)), Instruction.create(Code.VEX_VTESTPD_YMM_YMMM256, ICRegisters.ymm2, new MemoryOperand(ICRegisters.rcx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE)));
	}

	@Test
	void vtestps_xmm_xmm() {
		testAssembler(c -> c.vtestps(xmm2, xmm3), Instruction.create(Code.VEX_VTESTPS_XMM_XMMM128, ICRegisters.xmm2, ICRegisters.xmm3));
	}

	@Test
	void vtestps_ymm_ymm() {
		testAssembler(c -> c.vtestps(ymm2, ymm3), Instruction.create(Code.VEX_VTESTPS_YMM_YMMM256, ICRegisters.ymm2, ICRegisters.ymm3));
	}

	@Test
	void vtestps_xmm_m() {
		testAssembler(c -> c.vtestps(xmm2, xmmword_ptr(0x0L).base(rcx)), Instruction.create(Code.VEX_VTESTPS_XMM_XMMM128, ICRegisters.xmm2, new MemoryOperand(ICRegisters.rcx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE)));
	}

	@Test
	void vtestps_ymm_m() {
		testAssembler(c -> c.vtestps(ymm2, ymmword_ptr(0x0L).base(rcx)), Instruction.create(Code.VEX_VTESTPS_YMM_YMMM256, ICRegisters.ymm2, new MemoryOperand(ICRegisters.rcx, ICRegister.NONE, 1, 0x0L, 0, false, ICRegister.NONE)));
	}

}
