// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

package com.github.icedland.iced.x86.info;

import com.github.icedland.iced.x86.CodeSize;
import com.github.icedland.iced.x86.Register;
import com.github.icedland.iced.x86.VAGetRegisterValue;

/**
 * A memory location used by an instruction
 */
public final class UsedMemory {
	private final long displ;
	private final byte segReg;
	private final byte baseReg;
	private final byte indexReg;
	private final byte memorySize;
	private final byte scale;
	private final byte access;
	private final byte addressSize;
	private final byte vsibSize;

	/**
	 * Effective segment register (a {@link com.github.icedland.iced.x86.Register} enum variant) or {@link com.github.icedland.iced.x86.Register#NONE}
	 * if the segment register is ignored
	 */
	@SuppressWarnings("deprecation")
	public int getSegment() {
		return segReg & com.github.icedland.iced.x86.internal.Constants.REG_MASK;
	}

	/**
	 * Base register (a {@link com.github.icedland.iced.x86.Register} enum variant) or {@link com.github.icedland.iced.x86.Register#NONE} if none
	 */
	@SuppressWarnings("deprecation")
	public int getBase() {
		return baseReg & com.github.icedland.iced.x86.internal.Constants.REG_MASK;
	}

	/**
	 * Index register (a {@link com.github.icedland.iced.x86.Register} enum variant) or {@link com.github.icedland.iced.x86.Register#NONE} if none
	 */
	@SuppressWarnings("deprecation")
	public int getIndex() {
		return indexReg & com.github.icedland.iced.x86.internal.Constants.REG_MASK;
	}

	/**
	 * Index scale (1, 2, 4 or 8)
	 */
	public int getScale() {
		return scale;
	}

	/**
	 * Displacement
	 */
	public long getDisplacement() {
		return displ;
	}

	/**
	 * Size of location (a {@link com.github.icedland.iced.x86.MemorySize} enum variant)
	 */
	public int getMemorySize() {
		return memorySize;
	}

	/**
	 * Memory access (an {@link com.github.icedland.iced.x86.info.OpAccess} enum variant)
	 */
	public int getAccess() {
		return access;
	}

	/**
	 * Address size (a {@link com.github.icedland.iced.x86.CodeSize} enum variant)
	 */
	public int getAddressSize() {
		return addressSize;
	}

	/**
	 * VSIB size (<code>0</code>, <code>4</code> or <code>8</code>)
	 */
	public int getVsibSize() {
		return vsibSize;
	}

	/**
	 * Returns a copy of this instance
	 */
	public UsedMemory copy() {
		return new UsedMemory(getSegment(), getBase(), getIndex(), getScale(), getDisplacement(), getMemorySize(), getAccess(), getAddressSize(),
				getVsibSize());
	}

	@SuppressWarnings("deprecation")
	UsedMemory(int segReg, int baseReg, int indexReg, int scale, long displ, int memorySize, int access, int addressSize, int vsibSize) {
		assert Integer.compareUnsigned(segReg, com.github.icedland.iced.x86.internal.Constants.REG_MASK) <= 0 : segReg;
		assert Integer.compareUnsigned(baseReg, com.github.icedland.iced.x86.internal.Constants.REG_MASK) <= 0 : baseReg;
		assert Integer.compareUnsigned(indexReg, com.github.icedland.iced.x86.internal.Constants.REG_MASK) <= 0 : indexReg;
		this.displ = displ;
		assert Integer.compareUnsigned(segReg, 0xFF) <= 0 : segReg;
		this.segReg = (byte)segReg;
		assert Integer.compareUnsigned(baseReg, 0xFF) <= 0 : baseReg;
		this.baseReg = (byte)baseReg;
		assert Integer.compareUnsigned(indexReg, 0xFF) <= 0 : indexReg;
		this.indexReg = (byte)indexReg;
		assert Integer.compareUnsigned(memorySize, 0xFF) <= 0 : memorySize;
		this.memorySize = (byte)memorySize;
		assert scale == 1 || scale == 2 || scale == 4 || scale == 8 : scale;
		this.scale = (byte)scale;
		assert Integer.compareUnsigned(access, 0xFF) <= 0 : access;
		this.access = (byte)access;
		assert Integer.compareUnsigned(addressSize, 0xFF) <= 0 : addressSize;
		this.addressSize = (byte)addressSize;
		assert vsibSize == 0 || vsibSize == 4 || vsibSize == 8 : vsibSize;
		this.vsibSize = (byte)vsibSize;
	}

	/**
	 * Gets the virtual address of a memory operand
	 *
	 * @param elementIndex     Only used if it's a vsib memory operand. This is the element index of the vector index register.
	 * @param getRegisterValue Returns values of registers and segment base addresses
	 * @return <code>null</code> if it failed to read all registers, else the calculated virtual address
	 */
	public Long getVirtualAddress(int elementIndex, VAGetRegisterValue getRegisterValue) {
		if (getRegisterValue == null)
			throw new NullPointerException("getRegisterValue");

		long tmpAddr = getDisplacement();
		Long tmpValue;

		int reg = getBase();
		if (reg != Register.NONE) {
			if ((tmpValue = getRegisterValue.get(reg, 0, 0)) == null)
				return null;
			tmpAddr += tmpValue.longValue();
		}

		reg = getIndex();
		if (reg != Register.NONE) {
			if ((tmpValue = getRegisterValue.get(reg, elementIndex, vsibSize)) == null)
				return null;
			long v = tmpValue.longValue();
			if (vsibSize == 4)
				v &= 0xFFFF_FFFF;
			tmpAddr += v * getScale();
		}

		switch (getAddressSize()) {
		case CodeSize.CODE16:
			tmpAddr &= 0xFFFF;
			break;
		case CodeSize.CODE32:
			tmpAddr &= 0xFFFF_FFFF;
			break;
		}

		reg = getSegment();
		if (reg != Register.NONE) {
			if ((tmpValue = getRegisterValue.get(reg, 0, 0)) == null)
				return null;
			tmpAddr += tmpValue.longValue();
		}

		return tmpAddr;
	}
}
