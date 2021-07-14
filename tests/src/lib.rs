//! Test crate

#![deny(missing_docs)]
#![deny(dead_code)]
#![deny(unused_imports)]

use ethabi_contract::use_contract;

use_contract!(eip20, "../res/eip20.abi");
use_contract!(constructor, "../res/constructor.abi");
use_contract!(validators, "../res/Validators.abi");
use_contract!(operations, "../res/Operations.abi");
use_contract!(urlhint, "../res/urlhint.abi");
use_contract!(test_rust_keywords, "../res/test_rust_keywords.abi");

pub mod fuzztests;

#[cfg(test)]
mod tests {
	use crate::{eip20, validators};
	use ethabi::{Address, Uint};
	use hex_literal::hex;

	struct Wrapper([u8; 20]);

	impl From<Wrapper> for Address {
		fn from(wrapper: Wrapper) -> Self {
			wrapper.0.into()
		}
	}

	#[test]
	fn test_encoding_function_input_as_array() {
		use validators::functions;

		let first = [0x11u8; 20];
		let second = [0x22u8; 20];

		let encoded_from_vec = functions::set_validators::encode_input(vec![first, second]);
		let encoded_from_vec_iter = functions::set_validators::encode_input(vec![first, second].into_iter());
		let encoded_from_vec_wrapped = functions::set_validators::encode_input(vec![Wrapper(first), Wrapper(second)]);

		let expected = "9300c9260000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000200000000000000000000000011111111111111111111111111111111111111110000000000000000000000002222222222222222222222222222222222222222".to_owned();
		assert_eq!(expected, hex::encode(&encoded_from_vec));
		assert_eq!(expected, hex::encode(&encoded_from_vec_iter));
		assert_eq!(expected, hex::encode(&encoded_from_vec_wrapped));
	}

	#[test]
	fn test_decoding_function_output() {
		// Make sure that the output param type of the derived contract is correct

		// given

		let output = hex!("000000000000000000000000000000000000000000000000000000000036455B").to_vec();

		// when
		let decoded_output = eip20::functions::total_supply::decode_output(&output).unwrap();

		// then

		let expected_output: Uint = 0x36455b.into();
		assert_eq!(expected_output, decoded_output);
	}

	#[test]
	fn test_encoding_constructor_as_array() {
		use validators::constructor;

		let code = Vec::new();
		let first = [0x11u8; 20];
		let second = [0x22u8; 20];

		let encoded_from_vec = constructor(code.clone(), vec![first, second]);
		let encoded_from_vec_iter = constructor(code.clone(), vec![first, second].into_iter());
		let encoded_from_vec_wrapped = constructor(code, vec![Wrapper(first), Wrapper(second)]);

		let expected = "0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000200000000000000000000000011111111111111111111111111111111111111110000000000000000000000002222222222222222222222222222222222222222".to_owned();
		assert_eq!(expected, hex::encode(&encoded_from_vec));
		assert_eq!(expected, hex::encode(&encoded_from_vec_iter));
		assert_eq!(expected, hex::encode(&encoded_from_vec_wrapped));
	}

	#[test]
	fn test_encoding_function_input_as_fixed_array() {
		use validators::functions;

		let first = [0x11u8; 20];
		let second = [0x22u8; 20];

		let encoded_from_array = functions::add_two_validators::encode_input([first, second]);
		let encoded_from_array_wrapped = functions::add_two_validators::encode_input([Wrapper(first), Wrapper(second)]);
		let encoded_from_string = functions::set_title::encode_input("foo");

		let expected_array = "7de33d2000000000000000000000000011111111111111111111111111111111111111110000000000000000000000002222222222222222222222222222222222222222".to_owned();
		let expected_string = "72910be000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000003666f6f0000000000000000000000000000000000000000000000000000000000".to_owned();
		assert_eq!(expected_array, hex::encode(&encoded_from_array));
		assert_eq!(expected_array, hex::encode(&encoded_from_array_wrapped));
		assert_eq!(expected_string, hex::encode(&encoded_from_string))
	}

	#[test]
	fn encoding_input_works() {
		let expected = "dd62ed3e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000101010101010101010101010101010101010101".to_owned();
		let owner = [0u8; 20];
		let spender = [1u8; 20];
		let encoded = eip20::functions::allowance::encode_input(owner, spender);
		// 4 bytes signature + 2 * 32 bytes for params
		assert_eq!(hex::encode(&encoded), expected);

		let from: Address = [2u8; 20].into();
		let to: Address = [3u8; 20].into();
		let to2: Address = [4u8; 20].into();
		let _filter = eip20::events::transfer::filter(from, vec![to, to2]);
		let wildcard_filter = eip20::events::transfer::filter(None, None);
		let wildcard_filter_sugared = eip20::events::transfer::wildcard_filter();
		assert_eq!(wildcard_filter, wildcard_filter_sugared);
	}
}
