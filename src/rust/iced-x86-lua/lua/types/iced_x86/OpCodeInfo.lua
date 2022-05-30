-- SPDX-License-Identifier: MIT
-- Copyright (C) 2018-present iced project and contributors

-- ⚠️This file was generated by GENERATOR!🦹‍♂️

---@meta
---@diagnostic disable unused-local

---Opcode info, returned by `Instruction:op_code()` or created by the constructor
---
---@class OpCodeInfo
local OpCodeInfo = {}

---Creates a new instance
---
---@param code Code #Code value
---@return OpCodeInfo
---
---# Examples
---
---```lua
---local Code = require("iced_x86.Code")
---local EncodingKind = require("iced_x86.EncodingKind")
---local OpCodeInfo = require("iced_x86.OpCodeInfo")
---
---local op_code = OpCodeInfo.new(Code.EVEX_Vmovapd_ymm_k1z_ymmm256)
---assert(op_code:op_code_string() == "EVEX.256.66.0F.W1 28 /r")
---assert(op_code:encoding() == EncodingKind.EVEX)
---assert(OpCodeInfo.new(Code.Sub_r8_rm8):op_code() == 0x2A)
---assert(OpCodeInfo.new(Code.Cvtpi2ps_xmm_mmm64):op_code() == 0x2A)
---```
function OpCodeInfo.new(code) end

---Gets the code (a `Code` enum value)
---
---@return integer #A `Code` enum value
---
---# Examples
---
---```lua
---local Code = require("iced_x86.Code")
---local OpCodeInfo = require("iced_x86.OpCodeInfo")
---
---local op_code = OpCodeInfo.new(Code.EVEX_Vmovapd_ymm_k1z_ymmm256)
---assert(op_code:code() == Code.EVEX_Vmovapd_ymm_k1z_ymmm256)
---```
function OpCodeInfo:code() end

---Gets the mnemonic (a `Mnemonic` enum value)
---
---@return integer #A `Mnemonic` enum value
---
---# Examples
---
---```lua
---local Code = require("iced_x86.Code")
---local OpCodeInfo = require("iced_x86.OpCodeInfo")
---local Mnemonic = require("iced_x86.Mnemonic")
---
---local op_code = OpCodeInfo.new(Code.EVEX_Vmovapd_ymm_k1z_ymmm256)
---assert(op_code:mnemonic() == Mnemonic.Vmovapd)
---```
function OpCodeInfo:mnemonic() end

---Gets the encoding (an `EncodingKind` enum value)
---
---@return integer #An `EncodingKind` enum value
---
---# Examples
---
---```lua
---local Code = require("iced_x86.Code")
---local EncodingKind = require("iced_x86.EncodingKind")
---local OpCodeInfo = require("iced_x86.OpCodeInfo")
---
---local op_code = OpCodeInfo.new(Code.EVEX_Vmovapd_ymm_k1z_ymmm256)
---assert(op_code:encoding() == EncodingKind.EVEX)
---```
function OpCodeInfo:encoding() end

---`true` if it's an instruction, `false` if it's eg. `Code.INVALID`, `db`, `dw`, `dd`, `dq`, `zero_bytes`
---
---@return boolean
---
---# Examples
---
---```lua
---local Code = require("iced_x86.Code")
---local OpCodeInfo = require("iced_x86.OpCodeInfo")
---
---assert(OpCodeInfo.new(Code.EVEX_Vmovapd_ymm_k1z_ymmm256):is_instruction())
---assert(not OpCodeInfo.new(Code.INVALID):is_instruction())
---assert(not OpCodeInfo.new(Code.DeclareByte):is_instruction())
---```
function OpCodeInfo:is_instruction() end

---`true` if it's an instruction available in 16-bit mode
---
---@return boolean
function OpCodeInfo:mode16() end

---`true` if it's an instruction available in 32-bit mode
---
---@return boolean
function OpCodeInfo:mode32() end

---`true` if it's an instruction available in 64-bit mode
---
---@return boolean
function OpCodeInfo:mode64() end

---`true` if an `FWAIT` (`9B`) instruction is added before the instruction
---
---@return boolean
function OpCodeInfo:fwait() end

