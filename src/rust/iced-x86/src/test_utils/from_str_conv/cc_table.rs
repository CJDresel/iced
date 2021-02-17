// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use super::super::super::{CC_a, CC_ae, CC_b, CC_be, CC_e, CC_g, CC_ge, CC_l, CC_le, CC_ne, CC_np, CC_p};
use std::collections::HashMap;

lazy_static! {
	pub(super) static ref TO_CC_B_HASH: HashMap<&'static str, CC_b> = {
		// GENERATOR-BEGIN: CC_b_hash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(3);
		let _ = h.insert("b", CC_b::b);
		let _ = h.insert("c", CC_b::c);
		let _ = h.insert("nae", CC_b::nae);
		// GENERATOR-END: CC_b_hash
		h
	};
}

lazy_static! {
	pub(super) static ref TO_CC_AE_HASH: HashMap<&'static str, CC_ae> = {
		// GENERATOR-BEGIN: CC_ae_hash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(3);
		let _ = h.insert("ae", CC_ae::ae);
		let _ = h.insert("nb", CC_ae::nb);
		let _ = h.insert("nc", CC_ae::nc);
		// GENERATOR-END: CC_ae_hash
		h
	};
}

lazy_static! {
	pub(super) static ref TO_CC_E_HASH: HashMap<&'static str, CC_e> = {
		// GENERATOR-BEGIN: CC_e_hash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(2);
		let _ = h.insert("e", CC_e::e);
		let _ = h.insert("z", CC_e::z);
		// GENERATOR-END: CC_e_hash
		h
	};
}

lazy_static! {
	pub(super) static ref TO_CC_NE_HASH: HashMap<&'static str, CC_ne> = {
		// GENERATOR-BEGIN: CC_ne_hash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(2);
		let _ = h.insert("ne", CC_ne::ne);
		let _ = h.insert("nz", CC_ne::nz);
		// GENERATOR-END: CC_ne_hash
		h
	};
}

lazy_static! {
	pub(super) static ref TO_CC_BE_HASH: HashMap<&'static str, CC_be> = {
		// GENERATOR-BEGIN: CC_be_hash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(2);
		let _ = h.insert("be", CC_be::be);
		let _ = h.insert("na", CC_be::na);
		// GENERATOR-END: CC_be_hash
		h
	};
}

lazy_static! {
	pub(super) static ref TO_CC_A_HASH: HashMap<&'static str, CC_a> = {
		// GENERATOR-BEGIN: CC_a_hash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(2);
		let _ = h.insert("a", CC_a::a);
		let _ = h.insert("nbe", CC_a::nbe);
		// GENERATOR-END: CC_a_hash
		h
	};
}

lazy_static! {
	pub(super) static ref TO_CC_P_HASH: HashMap<&'static str, CC_p> = {
		// GENERATOR-BEGIN: CC_p_hash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(2);
		let _ = h.insert("p", CC_p::p);
		let _ = h.insert("pe", CC_p::pe);
		// GENERATOR-END: CC_p_hash
		h
	};
}

lazy_static! {
	pub(super) static ref TO_CC_NP_HASH: HashMap<&'static str, CC_np> = {
		// GENERATOR-BEGIN: CC_np_hash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(2);
		let _ = h.insert("np", CC_np::np);
		let _ = h.insert("po", CC_np::po);
		// GENERATOR-END: CC_np_hash
		h
	};
}

lazy_static! {
	pub(super) static ref TO_CC_L_HASH: HashMap<&'static str, CC_l> = {
		// GENERATOR-BEGIN: CC_l_hash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(2);
		let _ = h.insert("l", CC_l::l);
		let _ = h.insert("nge", CC_l::nge);
		// GENERATOR-END: CC_l_hash
		h
	};
}

lazy_static! {
	pub(super) static ref TO_CC_GE_HASH: HashMap<&'static str, CC_ge> = {
		// GENERATOR-BEGIN: CC_ge_hash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(2);
		let _ = h.insert("ge", CC_ge::ge);
		let _ = h.insert("nl", CC_ge::nl);
		// GENERATOR-END: CC_ge_hash
		h
	};
}

lazy_static! {
	pub(super) static ref TO_CC_LE_HASH: HashMap<&'static str, CC_le> = {
		// GENERATOR-BEGIN: CC_le_hash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(2);
		let _ = h.insert("le", CC_le::le);
		let _ = h.insert("ng", CC_le::ng);
		// GENERATOR-END: CC_le_hash
		h
	};
}

lazy_static! {
	pub(super) static ref TO_CC_G_HASH: HashMap<&'static str, CC_g> = {
		// GENERATOR-BEGIN: CC_g_hash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(2);
		let _ = h.insert("g", CC_g::g);
		let _ = h.insert("nle", CC_g::nle);
		// GENERATOR-END: CC_g_hash
		h
	};
}
