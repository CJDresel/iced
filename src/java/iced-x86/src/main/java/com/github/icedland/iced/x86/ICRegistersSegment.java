// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

package com.github.icedland.iced.x86;

/**
 * Registers passed to {@code Instruction.create()} methods.
 *
 * @see ICRegisters
 * @see ICRegistersGPR
 * @see ICRegistersVec
 * @see ICRegistersGPR8
 * @see ICRegistersGPR16
 * @see ICRegistersGPR32
 * @see ICRegistersGPR64
 * @see ICRegistersIP
 * @see ICRegistersST
 * @see ICRegistersCR
 * @see ICRegistersDR
 * @see ICRegistersTR
 * @see ICRegistersBND
 * @see ICRegistersK
 * @see ICRegistersMM
 * @see ICRegistersXMM
 * @see ICRegistersYMM
 * @see ICRegistersZMM
 * @see ICRegistersTMM
 */
public final class ICRegistersSegment {
	private ICRegistersSegment() {
	}

	public static final ICRegister es = ICRegisters.es;
	public static final ICRegister cs = ICRegisters.cs;
	public static final ICRegister ss = ICRegisters.ss;
	public static final ICRegister ds = ICRegisters.ds;
	public static final ICRegister fs = ICRegisters.fs;
	public static final ICRegister gs = ICRegisters.gs;
}
