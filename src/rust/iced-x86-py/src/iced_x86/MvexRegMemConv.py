# SPDX-License-Identifier: MIT
# Copyright (C) 2018-present iced project and contributors

# ⚠️This file was generated by GENERATOR!🦹‍♂️

# pylint: disable=invalid-name
# pylint: disable=line-too-long
# pylint: disable=too-many-lines

"""
MVEX register/memory operand conversion
"""

import typing
if typing.TYPE_CHECKING:
	from ._iced_x86_py import MvexRegMemConv
else:
	MvexRegMemConv = int

NONE: MvexRegMemConv = 0 # type: ignore
"""
No operand conversion
"""
REG_SWIZZLE_NONE: MvexRegMemConv = 1 # type: ignore
"""
: Register swizzle: ``zmm0`` or ``zmm0 {dcba}``
"""
REG_SWIZZLE_CDAB: MvexRegMemConv = 2 # type: ignore
"""
: Register swizzle: ``zmm0 {cdab}``
"""
REG_SWIZZLE_BADC: MvexRegMemConv = 3 # type: ignore
"""
: Register swizzle: ``zmm0 {badc}``
"""
REG_SWIZZLE_DACB: MvexRegMemConv = 4 # type: ignore
"""
: Register swizzle: ``zmm0 {dacb}``
"""
REG_SWIZZLE_AAAA: MvexRegMemConv = 5 # type: ignore
"""
: Register swizzle: ``zmm0 {aaaa}``
"""
REG_SWIZZLE_BBBB: MvexRegMemConv = 6 # type: ignore
"""
: Register swizzle: ``zmm0 {bbbb}``
"""
REG_SWIZZLE_CCCC: MvexRegMemConv = 7 # type: ignore
"""
: Register swizzle: ``zmm0 {cccc}``
"""
REG_SWIZZLE_DDDD: MvexRegMemConv = 8 # type: ignore
"""
: Register swizzle: ``zmm0 {dddd}``
"""
MEM_CONV_NONE: MvexRegMemConv = 9 # type: ignore
"""
: Memory Up/DownConv: ``[rax]`` / ``zmm0``
"""
MEM_CONV_BROADCAST1: MvexRegMemConv = 10 # type: ignore
"""
: Memory UpConv: ``[rax] {1to16}`` or ``[rax] {1to8}``
"""
MEM_CONV_BROADCAST4: MvexRegMemConv = 11 # type: ignore
"""
: Memory UpConv: ``[rax] {4to16}`` or ``[rax] {4to8}``
"""
MEM_CONV_FLOAT16: MvexRegMemConv = 12 # type: ignore
"""
: Memory Up/DownConv: ``[rax] {float16}`` / ``zmm0 {float16}``
"""
MEM_CONV_UINT8: MvexRegMemConv = 13 # type: ignore
"""
: Memory Up/DownConv: ``[rax] {uint8}`` / ``zmm0 {uint8}``
"""
MEM_CONV_SINT8: MvexRegMemConv = 14 # type: ignore
"""
: Memory Up/DownConv: ``[rax] {sint8}`` / ``zmm0 {sint8}``
"""
MEM_CONV_UINT16: MvexRegMemConv = 15 # type: ignore
"""
: Memory Up/DownConv: ``[rax] {uint16}`` / ``zmm0 {uint16}``
"""
MEM_CONV_SINT16: MvexRegMemConv = 16 # type: ignore
"""
: Memory Up/DownConv: ``[rax] {sint16}`` / ``zmm0 {sint16}``
"""
