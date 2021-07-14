//! Testcases produced by running a fuzzer

/// function used by the fuzzing driver to run input decode for a given contract
pub fn run_fuzzcase_on_contract_functions(funcs: &[ethabi::Function], data: &[u8], output: bool) {
	let n = (data[0] as usize) % funcs.len();
	let func = funcs.into_iter().nth(n).unwrap();
	if output {
		println!("function: {}", func.signature());
		println!("input: {:?}", data);
	}
	match func.decode_input(&data[1..]) {
		Ok(dec) => {
			if output {
				println!("decode: {:?}", dec);
			}
		}
		Err(e) => {
			if output {
				println!("error: {:?}", e);
			}
		}
	}
}

/// load the big.abi ABI spec from the `res` directory and construct a contract and a list of
/// functions, which is sorted by signature.
pub fn load_abi() -> (ethabi::Contract, Vec<ethabi::Function>) {
	let contract: ethabi::Contract = {
		let b = include_bytes!("../../res/big.abi");
		ethabi::Contract::load(&b[0..]).unwrap()
	};
	let mut funcs: Vec<ethabi::Function> = contract.functions().cloned().collect();
	funcs.sort_by(|a, b| a.signature().partial_cmp(&b.signature()).unwrap());
	(contract, funcs)
}

#[test]
fn fuzz_test_1() {
	let (_contract, funcs) = load_abi();
	let input =
		include_bytes!("../fuzz/artifacts/fixed_abi_decode_random/crash-651f6fb1e4f699cffb23f8cb616f11590d81f5dd");

	run_fuzzcase_on_contract_functions(&funcs, &input[0..], true);
}
