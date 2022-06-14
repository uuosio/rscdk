const MAX_FIXED_STRING_SIZE: usize = 64;

///
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedString {
    ///
    pub buffer: [u8; MAX_FIXED_STRING_SIZE],
    ///
    pub length: usize
}

///
impl FixedString {
    ///
    pub fn new(s: &str) -> Self {
        let mut copy_size = s.len();
        if copy_size > MAX_FIXED_STRING_SIZE {
            copy_size = MAX_FIXED_STRING_SIZE;
        }
        let bytes = s.as_bytes();
        let mut ret = Self {buffer: [0; MAX_FIXED_STRING_SIZE], length: copy_size};
        for i in 0..copy_size {
            ret.buffer[i] = bytes[i];
        }
        return ret;
    }

    pub fn from(v: &[u8]) -> Self {
        let mut buffer: [u8; MAX_FIXED_STRING_SIZE] = [0; MAX_FIXED_STRING_SIZE];
        let mut copy_size = v.len();
        if copy_size > MAX_FIXED_STRING_SIZE {
            copy_size = MAX_FIXED_STRING_SIZE;
        }

        for i in 0..copy_size {
            buffer[i] = v[i];
        }
        Self {buffer: buffer, length: copy_size}
    }

    pub fn str(&self) -> String {
        let mut v: Vec<u8> = vec![0; self.length];
        for i in 0..self.length {
            v[i] = self.buffer[i];
        }
        String::from_utf8(v).unwrap()
    }
}
