#[cfg(feature = "std")]
use eosio_scale_info::TypeInfo;

use crate::{
    vec::Vec,
    string::String,
};

use crate::serializer::{ 
    Packer
};

use crate::vmapi::eosio::{
    check,
    eosio_memcpy,
};

///
pub const fn char_to_symbol(c: u8) -> u8 {
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
			return 0xff;
		}
	}
}

const INVALID_NAME: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;

///
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
		let tmp = char_to_symbol(_s[i]) as u64;
		if tmp == 0xff {
			return INVALID_NAME;
		}
		value <<= 5;
		value |= tmp;

		i += 1;
	}
	value <<=  4 + 5*(12 - n);

    if _s.len() == 13 {
		let tmp = char_to_symbol(_s[12]) as u64;
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

///
pub fn static_str_to_name_checked(s: &'static str) -> u64 {
	let n = static_str_to_name(s);
	check(n != INVALID_NAME, "bad name");
	return n;
}


///
pub fn s2n(s: &'static str) -> u64 {
	return static_str_to_name_checked(s);
}

///
pub fn n2s(value: u64) -> String {
	let charmap = ".12345abcdefghijklmnopqrstuvwxyz".as_bytes();
	// 13 dots
	let mut s: [u8; 13] = ['.' as u8, '.'  as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8, '.' as u8];
	let mut tmp = value;
	for i in 0..13 {
		let c: u8;
		if i == 0 {
			c = charmap[(tmp&0x0f) as usize];
		} else {
			c = charmap[(tmp&0x1f) as usize];
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

    let r = match String::from_utf8(s[0..i+1].to_vec()) {
        Ok(v) => v,
        Err(_) => String::from(""),
    };
    return r;
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
		let tmp = char_to_symbol(_s[i]) as u64;
		if tmp == 0xff {
			return INVALID_NAME;
		}
		value <<= 5;
		value |= tmp;

		i += 1;
	}
	value <<=  4 + 5*(12 - n);

    if _s.len() == 13 {
		let tmp = char_to_symbol(_s[12]) as u64;
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

///
#[repr(C)]
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

    fn pack(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        v.resize(8usize, 0);
        eosio_memcpy(v.as_mut_ptr(), &self.n as *const u64 as *mut u8, 8);
        return v;
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        check(raw.len() >= 8, "Name.unpack: buffer overflow!");
        self.n = 0;
        eosio_memcpy(&self.n as *const u64 as *mut u8, raw.as_ptr(), 8);
        return 8;
    }
}

///
#[macro_export]
macro_rules! name {
     ( $head:expr ) => {
        {
            const n: u64 = eosio_chain::name::static_str_to_name($head);
            eosio_chain::name::Name::from_u64(n)
        }
    };
}
