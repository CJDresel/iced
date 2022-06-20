// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

package com.github.icedland.iced.x86;

import java.util.HashMap;
import java.util.Iterator;

import com.github.icedland.iced.x86.fmt.CC_le;

public final class ToCC_le {
	static Integer tryGet(String key) {
		return map.get(key);
	}

	static Integer get(String key) {
		Integer value = tryGet(key);
		if (value == null)
			throw new UnsupportedOperationException(String.format("Couldn't find enum variant CC_le.%s", key));
		return value;
	}

	static Iterator<String> names() {
		return map.keySet().iterator();
	}

	static Iterator<Integer> values() {
		return map.values().iterator();
	}

	static int size() {
		return map.size();
	}

	static HashMap<String, Integer> copy() {
		return new HashMap<String, Integer>(map);
	}

	private static final HashMap<String, Integer> map = getMap();

	private static HashMap<String, Integer> getMap() {
		HashMap<String, Integer> map = new HashMap<String, Integer>(2);
		initMap0(map);
		return map;
	}

	private static void initMap0(HashMap<String, Integer> map) {
		map.put("le", CC_le.LE);
		map.put("ng", CC_le.NG);
	}
}
