use crate::{
    vec,
    vec::Vec,
};

use crate::{
    string::String,
};

use core::{
    slice
};

use core::mem::{
    size_of
};

use crate::vmapi::eosio::{
    slice_copy,
    check,
};

use crate::varint::{
    VarUint32,
};

// use crate::{
//     eosio_println,
// };

// use crate::print::{
//     Printable,
//     prints,
// };

///
pub trait Packer {
    ///
    fn size(&self) -> usize;
    ///
    fn pack(&self, enc: &mut Encoder) -> usize;
    ///
    fn unpack(&mut self, data: &[u8]) -> usize;
}

///
pub struct Encoder {
    buf: Vec<u8>,
}

impl Encoder {
    ///
    pub fn new(size: usize) -> Self {
        Self {
            buf: Vec::with_capacity(size)
        }
    }
    
    ///
    pub fn get_bytes(&self) -> Vec<u8> {
        return self.buf[0..].to_vec();
    }

    ///
    pub fn get_size(&self) -> usize {
        self.buf.len()
    }

    ///
    pub fn alloc(&mut self, size: usize) -> &mut [u8]
    {
        let old_size = self.buf.len();
        self.buf.resize(old_size+size, 0u8);
        &mut self.buf[old_size..]
    }

    ///
    pub fn pack<T: Packer>(value: &T) -> Vec<u8> 
    {
        let mut enc = Self::new(value.size());
        value.pack(&mut enc);
        enc.get_bytes()
    }

    ///
    pub fn pack_number<T>(&mut self, n: T) -> usize {
        let size: usize = size_of::<T>();
        let vec_size = self.buf.len();
        self.buf.resize_with(vec_size + size, Default::default);
        let src = unsafe {
            slice::from_raw_parts(&n as *const T as *const u8, size)
        };
        slice_copy(&mut self.buf[vec_size..vec_size+size], src);
        return size;
    }
}

///
pub struct Decoder<'a> {
    buf: &'a [u8],
    pos: usize
}

impl<'a> Decoder<'a> {
    ///
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            buf: data, pos: 0
        }
    }

    ///
    pub fn unpack<T>(&mut self, packer: &mut T) -> usize
    where T: Packer,
    {
        let size = packer.unpack(&self.buf[self.pos..]);
        self.pos += size;
        return size;
    }

        ///
        pub fn unpack_number<T>(&mut self) -> T 
        where T: Default
        {
            let size: usize = size_of::<T>();
            let mut n = T::default();
            check(self.pos + size <= self.buf.len(), "Decoder::unpack_number: buffer overflow!");
            let dst = unsafe {
                slice::from_raw_parts_mut(&mut n as *mut T as *mut u8, size)
            };
            slice_copy(dst, &self.buf[self.pos..self.pos+size]);
            self.pos += size;
            return n;
        }

        ///
    pub fn get_pos(&self) -> usize {
        return self.pos;
    }

}

macro_rules! impl_packed {
    ( $ty:ident ) => {
        impl Packer for $ty {
            fn size(&self) -> usize {
                size_of::<$ty>()
            }
        
            fn pack(&self, enc: &mut Encoder) -> usize {
                let data = enc.alloc(size_of::<$ty>());
                let src = unsafe {
                    slice::from_raw_parts_mut(self as *const $ty as *mut u8, size_of::<$ty>())
                };
                slice_copy(data, src);
                self.size()
            }
        
            fn unpack(&mut self, data: &[u8]) -> usize {
                check(data.len() >= self.size(), "number: buffer overflow");
                let dst = unsafe {
                    slice::from_raw_parts_mut(self as *const $ty as *mut u8, size_of::<$ty>())
                };
                slice_copy(dst, &data[..self.size()]);
                return size_of::<$ty>();
            }
        }
    };
}

impl Packer for bool {
    fn size(&self) -> usize {
        1usize
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let data = enc.alloc(self.size());
        if *self {
            data[0] = 1u8;
        } else {
            data[0] = 0u8;
        }
        self.size()
    }

    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= self.size(), "number: buffer overflow");
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

impl Packer for i8 {
    fn size(&self) -> usize {
        1usize
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let data = enc.alloc(self.size());
        data[0] = *self as u8;
        self.size()
    }

    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= self.size(), "i8::unpack: buffer overflow");
        *self = data[0] as i8;
        self.size()
    }
}

impl Packer for u8 {
    fn size(&self) -> usize {
        1usize
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let data = enc.alloc(self.size());
        data[0] = *self;
        self.size()
    }

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

impl Packer for String {
    ///
    fn size(&self) -> usize {
        return VarUint32::new(self.len() as u32).size() + self.len();
    }

    ///
    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();

        let size = self.size();

        let raw = self.as_bytes();

        let n = VarUint32::new(raw.len() as u32);
        n.pack(enc);

        let data = enc.alloc(raw.len());
        slice_copy(data, raw);

        enc.get_size() - pos
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        let mut length = VarUint32{n: 0};
        let size = length.unpack(data);
        if let Ok(s) = String::from_utf8(data[size..size+length.value() as usize].to_vec()) {
            *self = s;
        } else {
            check(false, "invalid utf8 string");
        }
        return size + length.value() as usize;
    }
}

impl<T> Packer for Vec<T> where T: Packer + Default {
    ///
    fn size(&self) -> usize {
        if self.len() == 0 {
            return 1;
        }

        let mut size: usize = 0;
        for i in 0..self.len() {
            size += self[i].size();
        }
        return VarUint32::new(size as u32).size() + size;
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();
        let len = VarUint32{n: self.len() as u32};
        len.pack(enc);
        for v in self {
            v.pack(enc);
        }
        enc.get_size() - pos
    }

    ///
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
        return dec.get_pos();
    }
}

impl<T> Packer for Option<T> where T: Packer + Default {
    ///
    fn size(&self) -> usize {
        match self {
            Some(x) => 1 + x.size(),
            None => 1,
        }
    }

    ///
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

    ///
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