---(Legacy encoding) Gets the required operand size (16,32,64) or 0
---
---@return integer
function OpCodeInfo:operand_size() end

---(Legacy encoding) Gets the required address size (16,32,64) or 0
---
---@return integer
function OpCodeInfo:address_size() end

---(VEX/XOP/EVEX) `L` / `L'L` value or default value if `OpCodeInfo:is_lig()` is `true`
---
---@return integer
function OpCodeInfo:l() end

---(VEX/XOP/EVEX/MVEX) `W` value or default value if `OpCodeInfo:is_wig()` or `OpCodeInfo:is_wig32()` is `true`
---
---@return integer
function OpCodeInfo:w() end

---(VEX/XOP/EVEX) `true` if the `L` / `L'L` fields are ignored.
---
---EVEX: if reg-only ops and `{er}` (`EVEX.b` is set), `L'L` is the rounding control and not ignored.
---
---@return boolean
function OpCodeInfo:is_lig() end

---(VEX/XOP/EVEX/MVEX) `true` if the `W` field is ignored in 16/32/64-bit modes
---
---@return boolean
function OpCodeInfo:is_wig() end

---(VEX/XOP/EVEX/MVEX) `true` if the `W` field is ignored in 16/32-bit modes (but not 64-bit mode)
---
---@return boolean
function OpCodeInfo:is_wig32() end

---(EVEX/MVEX) Gets the tuple type (a `TupleType` enum value)
---
---@return integer #A `TupleType` enum value
function OpCodeInfo:tuple_type() end

---(MVEX) Gets the `EH` bit that's required to encode this instruction (an `MvexEHBit` enum value)
---
---@return integer #An `MvexEHBit` enum value
function OpCodeInfo:mvex_eh_bit() end

---(MVEX) `true` if the instruction supports eviction hint (if it has a memory operand)
---
---@return boolean
function OpCodeInfo:mvex_can_use_eviction_hint() end

---(MVEX) `true` if the instruction's rounding control bits are stored in `imm8[1:0]`
---
---@return boolean
function OpCodeInfo:mvex_can_use_imm_rounding_control() end

---(MVEX) `true` if the instruction ignores op mask registers (eg. `{k1}`)
---
---@return boolean
function OpCodeInfo:mvex_ignores_op_mask_register() end

---(MVEX) `true` if the instruction must have `MVEX.SSS=000` if `MVEX.EH=1`
---
---@return boolean
function OpCodeInfo:mvex_no_sae_rc() end

---(MVEX) Gets the tuple type / conv lut kind (an `MvexTupleTypeLutKind` enum value)
---
---@return integer #An `MvexTupleTypeLutKind` enum value
function OpCodeInfo:mvex_tuple_type_lut_kind() end

---(MVEX) Gets the conversion function, eg. `Sf32` (an `MvexConvFn` enum value)
---
---@return integer #An `MvexConvFn` enum value
function OpCodeInfo:mvex_conversion_func() end

---(MVEX) Gets flags indicating which conversion functions are valid (bit 0 == func 0)
---
---@return integer
function OpCodeInfo:mvex_valid_conversion_funcs_mask() end

---(MVEX) Gets flags indicating which swizzle functions are valid (bit 0 == func 0)
---
---@return integer
function OpCodeInfo:mvex_valid_swizzle_funcs_mask() end

---If it has a memory operand, gets the `MemorySize` (non-broadcast memory type)
---
---@return integer #A `MemorySize` enum value
function OpCodeInfo:memory_size() end

---If it has a memory operand, gets the `MemorySize` (broadcast memory type)
---
---@return integer #A `MemorySize` enum value
function OpCodeInfo:broadcast_memory_size() end

---(EVEX) `true` if the instruction supports broadcasting (`EVEX.b` bit) (if it has a memory operand)
---
---@return boolean
function OpCodeInfo:can_broadcast() end

---(EVEX/MVEX) `true` if the instruction supports rounding control
---
---@return boolean
function OpCodeInfo:can_use_rounding_control() end

---(EVEX/MVEX) `true` if the instruction supports suppress all exceptions
---
---@return boolean
function OpCodeInfo:can_suppress_all_exceptions() end

