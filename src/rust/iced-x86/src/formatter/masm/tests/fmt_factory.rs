// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

use super::super::super::enums_shared::MemorySizeOptions;
use super::super::super::MasmFormatter;
use super::super::super::{Formatter, FormatterOptionsProvider, SymbolResolver};
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::string::String;

fn create_fmt() -> Box<MasmFormatter> {
	create_fmt2(None, None)
}

fn create_fmt2(symbol_resolver: Option<Box<dyn SymbolResolver>>, options_provider: Option<Box<dyn FormatterOptionsProvider>>) -> Box<MasmFormatter> {
	Box::new(MasmFormatter::with_options(symbol_resolver, options_provider))
}

pub(super) fn create_memdefault() -> Box<MasmFormatter> {
	let mut fmt = create_fmt();
	fmt.options_mut().set_memory_size_options(MemorySizeOptions::Default);
	fmt.options_mut().set_show_branch_size(false);
	fmt.options_mut().set_rip_relative_addresses(true);
	fmt.options_mut().set_signed_immediate_operands(false);
	fmt.options_mut().set_space_after_operand_separator(false);
	fmt.options_mut().set_masm_add_ds_prefix32(true);
	fmt
}

pub(super) fn create_memalways() -> Box<MasmFormatter> {
	let mut fmt = create_fmt();
	fmt.options_mut().set_memory_size_options(MemorySizeOptions::Always);
	fmt.options_mut().set_show_branch_size(true);
	fmt.options_mut().set_rip_relative_addresses(false);
	fmt.options_mut().set_signed_immediate_operands(true);
	fmt.options_mut().set_space_after_operand_separator(true);
	fmt.options_mut().set_masm_add_ds_prefix32(true);
	fmt
}

pub(super) fn create_memminimum() -> Box<MasmFormatter> {
	let mut fmt = create_fmt();
	fmt.options_mut().set_memory_size_options(MemorySizeOptions::Minimum);
	fmt.options_mut().set_show_branch_size(true);
	fmt.options_mut().set_rip_relative_addresses(false);
	fmt.options_mut().set_signed_immediate_operands(true);
	fmt.options_mut().set_space_after_operand_separator(true);
	fmt.options_mut().set_masm_add_ds_prefix32(false);
	fmt
}

pub(super) fn create() -> Box<MasmFormatter> {
	let mut fmt = create_fmt();
	fmt.options_mut().set_memory_size_options(MemorySizeOptions::Always);
	fmt.options_mut().set_show_branch_size(true);
	fmt.options_mut().set_rip_relative_addresses(false);
	fmt
}

pub(super) fn create_options() -> Box<MasmFormatter> {
	let mut fmt = create_fmt();
	fmt.options_mut().set_memory_size_options(MemorySizeOptions::Default);
	fmt.options_mut().set_show_branch_size(false);
	fmt.options_mut().set_rip_relative_addresses(true);
	fmt
}

pub(super) fn create_registers() -> Box<MasmFormatter> {
	create_fmt()
}

pub(super) fn create_numbers() -> Box<MasmFormatter> {
	let mut fmt = create_fmt();
	fmt.options_mut().set_uppercase_hex(true);
	fmt.options_mut().set_hex_prefix("");
	fmt.options_mut().set_hex_suffix("");
	fmt.options_mut().set_decimal_prefix("");
	fmt.options_mut().set_decimal_suffix("");
	fmt.options_mut().set_octal_prefix("");
	fmt.options_mut().set_octal_suffix("");
	fmt.options_mut().set_binary_prefix("");
	fmt.options_mut().set_binary_suffix("");
	fmt
}

pub(super) fn create_resolver(symbol_resolver: Box<dyn SymbolResolver>) -> Box<MasmFormatter> {
	let mut fmt = create_fmt2(Some(symbol_resolver), None);
	fmt.options_mut().set_memory_size_options(MemorySizeOptions::Default);
	fmt.options_mut().set_show_branch_size(false);
	fmt.options_mut().set_rip_relative_addresses(true);
	fmt
}
