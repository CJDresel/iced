// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

package com.github.icedland.iced.x86.internal.enc;

/**
 * DO NOT USE: INTERNAL API
 */
public final class EncoderFlags {
	private EncoderFlags() {
	}

	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int NONE = 0x0000_0000;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int B = 0x0000_0001;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int X = 0x0000_0002;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int R = 0x0000_0004;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int W = 0x0000_0008;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int MOD_RM = 0x0000_0010;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int SIB = 0x0000_0020;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int REX = 0x0000_0040;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int P66 = 0x0000_0080;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int P67 = 0x0000_0100;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int R2 = 0x0000_0200;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int BROADCAST = 0x0000_0400;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int HIGH_LEGACY_8_BIT_REGS = 0x0000_0800;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int DISPL = 0x0000_1000;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int PF0 = 0x0000_2000;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int REG_IS_MEMORY = 0x0000_4000;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int MUST_USE_SIB = 0x0000_8000;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int VVVVV_SHIFT = 0x0000_001B;
	/**
	 * DO NOT USE: INTERNAL API
	 */
	public static final int VVVVV_MASK = 0x0000_001F;
}