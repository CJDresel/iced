// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

package com.github.icedland.iced.x86.fmt;

/**
 * Mnemonic condition code selector (eg.<!-- --> {@code JB} / {@code JC} / {@code JNAE})
 */
public final class CC_b {
	private CC_b() {
	}

	/**
	 * {@code JB}, {@code CMOVB}, {@code SETB}
	 */
	public static final int B = 0;
	/**
	 * {@code JC}, {@code CMOVC}, {@code SETC}
	 */
	public static final int C = 1;
	/**
	 * {@code JNAE}, {@code CMOVNAE}, {@code SETNAE}
	 */
	public static final int NAE = 2;
}
