use core::{
    mem::size_of,
};

use crate::{
    string::String,
    vec,
    vec::Vec,
    vmapi::eosio::{
        check,
        slice_copy,
    },
    varint::VarUint32,
};

///
/// The `Packer` trait provides methods for packing and unpacking values to and from byte arrays.
///
/// # Examples
///
/// ```
/// use crate::rust_chain::serializer::{Encoder, Decoder, Packer};
///
/// let mut encoder = Encoder::new(4);
/// let value = 123u32;
/// value.pack(&mut encoder);
///
/// let mut decoder = Decoder::new(&encoder.get_bytes());
/// let mut unpacked_value = 0u32;
/// decoder.unpack(&mut unpacked_value);
///
/// assert_eq!(value, unpacked_value);
/// ```
pub trait Packer {
    /// Returns the size of the packed representation of this value in bytes.
    fn size(&self) -> usize;

    /// Packs this value into the given `Encoder`.
    ///
    /// # Arguments
    ///
    /// * `enc` - The encoder to pack this value into.
    ///
    /// # Returns
    ///
    /// The number of bytes written to the encoder.
    fn pack(&self, enc: &mut Encoder) -> usize;

    /// Unpacks this value from the given byte array.
    ///
    /// # Arguments
    ///
    /// * `data` - The byte array to unpack this value from.
    ///
    /// # Returns
    ///
    /// The number of bytes read from the byte array.
    fn unpack(&mut self, data: &[u8]) -> usize;
}

/// The `Encoder` struct provides methods for packing values that implement the `Packer` trait.
///
/// # Examples
///
/// ```
/// use rust_chain::serializer::{Encoder, Packer};
///
/// let mut encoder = Encoder::new(4);
/// let value = 123u32;
///
/// let bytes_written = value.pack(&mut encoder);
/// assert_eq!(bytes_written, 4);
///
/// let packed_bytes = encoder.get_bytes();
/// assert_eq!(packed_bytes, [123, 0, 0, 0]);
/// ```
pub struct Encoder {
    buf: Vec<u8>,
}

impl Encoder {
    /// Constructs a new `Encoder` with the given initial capacity.
    ///
    /// # Arguments
    ///
    /// * `size` - The initial capacity of the encoder in bytes.
    ///
    /// # Returns
    ///
    /// A new `Encoder` instance with the given initial capacity.
    pub fn new(size: usize) -> Self {
        Self {
            buf: Vec::with_capacity(size)
        }
    }
    
    /// Returns the packed bytes of this encoder as a byte array.
    ///
    /// # Returns
    ///
    /// A reference to the packed bytes of this encoder as a byte array.
    pub fn get_bytes(&self) -> &[u8] {
        &self.buf
    }

    /// Returns the number of packed bytes in this encoder.
    ///
    /// # Returns
    ///
    /// The number of packed bytes in this encoder.
    pub fn get_size(&self) -> usize {
        self.buf.len()
    }

    /// Allocates space in this encoder for packing a value of the given size.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of bytes to allocate in this encoder.
    ///
    /// # Returns
    ///
    /// A mutable reference to the allocated
    pub fn alloc(&mut self, size: usize) -> &mut [u8]
    {
        let old_size = self.buf.len();
        self.buf.resize(old_size+size, 0u8);
        &mut self.buf[old_size..]
    }

    /// Packs the given value using the encoder
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be packed
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_chain::serializer::{Encoder, Packer};
    ///
    /// let data = Encoder::pack(&1234u32);
    /// assert_eq!(data, vec![210, 4, 0, 0]);
    /// ```
    pub fn pack<T: Packer>(value: &T) -> Vec<u8> 
    {
        // Create a new Encoder with the size of the value being packed
        let mut enc = Self::new(value.size());
        // Pack the value using the encoder
        value.pack(&mut enc);
        // Return the packed data as a vector of bytes
        enc.get_bytes().to_vec()
    }

}

/// A struct for unpacking packed data
///
/// # Examples
///
/// ```
/// use crate::rust_chain::serializer::{Decoder, Packer};
///
/// let data = &vec![210, 4, 0, 0];
/// let mut decoder = Decoder::new(&data);
/// let mut value = 0u32;
/// decoder.unpack(&mut value);
/// assert_eq!(value, 1234);
/// ```
pub struct Decoder<'a> {
    buf: &'a [u8],
    pos: usize
}

/// A struct for unpacking packed data
impl<'a> Decoder<'a> {

    /// Creates a new `Decoder` instance from the given byte array.
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            buf: data, pos: 0
        }
    }

    /// Unpacks the given value from the decoder
    pub fn unpack<T>(&mut self, packer: &mut T) -> usize
    where T: Packer,
    {
        let size = packer.unpack(&self.buf[self.pos..]);
        self.pos += size;
        size
    }

    /// Returns the current position of the decoder
    pub fn get_pos(&self) -> usize {
        self.pos
    }

}

/// A trait for packing and unpacking values
/// 
macro_rules! impl_packed {
    ( $ty:ident ) => {
        impl Packer for $ty {
            /// Returns the size of this value in bytes.
            fn size(&self) -> usize {
                size_of::<$ty>()
            }

            /// Packs this value into the given encoder.
            fn pack(&self, enc: &mut Encoder) -> usize {
                let data = enc.alloc(size_of::<$ty>());
                let src = self.to_le_bytes();
                slice_copy(data, &src);
                self.size()
            }
        
            /// Unpacks this value from the given data.
            fn unpack(&mut self, data: &[u8]) -> usize {
                check(data.len() >= self.size(), "number: buffer overflow");
                *self = $ty::from_le_bytes(data[..self.size()].try_into().unwrap());
                size_of::<$ty>()
            }
        }
    };
}

