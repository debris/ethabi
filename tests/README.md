# Ethabi Test Suite

...

## Fuzz Testing

### Running Fuzz Testing

We support fuzzing the decoder with
[cargo-fuzz](https://rust-fuzz.github.io/book/cargo-fuzz.html). The fuzzing
harnesses are located in `./src/fuzz_targets`. To launch the fuzzer you can use
the following command:

```
rustup run nightly cargo fuzz run fixed_abi_decode_random
```

### Incorporating Generated Inputs Into Regular Tests

`cargo fuzz` generates crashing inputs (e.g., on a panic). Add this to the git
repository and update the `fuzztests` crate.

```sh
git add -f ./fuzz/artifacts/fixed_abi_decode_random/crash-651f6fb1e4f699cffb23f8cb616f11590d81f5dd
```

And then add a testcase to `src/fuzztests.rs` based on this template:

```rust
#[test]
fn fuzz_test_1() {
	let (_contract, funcs) = load_abi();
	let input =
		include_bytes!("../fuzz/artifacts/fixed_abi_decode_random/crash-651f6fb1e4f699cffb23f8cb616f11590d81f5dd");

	run_fuzzcase_on_contract_functions(&funcs, &input[0..], true);
}
```
