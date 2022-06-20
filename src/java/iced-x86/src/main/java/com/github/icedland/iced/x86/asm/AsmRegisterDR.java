// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

package com.github.icedland.iced.x86.asm;

import com.github.icedland.iced.x86.ICRegister;
import com.github.icedland.iced.x86.Register;

/**
 * An assembler register used with {@link CodeAssembler}.
 */
public final class AsmRegisterDR {
	/**
	 * Creates a new instance.
	 *
	 * @param register Register
	 */
	public AsmRegisterDR(ICRegister register) {
		if (!Register.isDR(register.get()))
			throw new IllegalArgumentException("Invalid register value. Must be a DR register");
		this.register = register;
	}

	private final ICRegister register;

	/**
	 * The register value
	 */
	public ICRegister get() {
		return register;
	}

	/**
	 * The register value (a {@link com.github.icedland.iced.x86.Register} enum variant)
	 */
	public int getRegister() {
		return register.get();
	}

	/** Checks if {@code obj} equals this object */
	@Override
	public boolean equals(Object obj) {
		if (obj == null || getClass() != obj.getClass())
			return false;
		AsmRegisterDR other = (AsmRegisterDR)obj;
		return register.get() == other.register.get();
	}

	/** Gets the hash code */
	@Override
	public int hashCode() {
		return register.get();
	}

	/** toString() */
	@Override
	public String toString() {
		return String.format("Register %d", getRegister());
	}
}
