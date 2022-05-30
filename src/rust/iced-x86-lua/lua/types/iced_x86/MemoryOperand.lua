-- SPDX-License-Identifier: MIT
-- Copyright (C) 2018-present iced project and contributors

-- ⚠️This file was generated by GENERATOR!🦹‍♂️

---@meta
---@diagnostic disable unused-local

---Memory operand passed to one of `Instruction`'s `create*()` constructor methods
---
---@class MemoryOperand
local MemoryOperand = {}

---Memory operand passed to one of `Instruction`'s `create*()` constructor methods
---
---@param base? integer #(A `Register` enum variant) (default = `None`) Base register or `Register.None`
---@param index? integer #(A `Register` enum variant) (default = `None`) Index register or `Register.None`
---@param scale? integer #(default = `1`) Index register scale (1, 2, 4, or 8)
---@param displ? integer #(default = `0`) Memory displacement
---@param displ_size? integer #(default = `0`) 0 (no displ), 1 (16/32/64-bit, but use 2/4/8 if it doesn't fit in a `i8`), 2 (16-bit), 4 (32-bit) or 8 (64-bit)
---@param is_broadcast? boolean #(default = `false`) `true` if it's broadcast memory (EVEX instructions)
---@param segment? integer #(A `Register` enum variant) (default = `None`) Segment override or `Register.None`
---@return MemoryOperand
function MemoryOperand.new(base, index, scale, displ, displ_size, is_broadcast, segment) end

---Memory operand passed to one of `Instruction`'s `create*()` constructor methods
---
---@param base integer #(A `Register` enum variant) Base register or `Register.None`
---@param index integer #(A `Register` enum variant) Index register or `Register.None`
---@param scale integer #Index register scale (1, 2, 4, or 8)
---@param displ integer #Memory displacement
---@param displ_size integer #0 (no displ), 1 (16/32/64-bit, but use 2/4/8 if it doesn't fit in a `i8`), 2 (16-bit), 4 (32-bit) or 8 (64-bit)
---@return MemoryOperand
function MemoryOperand.with_base_index_scale_displ_size(base, index, scale, displ, displ_size) end

---Memory operand passed to one of `Instruction`'s `create*()` constructor methods
---
---@param base integer #(A `Register` enum variant) Base register or `Register.None`
---@param index integer #(A `Register` enum variant) Index register or `Register.None`
---@param scale integer #Index register scale (1, 2, 4, or 8)
---@return MemoryOperand
function MemoryOperand.with_base_index_scale(base, index, scale) end

---Memory operand passed to one of `Instruction`'s `create*()` constructor methods
---
---@param base integer #(A `Register` enum variant) Base register or `Register.None`
---@param index integer #(A `Register` enum variant) Index register or `Register.None`
---@return MemoryOperand
function MemoryOperand.with_base_index(base, index) end

---Memory operand passed to one of `Instruction`'s `create*()` constructor methods
---
---@param base integer #(A `Register` enum variant) Base register or `Register.None`
---@param displ integer #Memory displacement
---@param displ_size integer #0 (no displ), 1 (16/32/64-bit, but use 2/4/8 if it doesn't fit in a `i8`), 2 (16-bit), 4 (32-bit) or 8 (64-bit)
---@return MemoryOperand
function MemoryOperand.with_base_displ_size(base, displ, displ_size) end

---Memory operand passed to one of `Instruction`'s `create*()` constructor methods
---
---@param index integer #(A `Register` enum variant) Index register or `Register.None`
---@param scale integer #Index register scale (1, 2, 4, or 8)
---@param displ integer #Memory displacement
---@param displ_size integer #0 (no displ), 1 (16/32/64-bit, but use 2/4/8 if it doesn't fit in a `i8`), 2 (16-bit), 4 (32-bit) or 8 (64-bit)
---@return MemoryOperand
function MemoryOperand.with_index_scale_displ_size(index, scale, displ, displ_size) end

---Memory operand passed to one of `Instruction`'s `create*()` constructor methods
---
---@param base integer #(A `Register` enum variant) Base register or `Register.None`
---@param displ integer #Memory displacement
---@return MemoryOperand
function MemoryOperand.with_base_displ(base, displ) end

---Memory operand passed to one of `Instruction`'s `create*()` constructor methods
---
---@param base integer #(A `Register` enum variant) Base register or `Register.None`
---@return MemoryOperand
function MemoryOperand.with_base(base) end

---Memory operand passed to one of `Instruction`'s `create*()` constructor methods
---
---@param displ integer #Memory displacement
---@param displ_size? integer #(default = `1`) 0 (no displ), 1 (16/32/64-bit, but use 2/4/8 if it doesn't fit in a `i8`), 2 (16-bit), 4 (32-bit) or 8 (64-bit)
---@return MemoryOperand
function MemoryOperand.with_displ(displ, displ_size) end

---Returns a copy of this instance.
---
---@return MemoryOperand
function MemoryOperand:copy() end

return MemoryOperand
