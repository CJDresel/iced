# SPDX-License-Identifier: MIT
# Copyright (C) 2018-present iced project and contributors

# ⚠️This file was generated by GENERATOR!🦹‍♂️

# pylint: disable=invalid-name
# pylint: disable=line-too-long
# pylint: disable=too-many-lines

"""
Mnemonic condition code selector (eg. ``JAE`` / ``JNB`` / ``JNC``)
"""
AE: int = 0
"""
``JAE``, ``CMOVAE``, ``SETAE``
"""
NB: int = 1
"""
``JNB``, ``CMOVNB``, ``SETNB``
"""
NC: int = 2
"""
``JNC``, ``CMOVNC``, ``SETNC``
"""
