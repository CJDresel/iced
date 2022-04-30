-- SPDX-License-Identifier: MIT
-- Copyright (C) 2018-present iced project and contributors

-- ⚠️This file was generated by GENERATOR!🦹‍♂️

--- MVEX register/memory operand conversion
return {
	--- No operand conversion
	None = 0,
	--- Register swizzle: `zmm0` or `zmm0 {dcba}`
	RegSwizzleNone = 1,
	--- Register swizzle: `zmm0 {cdab}`
	RegSwizzleCdab = 2,
	--- Register swizzle: `zmm0 {badc}`
	RegSwizzleBadc = 3,
	--- Register swizzle: `zmm0 {dacb}`
	RegSwizzleDacb = 4,
	--- Register swizzle: `zmm0 {aaaa}`
	RegSwizzleAaaa = 5,
	--- Register swizzle: `zmm0 {bbbb}`
	RegSwizzleBbbb = 6,
	--- Register swizzle: `zmm0 {cccc}`
	RegSwizzleCccc = 7,
	--- Register swizzle: `zmm0 {dddd}`
	RegSwizzleDddd = 8,
	--- Memory Up/DownConv: `[rax]` / `zmm0`
	MemConvNone = 9,
	--- Memory UpConv: `[rax] {1to16}` or `[rax] {1to8}`
	MemConvBroadcast1 = 10,
	--- Memory UpConv: `[rax] {4to16}` or `[rax] {4to8}`
	MemConvBroadcast4 = 11,
	--- Memory Up/DownConv: `[rax] {float16}` / `zmm0 {float16}`
	MemConvFloat16 = 12,
	--- Memory Up/DownConv: `[rax] {uint8}` / `zmm0 {uint8}`
	MemConvUint8 = 13,
	--- Memory Up/DownConv: `[rax] {sint8}` / `zmm0 {sint8}`
	MemConvSint8 = 14,
	--- Memory Up/DownConv: `[rax] {uint16}` / `zmm0 {uint16}`
	MemConvUint16 = 15,
	--- Memory Up/DownConv: `[rax] {sint16}` / `zmm0 {sint16}`
	MemConvSint16 = 16,
}
