use crate::structs::*;

mod intrinsics {
    use crate::structs::*;
    extern "C" {
        ///
        pub fn prints(cstr: *const u8);
        ///
        pub fn prints_l(cstr: *const u8, len: u32);
        ///
        pub fn printi(value: i64);
        ///
        pub fn printui(value: u64);
        ///
        pub fn printi128(value: *const Int128);
        ///
        pub fn printui128(value: *const Uint128);
        ///
        pub fn printsf(value: f32);
        ///
        pub fn printdf(value: f64);
        ///
        pub fn printqf(value: *const Float128);
        ///
        pub fn printn(name: u64);
        ///
        pub fn printhex(data: *const u8, datalen: u32);
    }
}

///
pub fn prints(cstr: *const u8){
    unsafe {
        return intrinsics::prints(cstr);
    }
}

///
pub fn prints_l(cstr: *const u8, len: u32){
    unsafe {
        return intrinsics::prints_l(cstr, len);
    }
}

///
pub fn printi(value: i64){
    unsafe {
        return intrinsics::printi(value);
    }
}

///
pub fn printui(value: u64){
    unsafe {
        return intrinsics::printui(value);
    }
}

///
pub fn printi128(value: i128){
    unsafe {
        let _value = Int128{lo: (value as u128 & 0xFFFF_FFFF_FFFF_FFFF) as u64, hi: (value as u128 >> 32) as u64};
        return intrinsics::printi128(&_value);
    }
}

///
pub fn printui128(value: u128){
    unsafe {
        let _value = Uint128{lo: (value & 0xFFFF_FFFF_FFFF_FFFF) as u64, hi: (value >> 32) as u64};
        return intrinsics::printui128(&_value);
    }
}

///
pub fn printsf(value: f32){
    unsafe {
        return intrinsics::printsf(value);
    }
}

///
pub fn printdf(value: f64){
    unsafe {
        return intrinsics::printdf(value);
    }
}

///
pub fn printqf(value: *const Float128){
    unsafe {
        return intrinsics::printqf(value);
    }
}

///
pub fn printn(name: u64){
    unsafe {
        return intrinsics::printn(name);
    }
}

///
pub fn printhex(data: *const u8, datalen: u32){
    unsafe {
        return intrinsics::printhex(data, datalen);
    }
}
