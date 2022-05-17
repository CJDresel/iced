-- SPDX-License-Identifier: MIT
-- Copyright (C) 2018-present iced project and contributors

local from_hex = require("iced_test_utils").from_hex

describe("Encoder", function()
	local Decoder = require("iced_x86.Decoder")
	local Encoder = require("iced_x86.Encoder")

	it("new", function()
		local _ = Encoder:new(64)
		local _ = Encoder:new(64, 0)
		local _ = Encoder:new(64, 1234)
	end)

	it("bitness", function()
		for _, bitness in ipairs({ 16, 32, 64 }) do
			local encoder = Encoder:new(bitness)
			assert.equals(bitness, encoder:bitness())
		end
	end)

	it("invalid bitness", function()
		assert.has_error(function()
			local _ = Encoder:new(1)
		end)
		assert.has_error(function()
			local _ = Encoder:new(-0x80000001)
		end)
		assert.has_error(function()
			local _ = Encoder:new(0x100000000)
		end)
	end)

	it("take_buffer: empty", function()
		local encoder = Encoder:new(64)
		local buffer = encoder:take_buffer()
		assert.equals("", buffer)
	end)

	it("write_u8", function()
		local encoder = Encoder:new(64)
		encoder:write_u8(-0x80)
		encoder:write_u8(0xFF)
		encoder:write_u8(0xCC)
		encoder:write_u8(0x90)
		local buffer = encoder:take_buffer()
		local buffer2 = encoder:take_buffer()
		assert.equals("\128\255\204\144", buffer)
		assert.equals("", buffer2)
	end)

	it("write_u8: error", function()
		local encoder = Encoder:new(64)
		assert.has_error(function()
			encoder:write_u8(-0x81)
		end)
		assert.has_error(function()
			encoder:write_u8(0x101)
		end)
	end)

	it("get_constant_offsets", function()
		local encoder = Encoder:new(64)
		assert.has_error(function()
			encoder:get_constant_offsets()
		end)
	end)

	it("options", function()
		local encoder = Encoder:new(64)

		assert.is_false(encoder:prevent_vex2())
		assert.equals(0, encoder:vex_wig())
		assert.equals(0, encoder:vex_lig())
		assert.equals(0, encoder:evex_wig())
		assert.equals(0, encoder:evex_lig())
		assert.equals(0, encoder:mvex_wig())

		encoder:set_prevent_vex2(true)
		assert.is_true(encoder:prevent_vex2())
		encoder:set_vex_wig(1)
		assert.equals(1, encoder:vex_wig())
		encoder:set_vex_lig(1)
		assert.equals(1, encoder:vex_lig())
		encoder:set_evex_wig(1)
		assert.equals(1, encoder:evex_wig())
		encoder:set_evex_lig(1)
		assert.equals(1, encoder:evex_lig())
		encoder:set_mvex_wig(1)
		assert.equals(1, encoder:mvex_wig())
	end)

	it("encode", function()
		local decoder = Decoder:new(64, from_hex("F390" .. "90"))
		local encoder = Encoder:new(64)
		for instr in decoder:iter_out() do
			encoder:encode(instr, instr:ip())
		end
		local buffer = encoder:take_buffer()
		assert.equals("\243\144\144", buffer)
	end)
end)
