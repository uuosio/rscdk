// This is a Rust implementation of an `name` object.
// The `name` object is a 64-bit integer that is used to represent
// an account name in the EOSIO blockchain.
// This implementation includes functions for converting strings to name objects and vice versa,
// as well as functions for checking the validity of a eosio::name object.

#[cfg(feature = "std")]
use eosio_scale_info::TypeInfo;

use crate::{
    string::String,
};

use crate::serializer::{ 
    Packer,
	Encoder,
};

use crate::vmapi::eosio::{
    check,
    eosio_memcpy,
};

const INVALID_NAME_CHAR: u8 = 0xffu8;

/// a helper function that converts a single ASCII character to
/// a symbol used by the eosio::name object.
/// ".12345abcdefghijklmnopqrstuvwxyz"
pub const fn char_to_index(c: u8) -> u8 {
	match c as char {
		'a'..='z' => {
			return (c - 'a' as u8) + 6;
		}
		'1'..='5' => {
			return  (c - '1' as u8) + 1;
		}
		'.' => {
			return 0;
		}
		_ => {
			return INVALID_NAME_CHAR;
		}
	}
}

const INVALID_NAME: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;


// converts a static string to an `name` object.
pub const fn static_str_to_name(s: &'static str) -> u64 {
	let mut value: u64 = 0;
	let _s = s.as_bytes();

	if _s.len() > 13 {
		return INVALID_NAME;
	}

	if _s.len() == 0 {
		return 0;
	}

	let mut n = _s.len();
	if n == 13 {
		n = 12;
	}

	let mut i = 0usize;

	loop {
		if i >= n {
			break;
		}
		let tmp = char_to_index(_s[i]) as u64;
		if tmp == INVALID_NAME_CHAR as u64 {
			return INVALID_NAME;
		}
		value <<= 5;
		value |= tmp;

		i += 1;
	}
	value <<=  4 + 5*(12 - n);

    if _s.len() == 13 {
		let tmp = char_to_index(_s[12]) as u64;
		if tmp == INVALID_NAME_CHAR as u64 {
			return INVALID_NAME;
		}
		if tmp > 0x0f {
			return INVALID_NAME;
		}
		value |= tmp;
    }

	return value;
}


/// similar to static_str_to_name,
/// but also checks the validity of the resulting `name` object.
pub fn static_str_to_name_checked(s: &'static str) -> u64 {
	let n = static_str_to_name(s);
	check(n != INVALID_NAME, "bad name");
	return n;
}


// a shorthand for static_str_to_name_checked.
pub fn s2n(s: &'static str) -> u64 {
	return static_str_to_name_checked(s);
}

// ".12345abcdefghijklmnopqrstuvwxyz"
pub const CHAR_MAP: [u8; 32] = [46,49,50,51,52,53,97,98,99,100,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122];

/// converts an `name` object to a string.
pub fn n2s(value: u64) -> String {
	// 13 dots
	let mut s: [u8; 13] = [46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46]; //'.'
	let mut tmp = value;
	for i in 0..13 {
		let c: u8;
		if i == 0 {
			c = CHAR_MAP[(tmp&0x0f) as usize];
		} else {
			c = CHAR_MAP[(tmp&0x1f) as usize];
		}
		s[12-i] = c;
		if i == 0 {
			tmp >>= 4
		} else {
			tmp >>= 5
		}
	}

	let mut i = s.len() - 1;
	while i != 0 {
		if s[i] != '.' as u8 {
			break
		}
        i -= 1;
	}
	return String::from_utf8(s[0..i+1].to_vec()).unwrap();
}


///
fn str_to_name(s: &str) -> u64 {
	let mut value: u64 = 0;
	let _s = s.as_bytes();

	if _s.len() > 13 {
		return INVALID_NAME;
	}

	if _s.len() == 0 {
		return 0;
	}

	let mut n = _s.len();
	if n == 13 {
		n = 12;
	}

	let mut i = 0usize;

	loop {
		if i >= n {
			break;
		}
		let tmp = char_to_index(_s[i]) as u64;
		if tmp == 0xff {
			return INVALID_NAME;
		}
		value <<= 5;
		value |= tmp;

		i += 1;
	}
	value <<=  4 + 5*(12 - n);

    if _s.len() == 13 {
		let tmp = char_to_index(_s[12]) as u64;
		if tmp == 0xff {
			return INVALID_NAME;
		}
		if tmp > 0x0f {
			return INVALID_NAME;
		}
		value |= tmp;
    }

	return value;
}

fn str_to_name_checked(s: &str) -> u64 {
	let n = str_to_name(s);
	check(n != INVALID_NAME, "bad name string");
	return n;
}

/// a wrapper around a 64-bit unsigned integer that represents a name in the EOSIO blockchain
#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(TypeInfo))]
pub struct Name {
    ///
    pub n: u64,
}

impl Name {
    ///
    pub fn new(s: &'static str) -> Self {
        Name { n: s2n(s) }
    }

	pub fn value(&self) -> u64 {
		return self.n
	}

    ///
    pub fn from_u64(n: u64) -> Self {
        check(n != INVALID_NAME, "bad name value");
        Name { n: n }
    }

    ///
    pub fn from_str(s: &str) -> Self {
		return Name{ n: str_to_name_checked(s) };
    }

	///
    pub fn to_string(&self) -> String {
        n2s(self.n)
    }
}

impl Packer for Name {
    fn size(&self) -> usize {
        return 8;
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
		self.n.pack(enc)
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        check(raw.len() >= 8, "Name.unpack: buffer overflow!");
        self.n = 0;
        eosio_memcpy(&self.n as *const u64 as *mut u8, raw.as_ptr(), 8);
        return 8;
    }
}

pub const SAME_PAYER: Name = Name{n: 0};
pub const ACTIVE: Name = Name{n: static_str_to_name("active")};
pub const OWNER: Name = Name{n: static_str_to_name("owner")};
pub const CODE: Name = Name{n: static_str_to_name("eosio.code")};


///
#[macro_export]
macro_rules! name {
     ( $head:expr ) => {
        {
            const n: u64 = $crate::name::static_str_to_name($head);
            $crate::name::Name::from_u64(n)
        }
    };
}
