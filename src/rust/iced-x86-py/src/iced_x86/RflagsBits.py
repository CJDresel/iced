# SPDX-License-Identifier: MIT
# Copyright (C) 2018-present iced project and contributors

# ⚠️This file was generated by GENERATOR!🦹‍♂️

# pylint: disable=invalid-name
# pylint: disable=line-too-long
# pylint: disable=too-many-lines

"""
``RFLAGS`` bits, FPU condition code bits and misc bits (``UIF``) supported by the instruction info code
"""
NONE: int = 0x0000_0000
"""
No bit is set
"""
OF: int = 0x0000_0001
"""
``RFLAGS.OF``
"""
SF: int = 0x0000_0002
"""
``RFLAGS.SF``
"""
ZF: int = 0x0000_0004
"""
``RFLAGS.ZF``
"""
AF: int = 0x0000_0008
"""
``RFLAGS.AF``
"""
CF: int = 0x0000_0010
"""
``RFLAGS.CF``
"""
PF: int = 0x0000_0020
"""
``RFLAGS.PF``
"""
DF: int = 0x0000_0040
"""
``RFLAGS.DF``
"""
IF: int = 0x0000_0080
"""
``RFLAGS.IF``
"""
AC: int = 0x0000_0100
"""
``RFLAGS.AC``
"""
UIF: int = 0x0000_0200
"""
``UIF``
"""
C0: int = 0x0000_0400
"""
FPU status word bit ``C0``
"""
C1: int = 0x0000_0800
"""
FPU status word bit ``C1``
"""
C2: int = 0x0000_1000
"""
FPU status word bit ``C2``
"""
C3: int = 0x0000_2000
"""
FPU status word bit ``C3``
"""
