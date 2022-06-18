// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

package com.github.icedland.iced.x86.asm;

import com.github.icedland.iced.x86.ICRegister;
import com.github.icedland.iced.x86.Register;

/**
 * An assembler register used with {@link CodeAssembler}.
 */
public final class AsmRegisterXMM {
	/**
	 * Creates a new instance.
	 *
	 * @param register Register
	 */
	public AsmRegisterXMM(ICRegister register) {
		if (!Register.isXMM(register.get()))
			throw new IllegalArgumentException("Invalid register value. Must be a XMM register");
		this.register = register;
		this.flags = AsmOperandFlags.NONE;
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

	/**
	 * Creates a new instance.
	 *
	 * @param register Register
	 * @param flags Flags (an {@link AsmOperandFlags} flags value)
	 */
	public AsmRegisterXMM(ICRegister register, int flags) {
		if (!Register.isXMM(register.get()))
			throw new IllegalArgumentException("Invalid register value. Must be a XMM register");
		this.register = register;
		this.flags = flags;
	}

	final int flags;

	/**
	 * Apply op mask register <code>K1</code>.
	 */
	public AsmRegisterXMM k1() {
		return new AsmRegisterXMM(register, (flags & ~AsmOperandFlags.REGISTER_MASK) | AsmOperandFlags.K1);
	}

	/**
	 * Apply op mask register <code>K2</code>.
	 */
	public AsmRegisterXMM k2() {
		return new AsmRegisterXMM(register, (flags & ~AsmOperandFlags.REGISTER_MASK) | AsmOperandFlags.K2);
	}

	/**
	 * Apply op mask register <code>K3</code>.
	 */
	public AsmRegisterXMM k3() {
		return new AsmRegisterXMM(register, (flags & ~AsmOperandFlags.REGISTER_MASK) | AsmOperandFlags.K3);
	}

	/**
	 * Apply op mask register <code>K4</code>.
	 */
	public AsmRegisterXMM k4() {
		return new AsmRegisterXMM(register, (flags & ~AsmOperandFlags.REGISTER_MASK) | AsmOperandFlags.K4);
	}

	/**
	 * Apply op mask register <code>K5</code>.
	 */
	public AsmRegisterXMM k5() {
		return new AsmRegisterXMM(register, (flags & ~AsmOperandFlags.REGISTER_MASK) | AsmOperandFlags.K5);
	}

	/**
	 * Apply op mask register <code>K6</code>.
	 */
	public AsmRegisterXMM k6() {
		return new AsmRegisterXMM(register, (flags & ~AsmOperandFlags.REGISTER_MASK) | AsmOperandFlags.K6);
	}

	/**
	 * Apply op mask register <code>K7</code>.
	 */
	public AsmRegisterXMM k7() {
		return new AsmRegisterXMM(register, (flags & ~AsmOperandFlags.REGISTER_MASK) | AsmOperandFlags.K7);
	}

	/**
	 * Apply op mask zeroing.
	 */
	public AsmRegisterXMM z() {
		return new AsmRegisterXMM(register, flags | AsmOperandFlags.ZEROING);
	}

	/**
	 * Suppress all exceptions
	 */
	public AsmRegisterXMM sae() {
		return new AsmRegisterXMM(register, flags | AsmOperandFlags.SUPPRESS_ALL_EXCEPTIONS);
	}

	/**
	 * Round to nearest (even)
	 */
	public AsmRegisterXMM rn_sae() {
		return new AsmRegisterXMM(register, (flags & ~AsmOperandFlags.ROUNDING_CONTROL_MASK) | AsmOperandFlags.ROUND_TO_NEAREST);
	}

	/**
	 * Round down (toward -inf)
	 */
	public AsmRegisterXMM rd_sae() {
		return new AsmRegisterXMM(register, (flags & ~AsmOperandFlags.ROUNDING_CONTROL_MASK) | AsmOperandFlags.ROUND_DOWN);
	}

	/**
	 * Round up (toward +inf)
	 */
	public AsmRegisterXMM ru_sae() {
		return new AsmRegisterXMM(register, (flags & ~AsmOperandFlags.ROUNDING_CONTROL_MASK) | AsmOperandFlags.ROUND_UP);
	}

	/**
	 * Round toward zero (truncate)
	 */
	public AsmRegisterXMM rz_sae() {
		return new AsmRegisterXMM(register, (flags & ~AsmOperandFlags.ROUNDING_CONTROL_MASK) | AsmOperandFlags.ROUND_TOWARD_ZERO);
	}

	/**
	 * Adds this register (index) to a displacement and returns a memory operand.
	 *
	 * @param displacement The displacement
	 */
	public AsmMemoryOperand add(long displacement) {
		return new AsmMemoryOperand(MemoryOperandSize.NONE, ICRegister.NONE, ICRegister.NONE, get(), 1, displacement, AsmOperandFlags.NONE);
	}

	/**
	 * Subtracts a displacement from this register (index) and returns a memory operand.
	 *
	 * @param displacement The displacement
	 */
	public AsmMemoryOperand sub(long displacement) {
		return new AsmMemoryOperand(MemoryOperandSize.NONE, ICRegister.NONE, ICRegister.NONE, get(), 1, -displacement, AsmOperandFlags.NONE);
	}

	/**
	 * Multiplies an index register by a scale and returns a memory operand.
	 *
	 * @param scale The scale (1, 2, 4 or 8)
	 */
	public AsmMemoryOperand scale(int scale) {
		return new AsmMemoryOperand(MemoryOperandSize.NONE, ICRegister.NONE, ICRegister.NONE, get(), scale, 0, AsmOperandFlags.NONE);
	}

	/**
	 * Adds this register (index) to a base register and returns a memory operand.
	 *
	 * @param base Base register
	 */
	public AsmMemoryOperand add(AsmRegister32 base) {
		return new AsmMemoryOperand(MemoryOperandSize.NONE, ICRegister.NONE, base.get(), get(), 1, 0, AsmOperandFlags.NONE);
	}

	/**
	 * Adds this register (index) to a base register and returns a memory operand.
	 *
	 * @param base Base register
	 */
	public AsmMemoryOperand add(AsmRegister64 base) {
		return new AsmMemoryOperand(MemoryOperandSize.NONE, ICRegister.NONE, base.get(), get(), 1, 0, AsmOperandFlags.NONE);
	}

	/**
	 * Adds this register (index) to a memory operand and returns a memory operand.
	 *
	 * @param mem Memory operand
	 */
	public AsmMemoryOperand add(AsmMemoryOperand mem) {
		return mem.add(this);
	}

	/** Checks if <code>obj</code> equals this object */
	@Override
	public boolean equals(Object obj) {
		if (obj == null || getClass() != obj.getClass())
			return false;
		AsmRegisterXMM other = (AsmRegisterXMM)obj;
		return register.get() == other.register.get() && flags == other.flags;
	}

	/** Gets the hash code */
	@Override
	public int hashCode() {
		return (register.get() * 397) ^ flags;
	}

	/** toString() */
	@Override
	public String toString() {
		return String.format("Register %d, flags: %d", getRegister(), flags);
	}
}
