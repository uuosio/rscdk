use crate::{
    vec,
    vec::Vec,
};

use core::mem::{
    size_of
};

use crate::vmapi::eosio::{
    eosio_memcpy,
    check,
};

use crate::varint::{
    VarUint32,
};

use crate::{
    eosio_println,
};

use crate::print::{
    Printable,
    prints,
};

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
                let ptr: *const $ty = self;
                let mut data: Vec<u8> = Vec::with_capacity(size_of::<$ty>());
                data.resize_with(size_of::<$ty>(), Default::default);
                eosio_memcpy(data.as_mut_ptr(), ptr as *mut u8, size_of::<$ty>());
                return data;
            }
        
            fn unpack(&mut self, data: &[u8]) -> usize {
                check(data.len() >= self.size(), "number: buffer overflow");
                let ptr: *const $ty = self;
                eosio_memcpy(ptr as *mut u8, data.as_ptr(), size_of::<$ty>());
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
    pub fn pack_bytes(&mut self, bs: &Vec<u8>) -> usize {
        let var = VarUint32::new(bs.len() as u32);
        self.pack(&var);
        self.buf.extend(bs);
        return bs.len() + var.size();
    }

    ///
    pub fn pack_vec<T: Packer>(&mut self, bs: &Vec<T>) -> usize {
        let pos = self.buf.len();
        let var = VarUint32::new(bs.len() as u32);
        self.pack(&var);
        for i in 0..bs.len() {
            self.pack(&bs[i]);
        }
        return self.buf.len() - pos;
    }

    ///
    pub fn pack_u32(&mut self, mut n: u32) -> usize {
        let size: usize = size_of::<u32>();
        for _ in 0..size {
            self.buf.push((n & 0xff) as u8);
            n >>= 8;
        }
        return size;
    }

    ///
    pub fn pack_u64(&mut self, mut n: u64) -> usize {
        let size: usize = size_of::<u64>();
        for _ in 0..size {
            self.buf.push((n & 0xff) as u8);
            n >>= 8;
        }
        return size;
    }

    ///
    pub fn pack_number<T>(&mut self, n: T) -> usize {
        let size: usize = size_of::<T>();
        let _n: Vec<T> = vec![n];
        let vec_size = self.buf.len();
        self.buf.resize_with(vec_size + size, Default::default);
        eosio_memcpy(self.buf[vec_size..].as_mut_ptr(), _n.as_ptr() as *const u8, 10);
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
    pub fn unpack<T>(&mut self, packed: &mut T) -> usize
    where T: Packer,
    {
        let size = packed.unpack(&self.buf[self.pos..]);
        self.pos += size;
        return size;
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
        if let Ok(s) = String::from_utf8(data[size..size+length.n as usize].to_vec()) {
            *self = s;
        } else {
            check(false, "invalid utf8 string");
        }
        return size + length.n as usize;
    }
}

impl<T> Packer for Vec<T> where T: Packer + Default {
    ///
    fn size(&self) -> usize {
        if self.len() == 0 {
            return 0;
        }

        let mut size: usize = 0;
        for i in 0..self.len() {
            size += self[i].size();
        }
        return size;
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
        self.reserve(size.n as usize);
        for _ in 0..size.n {
            let mut v: T = Default::default();
            dec.unpack(&mut v);
            self.push(v);
        }
        return dec.get_pos();
    }
}

// pub enum Option<T> {

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
            return 1;
        }

        check(ty == 1, "bad option type!");

        dec.unpack(&mut value);
        if ty == 0 {
            *self = None;
        } else {
            *self = Some(value);
        }
        dec.get_pos()
    }
}