/// Implement`Packer` for bool type.
impl Packer for bool {
    fn size(&self) -> usize {
        1usize
    }

    /// Packs this value into the given encoder.
    fn pack(&self, enc: &mut Encoder) -> usize {
        let data = enc.alloc(self.size());
        if *self {
            data[0] = 1u8;
        } else {
            data[0] = 0u8;
        }
        self.size()
    }

    /// Unpacks this value from the given data.
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= self.size(), "bool::unpack: buffer overflow");
        if data[0] == 1 {
            *self = true;
        } else if data[0] == 0 {
            *self = false;
        } else {
            check(false, "bool::unpack: invalid raw bool value");
        }
        self.size()
    }
}

/// Implement `Packer` for i8 type.
impl Packer for i8 {

    /// Returns the size of this value in bytes.
    fn size(&self) -> usize {
        1usize
    }

    /// Packs this value into the given encoder.
    fn pack(&self, enc: &mut Encoder) -> usize {
        let data = enc.alloc(self.size());
        data[0] = *self as u8;
        self.size()
    }

    /// Unpacks this value from the given data.
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= self.size(), "i8::unpack: buffer overflow");
        *self = data[0] as i8;
        self.size()
    }
}

/// Implement `Packer` for u8 type.
impl Packer for u8 {

    /// Returns the size of this value in bytes.
    fn size(&self) -> usize {
        1usize
    }

    /// Packs this value into the given encoder.
    fn pack(&self, enc: &mut Encoder) -> usize {
        let data = enc.alloc(self.size());
        data[0] = *self;
        self.size()
    }

    /// Unpacks this value from the given data.
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= self.size(), "u8::unpack: buffer overflow");
        *self = data[0];
        self.size()
    }
}

impl_packed!(i16);
impl_packed!(u16);
impl_packed!(i32);
impl_packed!(u32);
impl_packed!(i64);
impl_packed!(u64);
impl_packed!(i128);
impl_packed!(u128);
impl_packed!(f32);
impl_packed!(f64);

/// Implement `Packer` for `String` type.
impl Packer for String {

    /// Returns the size of this value in bytes.
    fn size(&self) -> usize {
        VarUint32::new(self.len() as u32).size() + self.len()
    }

    /// Packs this value into the given encoder.
    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();

        let raw = self.as_bytes();

        let n = VarUint32::new(raw.len() as u32);
        n.pack(enc);

        let data = enc.alloc(raw.len());
        slice_copy(data, raw);

        enc.get_size() - pos
    }

    /// Unpacks this value from the given data.
    fn unpack(&mut self, data: &[u8]) -> usize {
        let mut length = VarUint32{n: 0};
        let size = length.unpack(data);
        if let Ok(s) = String::from_utf8(data[size..size+length.value() as usize].to_vec()) {
            *self = s;
        } else {
            check(false, "invalid utf8 string");
        }
        size + length.value() as usize
    }
}

/// Implement `Packer` for `Vec<T>` type.
impl<T> Packer for Vec<T> where T: Packer + Default {
    /// Returns the size of this value in bytes.
    fn size(&self) -> usize {
        if self.len() == 0 {
            return 1;
        }

        let mut size: usize = 0;
        for i in 0..self.len() {
            size += self[i].size();
        }
        VarUint32::new(size as u32).size() + size
    }

    /// Packs this value into the given encoder.
    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();
        let len = VarUint32{n: self.len() as u32};
        len.pack(enc);
        for v in self {
            v.pack(enc);
        }
        enc.get_size() - pos
    }

    /// Unpacks this value from the given data.
    fn unpack(&mut self, data: &[u8]) -> usize {
        let mut dec = Decoder::new(data);
        let mut size = VarUint32{n: 0};
        dec.unpack(&mut size);
        self.reserve(size.value() as usize);
        for _ in 0..size.value() {
            let mut v: T = Default::default();
            dec.unpack(&mut v);
            self.push(v);
        }
        dec.get_pos()
    }
}

/// Implement `Packer` for `Option<T>` type.
impl<T> Packer for Option<T> where T: Packer + Default {

    /// Returns the size of this value in bytes.
    fn size(&self) -> usize {
        match self {
            Some(x) => 1 + x.size(),
            None => 1,
        }
    }

    /// Packs this value into the given encoder.
    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();
        match self {
            Some(x) => {
                1u8.pack(enc);
                x.pack(enc);
            }
            None => {
                0u8.pack(enc);
            }
        }
        enc.get_size() - pos
    }

    /// Unpacks this value from the given data.
    fn unpack(&mut self, data: &[u8]) -> usize {
        let mut dec = Decoder::new(data);
        let mut ty: u8 = 0;
        let mut value: T = Default::default();
        dec.unpack(&mut ty);
        if ty == 0 {
            *self = None;
            return 1;
        }

        check(ty == 1, "bad option type!");

        dec.unpack(&mut value);
        *self = Some(value);
        dec.get_pos()
    }
}