---(EVEX/MVEX) `true` if an opmask register can be used
---
---@return boolean
function OpCodeInfo:can_use_op_mask_register() end

---(EVEX/MVEX) `true` if a non-zero opmask register must be used
---
---@return boolean
function OpCodeInfo:require_op_mask_register() end

---(EVEX) `true` if the instruction supports zeroing masking (if one of the opmask registers `K1`-`K7` is used and destination operand is not a memory operand)
---
---@return boolean
function OpCodeInfo:can_use_zeroing_masking() end

---`true` if the `LOCK` (`F0`) prefix can be used
---
---@return boolean
function OpCodeInfo:can_use_lock_prefix() end

---`true` if the `XACQUIRE` (`F2`) prefix can be used
---
---@return boolean
function OpCodeInfo:can_use_xacquire_prefix() end

---`true` if the `XRELEASE` (`F3`) prefix can be used
---
---@return boolean
function OpCodeInfo:can_use_xrelease_prefix() end

---`true` if the `REP` / `REPE` (`F3`) prefixes can be used
---
---@return boolean
function OpCodeInfo:can_use_rep_prefix() end

---`true` if the `REPNE` (`F2`) prefix can be used
---
---@return boolean
function OpCodeInfo:can_use_repne_prefix() end

---`true` if the `BND` (`F2`) prefix can be used
---
---@return boolean
function OpCodeInfo:can_use_bnd_prefix() end

---`true` if the `HINT-TAKEN` (`3E`) and `HINT-NOT-TAKEN` (`2E`) prefixes can be used
---
---@return boolean
function OpCodeInfo:can_use_hint_taken_prefix() end

---`true` if the `NOTRACK` (`3E`) prefix can be used
---
---@return boolean
function OpCodeInfo:can_use_notrack_prefix() end

