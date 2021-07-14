#![no_main]
use libfuzzer_sys::fuzz_target;

#[macro_use]
extern crate lazy_static;

use ethabi_tests::fuzztests::{load_abi, run_fuzzcase_on_contract_functions};

lazy_static! {
	static ref FUNCTIONS: Vec<ethabi::Function> = load_abi().1;
}

fuzz_target!(|data: &[u8]| {
	if data.len() > 2 {
		run_fuzzcase_on_contract_functions(&FUNCTIONS[0..], data, false);
	}
});
