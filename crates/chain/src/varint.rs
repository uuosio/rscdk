use crate::{
    vec,
    vec::Vec,
};

use crate::serializer::{
    Packer,
    Encoder,
};

use crate::print::{
    Printable,
    printi,
};

///
#[cfg_attr(feature = "std", derive(crate::eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct VarUint32 {
    ///
    pub n: u32,
}

impl VarUint32 {
    ///
    pub fn new(n: u32) -> Self {
        Self { n: n }
    }

    pub fn value(&self) -> u32 {
        return self.n;
    }
}

impl Packer for VarUint32 {
    ///
    fn size(&self) -> usize {
        let mut size: usize = 0;
        let mut val = self.n;
        if val == 0 {
            return 1;
        }

        while val > 0 {
            val >>= 7;
            size += 1;
        }
        return size;
    }

    ///
    fn pack(&self, enc: &mut Encoder) -> usize {
        let mut val = self.n;
        if val == 0 {
            return 0u8.pack(enc);
        }

        let mut size = 0usize;

        while val > 0 {
            let mut b: u32 = val & 0x7f;
            val >>= 7;
            if val > 0 {
                b |= 1 << 7;
            }
            let data = enc.alloc(1);
            data[0] = b as u8;
            size += 1;
        }
        size
    }

    fn unpack(&mut self, data: &[u8]) -> usize {
        let mut by: u32 = 0;
        let mut value: u32 = 0;
        let mut length: usize = 0;
        for b in data {
            value |= (*b as u32 & 0x7f) << by;
            by += 7;
            length += 1;
            if (*b & 0x80) == 0 {
                break;
            }
        }
        self.n = value;
        return length;
    }
}

impl Printable for VarUint32 {
    fn print(&self) {
        printi(self.n as i64);
    }
}
