//! Contract constructor call builder.

use spec::Constructor as ConstructorInterface;
//use function::type_check;
use token::Token;
use errors::{Error, ErrorKind};
use encoder::Encoder;

/// Contract constructor call builder.
#[derive(Clone, Debug, PartialEq)]
pub struct Constructor {
	_interface: ConstructorInterface,
}

impl From<ConstructorInterface> for Constructor {
	fn from(interface: ConstructorInterface) -> Self {
		Constructor {
			_interface: interface,
		}
	}
}

impl Constructor {
	/// Prepares ABI constructor call with given input params.
	pub fn encode_call(&self, tokens: Vec<Token>) -> Result<Vec<u8>, Error> {
		let params = self._interface.param_types();

		if Token::types_check(&tokens, &params) {
			Ok(Encoder::encode(tokens))
		} else {
			Err(ErrorKind::InvalidData.into())
		}
	}
}