---`true` if rounding control is ignored (#UD is not generated)
---
---@return boolean
function OpCodeInfo:ignores_rounding_control() end

---`true` if the `LOCK` prefix can be used as an extra register bit (bit 3) to access registers 8-15 without a `REX` prefix (eg. in 32-bit mode)
---
---@return boolean
function OpCodeInfo:amd_lock_reg_bit() end

---`true` if the default operand size is 64 in 64-bit mode. A `66` prefix can switch to 16-bit operand size.
---
---@return boolean
function OpCodeInfo:default_op_size64() end

---`true` if the operand size is always 64 in 64-bit mode. A `66` prefix is ignored.
---
---@return boolean
function OpCodeInfo:force_op_size64() end

---`true` if the Intel decoder forces 64-bit operand size. A `66` prefix is ignored.
---
---@return boolean
function OpCodeInfo:intel_force_op_size64() end

---`true` if it can only be executed when CPL=0
---
---@return boolean
function OpCodeInfo:must_be_cpl0() end

---`true` if it can be executed when CPL=0
---
---@return boolean
function OpCodeInfo:cpl0() end

---`true` if it can be executed when CPL=1
---
---@return boolean
function OpCodeInfo:cpl1() end

---`true` if it can be executed when CPL=2
---
---@return boolean
function OpCodeInfo:cpl2() end

---`true` if it can be executed when CPL=3
---
---@return boolean
function OpCodeInfo:cpl3() end

---`true` if the instruction accesses the I/O address space (eg. `IN`, `OUT`, `INS`, `OUTS`)
---
---@return boolean
function OpCodeInfo:is_input_output() end

---`true` if it's one of the many nop instructions (does not include FPU nop instructions, eg. `FNOP`)
---
---@return boolean
function OpCodeInfo:is_nop() end

---`true` if it's one of the many reserved nop instructions (eg. `0F0D`, `0F18-0F1F`)
---
---@return boolean
function OpCodeInfo:is_reserved_nop() end

---`true` if it's a serializing instruction (Intel CPUs)
---
---@return boolean
function OpCodeInfo:is_serializing_intel() end

---`true` if it's a serializing instruction (AMD CPUs)
---
---@return boolean
function OpCodeInfo:is_serializing_amd() end

---`true` if the instruction requires either CPL=0 or CPL<=3 depending on some CPU option (eg. `CR4.TSD`, `CR4.PCE`, `CR4.UMIP`)
---
---@return boolean
function OpCodeInfo:may_require_cpl0() end

---`true` if it's a tracked `JMP`/`CALL` indirect instruction (CET)
---
---@return boolean
function OpCodeInfo:is_cet_tracked() end

---`true` if it's a non-temporal hint memory access (eg. `MOVNTDQ`)
---
---@return boolean
function OpCodeInfo:is_non_temporal() end

---`true` if it's a no-wait FPU instruction, eg. `FNINIT`
---
---@return boolean
function OpCodeInfo:is_fpu_no_wait() end

---`true` if the mod bits are ignored and it's assumed `modrm[7:6] == 11b`
---
---@return boolean
function OpCodeInfo:ignores_mod_bits() end

---`true` if the `66` prefix is not allowed (it will #UD)
---
---@return boolean
function OpCodeInfo:no66() end

---`true` if the `F2`/`F3` prefixes aren't allowed
---
---@return boolean
function OpCodeInfo:nfx() end

---`true` if the index reg's reg-num (vsib op) (if any) and register ops' reg-nums must be unique,
---
---@return boolean
---
---eg. `MNEMONIC XMM1,YMM1,[RAX+ZMM1*2]` is invalid. Registers = `XMM`/`YMM`/`ZMM`/`TMM`.
function OpCodeInfo:requires_unique_reg_nums() end

---`true` if the destination register's reg-num must not be present in any other operand, eg.
---
---@return boolean
---
---`MNEMONIC XMM1,YMM1,[RAX+ZMM1*2]` is invalid. Registers = `XMM`/`YMM`/`ZMM`/`TMM`.
function OpCodeInfo:requires_unique_dest_reg_num() end

---`true` if it's a privileged instruction (all CPL=0 instructions (except `VMCALL`) and IOPL instructions `IN`, `INS`, `OUT`, `OUTS`, `CLI`, `STI`)
---
---@return boolean
function OpCodeInfo:is_privileged() end

---`true` if it reads/writes too many registers
---
---@return boolean
function OpCodeInfo:is_save_restore() end

---`true` if it's an instruction that implicitly uses the stack register, eg. `CALL`, `POP`, etc
---
---@return boolean
function OpCodeInfo:is_stack_instruction() end

---`true` if the instruction doesn't read the segment register if it uses a memory operand
---
---@return boolean
function OpCodeInfo:ignores_segment() end

---`true` if the opmask register is read and written (instead of just read). This also implies that it can't be `K0`.
---
---@return boolean
function OpCodeInfo:is_op_mask_read_write() end

---`true` if it can be executed in real mode
---
---@return boolean
function OpCodeInfo:real_mode() end

---`true` if it can be executed in protected mode
---
---@return boolean
function OpCodeInfo:protected_mode() end

---`true` if it can be executed in virtual 8086 mode
---
---@return boolean
function OpCodeInfo:virtual8086_mode() end

---`true` if it can be executed in compatibility mode
---
---@return boolean
function OpCodeInfo:compatibility_mode() end

---`true` if it can be executed in 64-bit mode
---
---@return boolean
function OpCodeInfo:long_mode() end

---`true` if it can be used outside SMM
---
---@return boolean
function OpCodeInfo:use_outside_smm() end

---`true` if it can be used in SMM
---
---@return boolean
function OpCodeInfo:use_in_smm() end

---`true` if it can be used outside an enclave (SGX)
---
---@return boolean
function OpCodeInfo:use_outside_enclave_sgx() end

---`true` if it can be used inside an enclave (SGX1)
---
---@return boolean
function OpCodeInfo:use_in_enclave_sgx1() end

---`true` if it can be used inside an enclave (SGX2)
---
---@return boolean
function OpCodeInfo:use_in_enclave_sgx2() end

---`true` if it can be used outside VMX operation
---
---@return boolean
function OpCodeInfo:use_outside_vmx_op() end

---`true` if it can be used in VMX root operation
---
---@return boolean
function OpCodeInfo:use_in_vmx_root_op() end

---`true` if it can be used in VMX non-root operation
---
---@return boolean
function OpCodeInfo:use_in_vmx_non_root_op() end

---`true` if it can be used outside SEAM
---
---@return boolean
function OpCodeInfo:use_outside_seam() end

---`true` if it can be used in SEAM
---
---@return boolean
function OpCodeInfo:use_in_seam() end

---`true` if #UD is generated in TDX non-root operation
---
---@return boolean
function OpCodeInfo:tdx_non_root_gen_ud() end

---`true` if #VE is generated in TDX non-root operation
---
---@return boolean
function OpCodeInfo:tdx_non_root_gen_ve() end

---`true` if an exception (eg. #GP(0), #VE) may be generated in TDX non-root operation
---
---@return boolean
function OpCodeInfo:tdx_non_root_may_gen_ex() end

---(Intel VMX) `true` if it causes a VM exit in VMX non-root operation
---
---@return boolean
function OpCodeInfo:intel_vm_exit() end

---(Intel VMX) `true` if it may cause a VM exit in VMX non-root operation
---
---@return boolean
function OpCodeInfo:intel_may_vm_exit() end

---(Intel VMX) `true` if it causes an SMM VM exit in VMX root operation (if dual-monitor treatment is activated)
---
---@return boolean
function OpCodeInfo:intel_smm_vm_exit() end

---(AMD SVM) `true` if it causes a #VMEXIT in guest mode
---
---@return boolean
function OpCodeInfo:amd_vm_exit() end

---(AMD SVM) `true` if it may cause a #VMEXIT in guest mode
---
---@return boolean
function OpCodeInfo:amd_may_vm_exit() end

---`true` if it causes a TSX abort inside a TSX transaction
---
---@return boolean
function OpCodeInfo:tsx_abort() end

---`true` if it causes a TSX abort inside a TSX transaction depending on the implementation
---
---@return boolean
function OpCodeInfo:tsx_impl_abort() end

---`true` if it may cause a TSX abort inside a TSX transaction depending on some condition
---
---@return boolean
function OpCodeInfo:tsx_may_abort() end

---`true` if it's decoded by iced's 16-bit Intel decoder
---
---@return boolean
function OpCodeInfo:intel_decoder16() end

---`true` if it's decoded by iced's 32-bit Intel decoder
---
---@return boolean
function OpCodeInfo:intel_decoder32() end

---`true` if it's decoded by iced's 64-bit Intel decoder
---
---@return boolean
function OpCodeInfo:intel_decoder64() end

---`true` if it's decoded by iced's 16-bit AMD decoder
---
---@return boolean
function OpCodeInfo:amd_decoder16() end

---`true` if it's decoded by iced's 32-bit AMD decoder
---
---@return boolean
function OpCodeInfo:amd_decoder32() end

---`true` if it's decoded by iced's 64-bit AMD decoder
---
---@return boolean
function OpCodeInfo:amd_decoder64() end

---Gets the decoder option that's needed to decode the instruction or `DecoderOptions.None`.
---
---@return integer #A `DecoderOptions` enum value
function OpCodeInfo:decoder_option() end

---Gets the opcode table (an `OpCodeTableKind` enum value)
---
---@return integer #An `OpCodeTableKind` enum value
function OpCodeInfo:table() end

---Gets the mandatory prefix (a `MandatoryPrefix` enum value)
---
---@return integer #A `MandatoryPrefix` enum value
function OpCodeInfo:mandatory_prefix() end

---Gets the opcode byte(s). The low byte(s) of this value is the opcode. The length is in `OpCodeInfo:op_code_len()`.
---It doesn't include the table value, see `OpCodeInfo:table()`.
---
---@return integer
---
---# Examples
---
---```lua
---local Code = require("iced_x86.Code")
---local OpCodeInfo = require("iced_x86.OpCodeInfo")
---
---assert(OpCodeInfo.new(Code.Ffreep_sti):op_code() == 0xDFC0)
---assert(OpCodeInfo.new(Code.Vmrunw):op_code() == 0x01D8)
---assert(OpCodeInfo.new(Code.Sub_r8_rm8):op_code() == 0x2A)
---assert(OpCodeInfo.new(Code.Cvtpi2ps_xmm_mmm64):op_code() == 0x2A)
---```
function OpCodeInfo:op_code() end

---Gets the length of the opcode bytes (`OpCodeInfo:op_code()`). The low bytes is the opcode value.
---
---@return integer
---
---# Examples
---
---```lua
---local Code = require("iced_x86.Code")
---local OpCodeInfo = require("iced_x86.OpCodeInfo")
---
---assert(OpCodeInfo.new(Code.Ffreep_sti):op_code_len() == 2)
---assert(OpCodeInfo.new(Code.Vmrunw):op_code_len() == 2)
---assert(OpCodeInfo.new(Code.Sub_r8_rm8):op_code_len() == 1)
---assert(OpCodeInfo.new(Code.Cvtpi2ps_xmm_mmm64):op_code_len() == 1)
---```
function OpCodeInfo:op_code_len() end

---`true` if it's part of a group
---
---@return boolean
function OpCodeInfo:is_group() end

---Group index (0-7) or -1. If it's 0-7, it's stored in the `reg` field of the `modrm` byte.
---
---@return integer
function OpCodeInfo:group_index() end

---`true` if it's part of a modrm.rm group
---
---@return boolean
function OpCodeInfo:is_rm_group() end

---Group index (0-7) or -1. If it's 0-7, it's stored in the `rm` field of the `modrm` byte.
---
---@return integer
function OpCodeInfo:rm_group_index() end

---Gets the number of operands
---
---@return integer
function OpCodeInfo:op_count() end

---Gets operand #0's opkind (an `OpCodeOperandKind` enum value)
---
---@return integer #An `OpCodeOperandKind` enum value
function OpCodeInfo:op0_kind() end

---Gets operand #1's opkind (an `OpCodeOperandKind` enum value)
---
---@return integer #An `OpCodeOperandKind` enum value
function OpCodeInfo:op1_kind() end

---Gets operand #2's opkind (an `OpCodeOperandKind` enum value)
---
---@return integer #An `OpCodeOperandKind` enum value
function OpCodeInfo:op2_kind() end

---Gets operand #3's opkind (an `OpCodeOperandKind` enum value)
---
---@return integer #An `OpCodeOperandKind` enum value
function OpCodeInfo:op3_kind() end

---Gets operand #4's opkind (an `OpCodeOperandKind` enum value)
---
---@return integer #An `OpCodeOperandKind` enum value
function OpCodeInfo:op4_kind() end

---Gets an operand's opkind (an `OpCodeOperandKind` enum value)
---
---@param operand integer #Operand number, 0-4
---@return integer #(An `OpCodeOperandKind` enum value) Operand kind
function OpCodeInfo:op_kind(operand) end

---Gets all operand kinds (a list of `OpCodeOperandKind` enum values)
---
---@return integer[] #(`OpCodeOperandKind[]`) All operand kinds
function OpCodeInfo:op_kinds() end

---Checks if the instruction is available in 16-bit mode, 32-bit mode or 64-bit mode
---
---@param bitness integer #16, 32 or 64
---@return boolean #`true` if it's available in the mode
function OpCodeInfo:is_available_in_mode(bitness) end

---Gets the opcode string, eg. `VEX.128.66.0F38.W0 78 /r`, see also `OpCodeInfo:instruction_string()`
---
---@return string
---
---# Examples
---
---```lua
---local Code = require("iced_x86.Code")
---local OpCodeInfo = require("iced_x86.OpCodeInfo")
---
---local op_code = OpCodeInfo.new(Code.EVEX_Vmovapd_ymm_k1z_ymmm256)
---assert(op_code:op_code_string() == "EVEX.256.66.0F.W1 28 /r")
---```
function OpCodeInfo:op_code_string() end

---Gets the instruction string, eg. `VPBROADCASTB xmm1, xmm2/m8`, see also `OpCodeInfo:op_code_string()`
---
---@return string
---
---# Examples
---
---```lua
---local Code = require("iced_x86.Code")
---local OpCodeInfo = require("iced_x86.OpCodeInfo")
---
---local op_code = OpCodeInfo.new(Code.EVEX_Vmovapd_ymm_k1z_ymmm256)
---assert(op_code:instruction_string() == "VMOVAPD ymm1 {k1}{z}, ymm2/m256")
---```
function OpCodeInfo:instruction_string() end

return OpCodeInfo
