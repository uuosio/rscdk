use crate::{
    vec,
    vec::Vec,
};

use core::{
    slice
};

use core::mem::{
    size_of
};

use crate::vmapi::eosio::{
    eosio_memcpy,
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
    fn pack(&self) -> Vec<u8>;
    ///
    fn unpack(&mut self, data: &[u8]) -> usize;
}


macro_rules! impl_packed {
    ( $ty:ident ) => {
        impl Packer for $ty {
            fn size(&self) -> usize {
                size_of::<$ty>()
            }
        
            fn pack(&self) -> Vec<u8> {
                let mut data: Vec<u8> = Vec::with_capacity(size_of::<$ty>());
                data.resize_with(size_of::<$ty>(), Default::default);
                let src = unsafe {
                    slice::from_raw_parts_mut(self as *const $ty as *mut u8, size_of::<$ty>())
                };
                slice_copy(&mut data, src);
                return data;
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

impl_packed!(bool);
impl_packed!(i8);
impl_packed!(u8);

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
    pub fn pack<T>(&mut self, value: &T) -> usize 
    where T: Packer,
    {
        let mut data = value.pack();
        self.buf.append(&mut data);
        return data.len();
    }

    ///
    pub fn pack_number<T>(&mut self, n: T) -> usize {
        let size: usize = size_of::<T>();
        let vec_size = self.buf.len();
        self.buf.resize_with(vec_size + size, Default::default);
        eosio_memcpy(self.buf[vec_size..].as_mut_ptr(), &n as *const T as *const u8, size);
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
            let n = T::default();
            check(self.pos + size <= self.buf.len(), "Decoder::unpack_number: buffer overflow!");
            eosio_memcpy(&n as *const T as *mut T as *mut u8, self.buf[self.pos..].as_ptr(), size);
            self.pos += size;
            return n;
        }

        ///
    pub fn get_pos(&self) -> usize {
        return self.pos;
    }

}

use crate::{
    string::String,
};

impl Packer for String {
    ///
    fn size(&self) -> usize {
        return VarUint32::new(self.len() as u32).size() + self.len();
    }

    ///
    fn pack(&self) -> Vec<u8> {
        let raw = self.as_bytes().to_vec();
        return raw.pack();
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

    ///
    fn pack(&self) -> Vec<u8> {
        let len = VarUint32 { n: self.len() as u32};
        let mut enc = Encoder::new(self.size());
        enc.pack(&len);
        for v in self {
            enc.pack(v);
        }
        return enc.get_bytes();
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
    fn pack(&self) -> Vec<u8> {
        match self {
            Some(x) => {
                let mut enc = Encoder::new(1 + x.size());
                enc.pack_number(1u8);
                enc.pack(x);
                enc.get_bytes()
            }
            None => {
                vec![0]
            }
        }
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