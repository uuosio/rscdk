use crate::{
    vec::Vec,
};

use crate::serializer::{
    Packer,
};

use crate::print::{
    Printable,
    printi,
};

#[cfg(feature = "std")]
use eosio_scale_info::TypeInfo;

///
#[derive(Copy, Clone, Eq, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(TypeInfo))]
pub struct VarUint32 {
    ///
    pub n: u32,
}

impl VarUint32 {
    ///
    pub fn new(n: u32) -> Self {
        Self { n: n }
    }
}

impl Packer for VarUint32 {
    ///
    fn size(&self) -> usize {
        let mut size: usize = 0;
        let mut val = self.n;
        while val > 0 {
            val >>= 7;
            size += 1;
        }
        return size;
    }

    ///
    fn pack(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let mut val = self.n;
        while val > 0 {
            let mut b: u32 = val & 0x7f;
            val >>= 7;
            if val > 0 {
                b |= 1 << 7;
            }
            result.push(b as u8);
        }
        return result;
    }

    ///TODO: validate data
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
