-- SPDX-License-Identifier: MIT
-- Copyright (C) 2018-present iced project and contributors

-- ⚠️This file was generated by GENERATOR!🦹‍♂️

---Decoder error
return {
	---No error. The last decoded instruction is a valid instruction
	None = 0,
	---It's an invalid instruction or an invalid encoding of an existing instruction (eg. some reserved bit is set/cleared)
	InvalidInstruction = 1,
	---There's not enough bytes left to decode the instruction
	NoMoreBytes = 2,
}
