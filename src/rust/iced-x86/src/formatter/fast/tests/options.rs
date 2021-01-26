// SPDX-License-Identifier: MIT
// Copyright wtfsckgh@gmail.com
// Copyright iced contributors

use super::super::super::tests::options::{test_format_file_common_fast, test_format_file_fast};
use super::fmt_factory;

#[test]
fn test_options_common() {
	test_format_file_common_fast("Fast", "OptionsResult.Common", fmt_factory::create_options);
}

#[test]
fn test_options2() {
	test_format_file_fast("Fast", "OptionsResult2", "Options2", fmt_factory::create_options);
}
