-- SPDX-License-Identifier: MIT
-- Copyright (C) 2018-present iced project and contributors

-- ⚠️This file was generated by GENERATOR!🦹‍♂️

--- MVEX tuple type lut kind used together with the `MVEX.SSS` bits to get the tuple type
return {
	--- `i32` elements, eg. `Si32`/`Di32`/`Ui32`
	Int32 = 0,
	--- `i32` elements, eg. `Si32`/`Di32`/`Ui32` with half memory size (32 bytes instead of 64 bytes, eg. `VCVTUDQ2PD`/`VCVTDQ2PD`)
	Int32_Half = 1,
	--- `i32` elements, eg. `Si32`/`Di32`/`Ui32` with built-in `{4to16}` broadcast
	Int32_4to16 = 2,
	--- `i32` elements, eg. `Si32`/`Di32`/`Ui32` with built-in `{1to16}` broadcast or element level
	Int32_1to16_or_elem = 3,
	--- `i64` elements, eg. `Si64`/`Di64`/`Ui64`
	Int64 = 4,
	--- `i64` elements, eg. `Si64`/`Di64`/`Ui64` with built-in `{4to8}` broadcast
	Int64_4to8 = 5,
	--- `i64` elements, eg. `Si64`/`Di64`/`Ui64` with built-in `{1to8}` broadcast or element level
	Int64_1to8_or_elem = 6,
	--- `f32` elements, eg. `Sf32`/`Df32`/`Uf32`
	Float32 = 7,
	--- `f32` elements, eg. `Sf32`/`Df32`/`Uf32` with half memory size (32 bytes instead of 64 bytes, eg. `VCVTPS2PD`
	Float32_Half = 8,
	--- `f32` elements, eg. `Sf32`/`Df32`/`Uf32` with built-in `{4to16}` broadcast
	Float32_4to16 = 9,
	--- `f32` elements, eg. `Sf32`/`Df32`/`Uf32` with built-in `{1to16}` broadcast or element level
	Float32_1to16_or_elem = 10,
	--- `f64` elements, eg. `Sf64`/`Df64`/`Uf64`
	Float64 = 11,
	--- `f64` elements, eg. `Sf64`/`Df64`/`Uf64` with built-in `{4to8}` broadcast
	Float64_4to8 = 12,
	--- `f64` elements, eg. `Sf64`/`Df64`/`Uf64` with built-in `{1to8}` broadcast or element level
	Float64_1to8_or_elem = 13,
}
