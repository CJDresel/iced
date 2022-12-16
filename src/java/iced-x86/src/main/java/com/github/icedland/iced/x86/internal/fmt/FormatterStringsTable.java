// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

package com.github.icedland.iced.x86.internal.fmt;

import com.github.icedland.iced.x86.internal.ResourceReader;

/** DO NOT USE: INTERNAL API */
public final class FormatterStringsTable {
	// The returned array isn't cached since only one formatter is normally used
	@SuppressWarnings("deprecation")
	public static String[] getStringsTable() {
		com.github.icedland.iced.x86.internal.DataReader reader = new com.github.icedland.iced.x86.internal.DataReader(getSerializedStrings(),
				MAX_STRING_LENGTH);
		String[] strings = new String[STRINGS_COUNT];
		for (int i = 0; i < strings.length; i++)
			strings[i] = reader.readAsciiString();
		if (reader.canRead())
			throw new UnsupportedOperationException();
		return strings;
	}

	private static byte[] getSerializedStrings() {
		return ResourceReader.readByteArray(FormatterStringsTable.class.getClassLoader(),
				"com/github/icedland/iced/x86/fmt/FormatterStringsTable.bin");
	}

	// GENERATOR-BEGIN: Table
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	/** DO NOT USE: INTERNAL API */
	public static final int MAX_STRING_LENGTH = 18;
	/** DO NOT USE: INTERNAL API */
	public static final int STRINGS_COUNT = 1730;
	// GENERATOR-END: Table
}
