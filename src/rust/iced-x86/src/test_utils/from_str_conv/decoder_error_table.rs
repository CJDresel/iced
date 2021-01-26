// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

#![allow(unused_results)]

use super::super::super::DecoderError;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

lazy_static! {
	pub(super) static ref TO_DECODER_ERROR_HASH: HashMap<&'static str, DecoderError> = {
		// GENERATOR-BEGIN: DecoderErrorHash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(3);
		h.insert("None", DecoderError::None);
		h.insert("InvalidInstruction", DecoderError::InvalidInstruction);
		h.insert("NoMoreBytes", DecoderError::NoMoreBytes);
		// GENERATOR-END: DecoderErrorHash
		h
	};
}
