// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use wasm_bindgen::prelude::*;

// GENERATOR-BEGIN: Enum
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Instruction encoding
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum EncodingKind {
	/// Legacy encoding
	Legacy = 0,
	/// VEX encoding
	VEX = 1,
	/// EVEX encoding
	EVEX = 2,
	/// XOP encoding
	XOP = 3,
	/// 3DNow! encoding
	D3NOW = 4,
}
// GENERATOR-END: Enum

#[allow(dead_code)]
pub(crate) fn iced_to_encoding_kind(value: iced_x86_rust::EncodingKind) -> EncodingKind {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}
