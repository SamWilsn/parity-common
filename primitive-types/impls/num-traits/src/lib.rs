// Copyright 2020 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! num-traits support for uint.

#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
pub use num_traits;

use core::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FromStrRadixErrKind {
	Parse,
	UnsupportedRadix,
}

#[derive(Debug)]
enum FromStrRadixErrSrc {
	Hex(uint::rustc_hex::FromHexError),
	Dec(uint::FromDecStrErr),
}

impl fmt::Display for FromStrRadixErrSrc {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			FromStrRadixErrSrc::Dec(d) => write!(f, "{}", d),
			FromStrRadixErrSrc::Hex(h) => write!(f, "{}", h),
		}
	}
}

#[derive(Debug)]
pub struct FromStrRadixErr {
	kind: FromStrRadixErrKind,
	source: Option<FromStrRadixErrSrc>,
}

impl FromStrRadixErr {
	pub fn unsupported() -> Self {
		Self {
			kind: FromStrRadixErrKind::UnsupportedRadix,
			source: None,
		}
	}

	pub fn kind(&self) -> FromStrRadixErrKind {
		self.kind
	}
}

impl fmt::Display for FromStrRadixErr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.kind {
			FromStrRadixErrKind::UnsupportedRadix => {
				write!(f, "the given radix is not supported")
			}
			FromStrRadixErrKind::Parse => match self.source {
				Some(ref src) => write!(f, "{}", src),
				None => write!(f, "parsing error"),
			}
		}
	}
}

#[cfg(feature = "std")]
impl std::error::Error for FromStrRadixErr {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self.source {
			Some(FromStrRadixErrSrc::Dec(ref d)) => Some(d),
			Some(FromStrRadixErrSrc::Hex(ref h)) => Some(h),
			None => None,
		}
	}
}

impl From<uint::FromDecStrErr> for FromStrRadixErr {
	fn from(e: uint::FromDecStrErr) -> Self {
		Self {
			kind: FromStrRadixErrKind::Parse,
			source: Some(FromStrRadixErrSrc::Dec(e)),
		}
	}
}


impl From<uint::rustc_hex::FromHexError> for FromStrRadixErr {
	fn from(e: uint::rustc_hex::FromHexError) -> Self {
		Self {
			kind: FromStrRadixErrKind::Parse,
			source: Some(FromStrRadixErrSrc::Hex(e)),
		}
	}
}

/// Add num-traits support to an integer created by `construct_uint!`.
#[macro_export]
macro_rules! impl_uint_num_traits {
	($name: ident, $len: expr) => {
		impl $crate::num_traits::identities::Zero for $name {
			#[inline]
			fn zero() -> Self {
				Self::zero()
			}

			#[inline]
			fn is_zero(&self) -> bool {
				self.is_zero()
			}
		}

		impl $crate::num_traits::identities::One for $name {
			#[inline]
			fn one() -> Self {
				Self::one()
			}
		}

		impl $crate::num_traits::Num for $name {
			type FromStrRadixErr = $crate::FromStrRadixErr;

			fn from_str_radix(txt: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
				let parsed = match radix {
					10 => Self::from_dec_str(txt)?,
					16 => core::str::FromStr::from_str(txt)?,
					_ => return Err(Self::FromStrRadixErr::unsupported()),
				};

				Ok(parsed)
			}
		}
	};
}

