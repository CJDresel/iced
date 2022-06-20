// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

package com.github.icedland.iced.x86;

import java.util.HashMap;
import java.util.Iterator;

public final class ToMvexConvFn {
	static Integer tryGet(String key) {
		return map.get(key);
	}

	static Integer get(String key) {
		Integer value = tryGet(key);
		if (value == null)
			throw new UnsupportedOperationException(String.format("Couldn't find enum variant MvexConvFn.%s", key));
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
		HashMap<String, Integer> map = new HashMap<String, Integer>(13);
		initMap0(map);
		return map;
	}

	private static void initMap0(HashMap<String, Integer> map) {
		map.put("None", MvexConvFn.NONE);
		map.put("Sf32", MvexConvFn.SF32);
		map.put("Sf64", MvexConvFn.SF64);
		map.put("Si32", MvexConvFn.SI32);
		map.put("Si64", MvexConvFn.SI64);
		map.put("Uf32", MvexConvFn.UF32);
		map.put("Uf64", MvexConvFn.UF64);
		map.put("Ui32", MvexConvFn.UI32);
		map.put("Ui64", MvexConvFn.UI64);
		map.put("Df32", MvexConvFn.DF32);
		map.put("Df64", MvexConvFn.DF64);
		map.put("Di32", MvexConvFn.DI32);
		map.put("Di64", MvexConvFn.DI64);
	}
}
