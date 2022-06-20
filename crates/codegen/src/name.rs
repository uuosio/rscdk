pub const INVALID_NAME: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;

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

///
pub fn s2n(s: &str) -> u64 {
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

///verify if name contain invalid character(s)
pub fn is_name_valid(name: &str) -> bool {
	INVALID_NAME != s2n(name)
}

