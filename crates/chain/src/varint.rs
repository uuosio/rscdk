use crate::serializer::{
    Packer,
    Encoder,
};

use crate::print::{
    Printable,
    printi,
};

use crate::vmapi::eosio::{
    check,
};

/// A variable-length unsigned integer structure.
#[cfg_attr(feature = "std", derive(crate::eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct VarUint32 {
    /// The unsigned integer value.
    pub n: u32,
}

impl VarUint32 {
    /// Create a new VarUint32 instance with the given value.
    pub fn new(n: u32) -> Self {
        Self { n: n }
    }

    /// Get the value of the VarUint32 instance.
    pub fn value(&self) -> u32 {
        return self.n;
    }
}

impl Packer for VarUint32 {
    /// Calculate the size of the serialized VarUint32.
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

    /// Serialize the VarUint32 value.
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

    /// Deserialize the VarUint32 value from the given byte slice.
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
            check(by < 32, "malformed varuint32 data");
        }
        self.n = value;
        return length;
    }
}

impl Printable for VarUint32 {
    /// Print the VarUint32 value.
    fn print(&self) {
        printi(self.n as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varuint32_pack_unpack() {
        let values = vec![
            0,
            1,
            127,
            128,
            255,
            256,
            16383,
            16384,
            2097151,
            2097152,
            268435455,
            268435456,
            u32::MAX,
        ];

        for value in values {
            let varuint32 = VarUint32::new(value);
            let mut encoder = Encoder::new(varuint32.size());
            varuint32.pack(&mut encoder);

            let mut unpacked_varuint32 = VarUint32::default();
            let _ = unpacked_varuint32.unpack(encoder.get_bytes());

            assert_eq!(varuint32, unpacked_varuint32);
        }
    }
}
